use crate::ratectl::SlackTokioRateController;
use async_recursion::async_recursion;
use bytes::Buf;
use futures::future::TryFutureExt;
use futures::future::{BoxFuture, FutureExt};
use hyper::body::HttpBody;
use hyper::client::*;
use hyper::http::StatusCode;
use hyper::{Body, Request, Response, Uri};
use hyper_rustls::HttpsConnector;
use log::*;
use mime::Mime;
use rvstruct::ValueStruct;
use slack_morphism::errors::*;
use slack_morphism::prelude::{SlackApiMethodRateControlConfig, SlackApiRateControlConfig};
use slack_morphism::signature_verifier::SlackEventAbsentSignatureError;
use slack_morphism::signature_verifier::SlackEventSignatureVerifier;
use slack_morphism::*;
use slack_morphism_models::{SlackClientId, SlackClientSecret};
use std::collections::HashMap;
use std::io::Read;
use std::sync::Arc;
use std::time::Duration;
use url::Url;

#[derive(Clone, Debug)]
pub struct SlackClientHyperConnector<H: Send + Sync + Clone + connect::Connect> {
    hyper_connector: Client<H>,
    tokio_rate_controller: Option<Arc<SlackTokioRateController>>,
}

pub type SlackClientHyperHttpsConnector = SlackClientHyperConnector<HttpsConnector<HttpConnector>>;

impl SlackClientHyperConnector<HttpsConnector<HttpConnector>> {
    pub fn new() -> Self {
        let https_connector = hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_only()
            .enable_http2()
            .build();
        Self::with_connector(https_connector)
    }
}

impl From<HttpsConnector<HttpConnector>>
    for SlackClientHyperConnector<HttpsConnector<HttpConnector>>
{
    fn from(https_connector: hyper_rustls::HttpsConnector<HttpConnector>) -> Self {
        Self::with_connector(https_connector)
    }
}

