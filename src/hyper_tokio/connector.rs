use crate::errors::*;
use crate::hyper_tokio::ratectl::SlackTokioRateController;
use crate::hyper_tokio::Body;
use crate::models::{SlackClientId, SlackClientSecret};
use crate::*;
use async_recursion::async_recursion;
use futures::future::{BoxFuture, FutureExt};
use http_body_util::{BodyExt, Empty, Full};
use hyper::http::StatusCode;
use hyper::Request;
use hyper_rustls::HttpsConnector;
use hyper_util::client::legacy::*;
use hyper_util::rt::TokioExecutor;
use rvstruct::ValueStruct;

use crate::hyper_tokio::multipart_form::{
    create_multipart_file_content, generate_multipart_boundary,
};
use crate::multipart_form::FileMultipartData;
use crate::prelude::hyper_ext::HyperExtensions;
use crate::ratectl::SlackApiRateControlConfig;
use bytes::BytesMut;
use std::hash::Hash;
use std::hash::Hasher;
use std::sync::Arc;
use std::time::Duration;

use tracing::*;
use url::Url;

#[derive(Clone, Debug)]
pub struct SlackClientHyperConnector<H: Send + Sync + Clone + connect::Connect> {
    hyper_connector: Client<H, Body>,
    tokio_rate_controller: Option<Arc<SlackTokioRateController>>,
    slack_api_url: String,
}

pub type SlackClientHyperHttpsConnector =
    SlackClientHyperConnector<HttpsConnector<connect::HttpConnector>>;

impl SlackClientHyperConnector<HttpsConnector<connect::HttpConnector>> {
    pub fn new() -> std::io::Result<Self> {
        let https_connector = hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()?
            .https_only()
            .enable_http2()
            .build();
        Ok(Self::with_connector(https_connector))
    }
}

impl From<HttpsConnector<connect::HttpConnector>>
    for SlackClientHyperConnector<HttpsConnector<connect::HttpConnector>>
{
    fn from(https_connector: HttpsConnector<connect::HttpConnector>) -> Self {
        Self::with_connector(https_connector)
    }
}