impl<H: 'static + Send + Sync + Clone + connect::Connect> SlackClientHyperConnector<H> {
    pub fn with_connector(connector: H) -> Self {
        Self {
            hyper_connector: Client::builder().build::<_, hyper::Body>(connector),
            tokio_rate_controller: None,
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

    pub(crate) fn parse_query_params(request: &Request<Body>) -> HashMap<String, String> {
        request
            .uri()
            .query()
            .map(|v| {
                url::form_urlencoded::parse(v.as_bytes())
                    .into_owned()
                    .collect()
            })
            .unwrap_or_else(HashMap::new)
    }

    pub(crate) fn hyper_redirect_to(
        url: &str,
    ) -> Result<Response<Body>, Box<dyn std::error::Error + Send + Sync>> {
        Response::builder()
            .status(hyper::http::StatusCode::FOUND)
            .header(hyper::header::LOCATION, url)
            .body(Body::empty())
            .map_err(|e| e.into())
    }

    fn setup_token_auth_header(
        request_builder: hyper::http::request::Builder,
        token: Option<&SlackApiToken>,
    ) -> hyper::http::request::Builder {
        if token.is_none() {
            request_builder
        } else {
            let token_header_value = format!("Bearer {}", token.unwrap().token_value.value());
            request_builder.header(hyper::header::AUTHORIZATION, token_header_value)
        }
    }

    pub(crate) fn setup_basic_auth_header(
        request_builder: hyper::http::request::Builder,
        username: &str,
        password: &str,
    ) -> hyper::http::request::Builder {
        let header_value = format!(
            "Basic {}",
            base64::encode(format!("{}:{}", username, password))
        );
        request_builder.header(hyper::header::AUTHORIZATION, header_value)
    }

    pub(crate) fn create_http_request(
        url: Url,
        method: hyper::http::Method,
    ) -> hyper::http::request::Builder {
        let uri: Uri = url.as_str().parse().unwrap();
        hyper::http::request::Builder::new()
            .method(method)
            .uri(uri)
            .header("accept-charset", "utf-8")
    }

    async fn http_body_to_string<T>(body: T) -> AnyStdResult<String>
    where
        T: HttpBody,
        T::Error: std::error::Error + Sync + Send + 'static,
    {
        let http_body = hyper::body::aggregate(body).await?;
        let mut http_reader = http_body.reader();
        let mut http_body_str = String::new();
        http_reader.read_to_string(&mut http_body_str)?;
        Ok(http_body_str)
    }

    fn http_response_content_type<RS>(response: &Response<RS>) -> Option<Mime> {
        let http_headers = response.headers();
        http_headers.get(hyper::header::CONTENT_TYPE).map(|hv| {
            let hvs = hv.to_str().unwrap();
            hvs.parse::<Mime>().unwrap()
        })
    }

    async fn send_http_request<RS>(&self, request: Request<Body>) -> ClientResult<RS>
    where
        RS: for<'de> serde::de::Deserialize<'de>,
    {
        let http_res = self
            .hyper_connector
            .request(request)
            .await
            .map_err(Self::map_http_error)?;
        let http_status = http_res.status();
        let http_headers = http_res.headers().clone();
        let http_content_type = Self::http_response_content_type(&http_res);
        let http_body_str = Self::http_body_to_string(http_res)
            .map_err(Self::map_system_error)
            .await?;
        let http_content_is_json = http_content_type.iter().all(|response_mime| {
            response_mime.type_() == mime::APPLICATION && response_mime.subtype() == mime::JSON
        });

        match http_status {
            StatusCode::OK if http_content_is_json => {
                let slack_message: SlackEnvelopeMessage =
                    serde_json::from_str(http_body_str.as_str())
                        .map_err(|err| Self::map_serde_error(err, Some(http_body_str.as_str())))?;
                match slack_message.error {
                    None => {
                        let decoded_body =
                            serde_json::from_str(http_body_str.as_str()).map_err(|err| {
                                Self::map_serde_error(err, Some(http_body_str.as_str()))
                            })?;
                        Ok(decoded_body)
                    }
                    Some(slack_error) => Err(SlackClientError::ApiError(
                        SlackClientApiError::new(slack_error)
                            .opt_warnings(slack_message.warnings)
                            .with_http_response_body(http_body_str),
                    )),
                }
            }
            StatusCode::OK | StatusCode::NO_CONTENT => {
                serde_json::from_str("{}").map_err(|err| Self::map_serde_error(err, Some("{}")))
            }
            StatusCode::TOO_MANY_REQUESTS if http_content_is_json => {
                let slack_message: SlackEnvelopeMessage =
                    serde_json::from_str(http_body_str.as_str())
                        .map_err(|err| Self::map_serde_error(err, Some(http_body_str.as_str())))?;

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
        token: Option<&'a SlackApiToken>,
        rate_control_params: Option<&'a SlackApiMethodRateControlConfig>,
        delayed: Option<Duration>,
        retried: usize,
    ) -> ClientResult<RS>
    where
        R: Fn() -> ClientResult<Request<Body>> + Send + Sync,
        RS: for<'de> serde::de::Deserialize<'de> + Send,
    {
        match (self.tokio_rate_controller.as_ref(), rate_control_params) {
            (Some(rate_controller), maybe_method_rate_params) => {
                if let Some(duration) = rate_controller
                    .calc_throttle_delay(
                        maybe_method_rate_params,
                        token.and_then(|t| t.team_id.clone()),
                        delayed,
                    )
                    .await
                {
                    if !duration.is_zero() {
                        debug!("Slack throttler postponed request for {:?}", duration);
                        let mut interval = tokio::time::interval(duration);

                        interval.tick().await;
                        interval.tick().await;
                    }
                }

                self.retry_request_if_needed(
                    rate_controller.clone(),
                    self.send_http_request(request()?).await,
                    retried,
                    request,
                    token,
                    rate_control_params,
                )
                .await
            }
            (None, _) => self.send_http_request(request()?).await,
        }
    }

    async fn retry_request_if_needed<R, RS>(
        &self,
        rate_controller: Arc<SlackTokioRateController>,
        result: ClientResult<RS>,
        retried: usize,
        request: R,
        token: Option<&SlackApiToken>,
        rate_control_params: Option<&SlackApiMethodRateControlConfig>,
    ) -> ClientResult<RS>
    where
        R: Fn() -> ClientResult<Request<Body>> + Send + Sync,
        RS: for<'de> serde::de::Deserialize<'de> + Send,
    {
        match result {
            Err(err) => match rate_controller.config.max_retries {
                Some(max_retries) if max_retries > retried => match err {
                    SlackClientError::RateLimitError(ref rate_error) => {
                        debug!(
                            "Rate limit error received: {}. Retrying: {}/{}",
                            rate_error,
                            retried + 1,
                            max_retries
                        );

                        self.send_rate_controlled_request(
                            request,
                            token,
                            rate_control_params,
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

    pub(crate) async fn decode_signed_response(
        req: Request<Body>,
        signature_verifier: &SlackEventSignatureVerifier,
    ) -> AnyStdResult<String> {
        let headers = &req.headers().clone();
        let req_body = req.into_body();
        match (
            headers.get(SlackEventSignatureVerifier::SLACK_SIGNED_HASH_HEADER),
            headers.get(SlackEventSignatureVerifier::SLACK_SIGNED_TIMESTAMP),
        ) {
            (Some(received_hash), Some(received_ts)) => {
                Self::http_body_to_string(req_body)
                    .and_then(|body| async {
                        signature_verifier
                            .verify(
                                received_hash.to_str().unwrap(),
                                &body,
                                received_ts.to_str().unwrap(),
                            )
                            .map(|_| body)
                            .map_err(|e| e.into())
                    })
                    .await
            }
            _ => Err(Box::new(SlackEventAbsentSignatureError::new())),
        }
    }

    pub(crate) fn map_http_error(hyper_err: hyper::Error) -> SlackClientError {
        SlackClientError::HttpProtocolError(
            SlackClientHttpProtocolError::new().with_cause(Box::new(hyper_err)),
        )
    }

    pub(crate) fn map_hyper_http_error(hyper_err: hyper::http::Error) -> SlackClientError {
        SlackClientError::HttpProtocolError(
            SlackClientHttpProtocolError::new().with_cause(Box::new(hyper_err)),
        )
    }

    pub(crate) fn map_serde_error(
        err: serde_json::Error,
        tried_to_parse: Option<&str>,
    ) -> SlackClientError {
        SlackClientError::ProtocolError(
            SlackClientProtocolError::new(err).opt_json_body(tried_to_parse.map(|s| s.to_string())),
        )
    }

    pub(crate) fn map_system_error(
        err: Box<dyn std::error::Error + Sync + Send>,
    ) -> SlackClientError {
        SlackClientError::SystemError(SlackClientSystemError::new().with_cause(err))
    }
}

impl<H: 'static + Send + Sync + Clone + connect::Connect> SlackClientHttpConnector
    for SlackClientHyperConnector<H>
{
    fn http_get_uri<'a, RS>(
        &'a self,
        full_uri: Url,
        token: Option<&'a SlackApiToken>,
        rate_control_params: Option<&'a SlackApiMethodRateControlConfig>,
    ) -> BoxFuture<'a, ClientResult<RS>>
    where
        RS: for<'de> serde::de::Deserialize<'de> + Send + 'a + Send,
    {
        async move {
            let body = self
                .send_rate_controlled_request(
                    || {
                        let base_http_request =
                            Self::create_http_request(full_uri.clone(), hyper::http::Method::GET);

                        let http_request = Self::setup_token_auth_header(base_http_request, token);

                        http_request
                            .body(Body::empty())
                            .map_err(Self::map_hyper_http_error)
                    },
                    token,
                    rate_control_params,
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
            self.send_rate_controlled_request(
                || {
                    Self::setup_basic_auth_header(
                        Self::create_http_request(full_uri.clone(), hyper::http::Method::GET),
                        client_id.value(),
                        client_secret.value(),
                    )
                    .body(Body::empty())
                    .map_err(Self::map_hyper_http_error)
                },
                None,
                None,
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
        token: Option<&'a SlackApiToken>,
        rate_control_params: Option<&'a SlackApiMethodRateControlConfig>,
    ) -> BoxFuture<'a, ClientResult<RS>>
    where
        RQ: serde::ser::Serialize + Send + Sync,
        RS: for<'de> serde::de::Deserialize<'de> + Send + 'a + Send + 'a,
    {
        async move {
            let post_json = serde_json::to_string(&request_body)
                .map_err(|err| Self::map_serde_error(err, None))?;

            let response_body = self
                .send_rate_controlled_request(
                    || {
                        let base_http_request =
                            Self::create_http_request(full_uri.clone(), hyper::http::Method::POST)
                                .header("content-type", "application/json; charset=utf-8");

                        let http_request = Self::setup_token_auth_header(base_http_request, token);

                        http_request
                            .body(post_json.clone().into())
                            .map_err(Self::map_hyper_http_error)
                    },
                    token,
                    rate_control_params,
                    None,
                    0,
                )
                .await?;

            Ok(response_body)
        }
        .boxed()
    }
}