impl<H: 'static + Send + Sync + Clone + connect::Connect> SlackClientHyperConnector<H> {
    pub fn with_connector(connector: H) -> Self {
        Self {
            hyper_connector: Client::builder(TokioExecutor::new()).build::<_, Body>(connector),
            tokio_rate_controller: None,
            slack_api_url: SlackClientHttpApiUri::SLACK_API_URI_STR.to_string(),
        }
    }

    pub fn with_rate_control(self, rate_control_config: SlackApiRateControlConfig) -> Self {
        Self {
            tokio_rate_controller: Some(Arc::new(SlackTokioRateController::new(
                rate_control_config,
            ))),
            ..self
        }
    }

    pub fn with_slack_api_url(self, slack_api_url: &str) -> Self {
        Self {
            slack_api_url: slack_api_url.to_string(),
            ..self
        }
    }

    async fn send_http_request<'a, RS>(
        &'a self,
        request: Request<Body>,
        context: SlackClientApiCallContext<'a>,
    ) -> ClientResult<RS>
    where
        RS: for<'de> serde::de::Deserialize<'de>,
    {
        let uri_str = if context.is_sensitive_url {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            request.uri().path().to_string().hash(&mut hasher);
            let u64 = hasher.finish();

            format!(
                "{}://{}/-redacted-/{}",
                request
                    .uri()
                    .scheme()
                    .map(|scheme| scheme.to_string())
                    .unwrap_or_else(|| "unknown-scheme".to_string()),
                request
                    .uri()
                    .host()
                    .map(|host| host.to_string())
                    .unwrap_or_else(|| "unknown-host".to_string()),
                u64
            )
        } else {
            request.uri().to_string()
        };

        context.tracing_span.in_scope(|| {
            debug!(
                slack_uri = uri_str.as_str(),
                "Sending HTTP request to {}",
                uri_str.as_str()
            );
        });

        let http_res = self.hyper_connector.request(request).await?;
        let http_status = http_res.status();
        let http_headers = http_res.headers().clone();
        let http_content_type = HyperExtensions::http_response_content_type(&http_res);
        let http_body_str = HyperExtensions::http_body_to_string(http_res).await?;
        let http_content_is_json = http_content_type.iter().all(|response_mime| {
            response_mime.type_() == mime::APPLICATION && response_mime.subtype() == mime::JSON
        });

        context.tracing_span.in_scope(|| {
            debug!(
                slack_uri = uri_str.as_str(),
                slack_http_status = http_status.as_u16(),
                "Received HTTP response {}",
                http_status
            );
        });

        match http_status {
            StatusCode::OK if http_content_is_json => {
                let slack_message: SlackEnvelopeMessage =
                    serde_json::from_str(http_body_str.as_str())
                        .map_err(|err| map_serde_error(err, Some(http_body_str.as_str())))?;
                match slack_message.error {
                    None => {
                        let decoded_body = serde_json::from_str(http_body_str.as_str())
                            .map_err(|err| map_serde_error(err, Some(http_body_str.as_str())))?;
                        Ok(decoded_body)
                    }
                    Some(slack_error) => Err(SlackClientError::ApiError(
                        SlackClientApiError::new(slack_error)
                            .opt_errors(slack_message.errors)
                            .opt_warnings(slack_message.warnings)
                            .with_http_response_body(http_body_str),
                    )),
                }
            }
            StatusCode::OK | StatusCode::NO_CONTENT => {
                serde_json::from_str("{}").map_err(|err| map_serde_error(err, Some("{}")))
            }
            StatusCode::TOO_MANY_REQUESTS if http_content_is_json => {
                let slack_message: SlackEnvelopeMessage =
                    serde_json::from_str(http_body_str.as_str())
                        .map_err(|err| map_serde_error(err, Some(http_body_str.as_str())))?;

                Err(SlackClientError::RateLimitError(
                    SlackRateLimitError::new()
                        .opt_retry_after(
                            http_headers
                                .get(hyper::header::RETRY_AFTER)
                                .and_then(|ra| ra.to_str().ok().and_then(|s| s.parse().ok()))
                                .map(Duration::from_secs),
                        )
                        .opt_code(slack_message.error)
                        .opt_warnings(slack_message.warnings)
                        .with_http_response_body(http_body_str),
                ))
            }
            StatusCode::TOO_MANY_REQUESTS => Err(SlackClientError::RateLimitError(
                SlackRateLimitError::new()
                    .opt_retry_after(
                        http_headers
                            .get(hyper::header::RETRY_AFTER)
                            .and_then(|ra| ra.to_str().ok().and_then(|s| s.parse().ok()))
                            .map(Duration::from_secs),
                    )
                    .with_http_response_body(http_body_str),
            )),
            _ => Err(SlackClientError::HttpError(
                SlackClientHttpError::new(http_status).with_http_response_body(http_body_str),
            )),
        }
    }

    #[async_recursion]
    async fn send_rate_controlled_request<'a, R, RS>(
        &'a self,
        request: R,
        context: SlackClientApiCallContext<'a>,
        delayed: Option<Duration>,
        retried: usize,
    ) -> ClientResult<RS>
    where
        R: Fn() -> ClientResult<Request<Body>> + Send + Sync,
        RS: for<'de> serde::de::Deserialize<'de> + Send,
    {
        match (
            self.tokio_rate_controller.as_ref(),
            context.rate_control_params,
        ) {
            (Some(rate_controller), maybe_method_rate_params) => {
                rate_controller
                    .throttle_delay(
                        maybe_method_rate_params,
                        context.token.and_then(|t| t.team_id.clone()),
                        delayed,
                    )
                    .await;

                self.retry_request_if_needed(
                    rate_controller.clone(),
                    self.send_http_request(request()?, context.clone()).await,
                    retried,
                    request,
                    context,
                )
                .await
            }
            (None, _) => self.send_http_request(request()?, context).await,
        }
    }

    async fn retry_request_if_needed<'a, R, RS>(
        &'a self,
        rate_controller: Arc<SlackTokioRateController>,
        result: ClientResult<RS>,
        retried: usize,
        request: R,
        context: SlackClientApiCallContext<'a>,
    ) -> ClientResult<RS>
    where
        R: Fn() -> ClientResult<Request<Body>> + Send + Sync,
        RS: for<'de> serde::de::Deserialize<'de> + Send,
    {
        match result {
            Err(err) => match rate_controller.config.max_retries {
                Some(max_retries) if max_retries > retried => match err {
                    SlackClientError::RateLimitError(ref rate_error) => {
                        context.tracing_span.in_scope(|| {
                            debug!(
                                "Rate limit error received: {}. Retrying: {}/{}",
                                rate_error,
                                retried + 1,
                                max_retries
                            );
                        });

                        self.send_rate_controlled_request(
                            request,
                            context,
                            rate_error.retry_after,
                            retried + 1,
                        )
                        .await
                    }
                    _ => Err(err),
                },
                _ => Err(err),
            },
            Ok(result) => Ok(result),
        }
    }
}

impl<H: 'static + Send + Sync + Clone + connect::Connect> SlackClientHttpConnector
    for SlackClientHyperConnector<H>
{
    fn create_method_uri_path(&self, method_relative_uri: &str) -> ClientResult<Url> {
        Ok(format!("{}/{}", self.slack_api_url, method_relative_uri).parse()?)
    }

    fn http_get_uri<'a, RS>(
        &'a self,
        full_uri: Url,
        context: SlackClientApiCallContext<'a>,
    ) -> BoxFuture<'a, ClientResult<RS>>
    where
        RS: for<'de> serde::de::Deserialize<'de> + Send + 'a + Send,
    {
        let context_token = context.token;
        async move {
            let body = self
                .send_rate_controlled_request(
                    || {
                        let base_http_request = HyperExtensions::create_http_request(
                            full_uri.clone(),
                            hyper::http::Method::GET,
                        );

                        let http_request = HyperExtensions::setup_token_auth_header(
                            base_http_request,
                            context_token,
                        );

                        http_request
                            .body(Empty::new().boxed())
                            .map_err(|e| e.into())
                    },
                    context,
                    None,
                    0,
                )
                .await?;

            Ok(body)
        }
        .boxed()
    }

    fn http_get_with_client_secret<'a, RS>(
        &'a self,
        full_uri: Url,
        client_id: &'a SlackClientId,
        client_secret: &'a SlackClientSecret,
    ) -> BoxFuture<'a, ClientResult<RS>>
    where
        RS: for<'de> serde::de::Deserialize<'de> + Send + 'a + 'a + Send,
    {
        async move {
            let http_oauth_span = span!(Level::DEBUG, "Slack OAuth Get");

            let context = crate::SlackClientApiCallContext {
                rate_control_params: None,
                token: None,
                tracing_span: &http_oauth_span,
                is_sensitive_url: false,
            };

            self.send_rate_controlled_request(
                || {
                    HyperExtensions::setup_basic_auth_header(
                        HyperExtensions::create_http_request(
                            full_uri.clone(),
                            hyper::http::Method::GET,
                        ),
                        client_id.value(),
                        client_secret.value(),
                    )
                    .body(Empty::new().boxed())
                    .map_err(|e| e.into())
                },
                context,
                None,
                0,
            )
            .await
        }
        .boxed()
    }

    fn http_post_uri<'a, RQ, RS>(
        &'a self,
        full_uri: Url,
        request_body: &'a RQ,
        context: SlackClientApiCallContext<'a>,
    ) -> BoxFuture<'a, ClientResult<RS>>
    where
        RQ: serde::ser::Serialize + Send + Sync,
        RS: for<'de> serde::de::Deserialize<'de> + Send + 'a + Send + 'a,
    {
        let context_token = context.token;

        async move {
            let post_json =
                serde_json::to_string(&request_body).map_err(|err| map_serde_error(err, None))?;

            let response_body = self
                .send_rate_controlled_request(
                    || {
                        let base_http_request = HyperExtensions::create_http_request(
                            full_uri.clone(),
                            hyper::http::Method::POST,
                        )
                        .header("content-type", "application/json; charset=utf-8");

                        let http_request = HyperExtensions::setup_token_auth_header(
                            base_http_request,
                            context_token,
                        );

                        http_request
                            .body(Full::new(post_json.clone().into()).boxed())
                            .map_err(|e| e.into())
                    },
                    context,
                    None,
                    0,
                )
                .await?;

            Ok(response_body)
        }
        .boxed()
    }

    fn http_post_uri_multipart_form<'a, 'p, RS, PT, TS>(
        &'a self,
        full_uri: Url,
        file: Option<FileMultipartData<'p>>,
        params: &'p PT,
        context: SlackClientApiCallContext<'a>,
    ) -> BoxFuture<'a, ClientResult<RS>>
    where
        RS: for<'de> serde::de::Deserialize<'de> + Send + 'a + Send + 'a,
        PT: std::iter::IntoIterator<Item = (&'p str, Option<TS>)> + Clone,
        TS: AsRef<str> + 'p + Send,
    {
        let context_token = context.token;
        let boundary = generate_multipart_boundary();
        match create_multipart_file_content(params, boundary.as_str(), file) {
            Ok(file_bytes) => self
                .send_rate_controlled_request(
                    move || {
                        let http_body = Full::new(file_bytes.clone()).boxed();

                        let http_base_request = HyperExtensions::create_http_request(
                            full_uri.clone(),
                            hyper::http::Method::POST,
                        )
                        .header(
                            "content-type",
                            format!("multipart/form-data; boundary={}", boundary),
                        );

                        let http_request = HyperExtensions::setup_token_auth_header(
                            http_base_request,
                            context_token,
                        );

                        http_request.body(http_body).map_err(|e| e.into())
                    },
                    context,
                    None,
                    0,
                )
                .boxed(),
            Err(err) => futures::future::err(err.into()).boxed(),
        }
    }

    fn http_post_uri_binary<'a, 'p, RS>(
        &'a self,
        full_uri: Url,
        content_type: String,
        data: &'a [u8],
        context: SlackClientApiCallContext<'a>,
    ) -> BoxFuture<'a, ClientResult<RS>>
    where
        RS: for<'de> serde::de::Deserialize<'de> + Send + 'a + Send + 'a,
    {
        let context_token = context.token;
        let body_bytes = BytesMut::from(data).freeze();

        async move {
            let response_body = self
                .send_rate_controlled_request(
                    move || {
                        let http_body = Full::new(body_bytes.clone()).boxed();
                        let http_base_request = HyperExtensions::create_http_request(
                            full_uri.clone(),
                            hyper::http::Method::POST,
                        )
                        .header("content-type", content_type.as_str());

                        let http_request = HyperExtensions::setup_token_auth_header(
                            http_base_request,
                            context_token,
                        );

                        http_request.body(http_body).map_err(|e| e.into())
                    },
                    context,
                    None,
                    0,
                )
                .await?;

            Ok(response_body)
        }
        .boxed()
    }
}
