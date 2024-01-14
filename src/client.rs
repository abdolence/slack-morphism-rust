use serde::{Deserialize, Serialize};
use std::future::Future;
use std::sync::Arc;

use crate::token::*;

use crate::errors::SlackClientError;
use crate::models::*;
use crate::ratectl::SlackApiMethodRateControlConfig;
use futures_util::future::BoxFuture;
use lazy_static::*;
use rvstruct::ValueStruct;
use tracing::*;
use url::Url;

#[derive(Clone, Debug)]
pub struct SlackClient<SCHC>
where
    SCHC: SlackClientHttpConnector + Send,
{
    pub http_api: SlackClientHttpApi<SCHC>,
}

#[derive(Clone, Debug)]
pub struct SlackClientHttpApi<SCHC>
where
    SCHC: SlackClientHttpConnector + Send,
{
    pub connector: Arc<SCHC>,
}

#[derive(Debug)]
pub struct SlackClientSession<'a, SCHC>
where
    SCHC: SlackClientHttpConnector + Send,
{
    pub http_session_api: SlackClientHttpSessionApi<'a, SCHC>,
}

#[derive(Debug)]
pub struct SlackClientHttpSessionApi<'a, SCHC>
where
    SCHC: SlackClientHttpConnector + Send,
{
    client: &'a SlackClient<SCHC>,
    token: &'a SlackApiToken,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct SlackClientApiCallContext<'a> {
    pub rate_control_params: Option<&'a SlackApiMethodRateControlConfig>,
    pub token: Option<&'a SlackApiToken>,
    pub tracing_span: &'a Span,
    pub is_sensitive_url: bool,
}

pub trait SlackClientHttpConnector {
    fn http_get_uri<'a, RS>(
        &'a self,
        full_uri: Url,
        context: SlackClientApiCallContext<'a>,
    ) -> BoxFuture<'a, ClientResult<RS>>
    where
        RS: for<'de> serde::de::Deserialize<'de> + Send + 'a + 'a + Send;

    fn http_get_with_client_secret<'a, RS>(
        &'a self,
        full_uri: Url,
        client_id: &'a SlackClientId,
        client_secret: &'a SlackClientSecret,
    ) -> BoxFuture<'a, ClientResult<RS>>
    where
        RS: for<'de> serde::de::Deserialize<'de> + Send + 'a + 'a + Send;

    fn http_get<'a, 'p, RS, PT, TS>(
        &'a self,
        method_relative_uri: &str,
        params: &'p PT,
        context: SlackClientApiCallContext<'a>,
    ) -> BoxFuture<'a, ClientResult<RS>>
    where
        RS: for<'de> serde::de::Deserialize<'de> + Send + 'a,
        PT: std::iter::IntoIterator<Item = (&'p str, Option<&'p TS>)> + Clone,
        TS: std::string::ToString + 'p + 'a + Send,
    {
        let full_uri = SlackClientHttpApiUri::create_url_with_params(
            &SlackClientHttpApiUri::create_method_uri_path(method_relative_uri),
            params,
        );

        self.http_get_uri(full_uri, context)
    }

    fn http_post_uri<'a, RQ, RS>(
        &'a self,
        full_uri: Url,
        request_body: &'a RQ,
        context: SlackClientApiCallContext<'a>,
    ) -> BoxFuture<'a, ClientResult<RS>>
    where
        RQ: serde::ser::Serialize + Send + Sync,
        RS: for<'de> serde::de::Deserialize<'de> + Send + 'a + Send + 'a;

    fn http_post<'a, RQ, RS>(
        &'a self,
        method_relative_uri: &str,
        request: &'a RQ,
        context: SlackClientApiCallContext<'a>,
    ) -> BoxFuture<'a, ClientResult<RS>>
    where
        RQ: serde::ser::Serialize + Send + Sync,
        RS: for<'de> serde::de::Deserialize<'de> + Send + 'a,
    {
        let full_uri = SlackClientHttpApiUri::create_url(
            &SlackClientHttpApiUri::create_method_uri_path(method_relative_uri),
        );

        self.http_post_uri(full_uri, request, context)
    }

    fn http_post_uri_form_urlencoded<'a, RQ, RS>(
        &'a self,
        full_uri: Url,
        request_body: &'a RQ,
        context: SlackClientApiCallContext<'a>,
    ) -> BoxFuture<'a, ClientResult<RS>>
    where
        RQ: serde::ser::Serialize + Send + Sync,
        RS: for<'de> serde::de::Deserialize<'de> + Send + 'a + Send + 'a;

    fn http_post_form_urlencoded<'a, RQ, RS>(
        &'a self,
        method_relative_uri: &str,
        request: &'a RQ,
        context: SlackClientApiCallContext<'a>,
    ) -> BoxFuture<'a, ClientResult<RS>>
    where
        RQ: serde::ser::Serialize + Send + Sync,
        RS: for<'de> serde::de::Deserialize<'de> + Send + 'a,
    {
        let full_uri = SlackClientHttpApiUri::create_url(
            &SlackClientHttpApiUri::create_method_uri_path(method_relative_uri),
        );

        self.http_post_uri_form_urlencoded(full_uri, request, context)
    }

    fn http_post_uri_multipart<'a, 'p, RS, PT, TS>(
        &'a self,
        full_uri: Url,
        file_name: String,
        file_content_type: String,
        file_content: &'p [u8],
        params: &'p PT,
        context: SlackClientApiCallContext<'a>,
    ) -> BoxFuture<'a, ClientResult<RS>>
    where
        RS: for<'de> serde::de::Deserialize<'de> + Send + 'a + Send + 'a,
        PT: std::iter::IntoIterator<Item = (&'p str, Option<&'p TS>)> + Clone,
        TS: std::string::ToString + 'p + 'a + Send;

    fn http_post_multipart<'a, 'p, RS, PT, TS>(
        &'a self,
        method_relative_uri: &str,
        file_name: String,
        file_content_type: String,
        file_content: &'p [u8],
        params: &'p PT,
        context: SlackClientApiCallContext<'a>,
    ) -> BoxFuture<'a, ClientResult<RS>>
    where
        RS: for<'de> serde::de::Deserialize<'de> + Send + 'a,
        PT: std::iter::IntoIterator<Item = (&'p str, Option<&'p TS>)> + Clone,
        TS: std::string::ToString + 'p + 'a + Send,
    {
        let full_uri = SlackClientHttpApiUri::create_url(
            &SlackClientHttpApiUri::create_method_uri_path(method_relative_uri),
        );

        self.http_post_uri_multipart(
            full_uri,
            file_name,
            file_content_type,
            file_content,
            params,
            context,
        )
    }
}

pub type UserCallbackResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub type ClientResult<T> = std::result::Result<T, SlackClientError>;

pub type AnyStdResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct SlackEnvelopeMessage {
    pub ok: bool,
    pub error: Option<String>,
    // Slack may return validation errors in `errors` field with `ok: false` for some methods (such as `apps.manifest.validate`.
    pub errors: Option<Vec<String>>,
    pub warnings: Option<Vec<String>>,
}

lazy_static! {
    pub static ref SLACK_HTTP_EMPTY_GET_PARAMS: Vec<(&'static str, Option<&'static String>)> =
        vec![];
}

impl<SCHC> SlackClientHttpApi<SCHC>
where
    SCHC: SlackClientHttpConnector + Send + Sync,
{
    fn new(http_connector: Arc<SCHC>) -> Self {
        Self {
            connector: http_connector,
        }
    }
}

pub struct SlackClientHttpApiUri;

impl SlackClientHttpApiUri {
    const SLACK_API_URI_STR: &'static str = "https://slack.com/api";

    pub fn create_method_uri_path(method_relative_uri: &str) -> String {
        format!("{}/{}", Self::SLACK_API_URI_STR, method_relative_uri)
    }

    pub(crate) fn create_url(url_str: &str) -> Url {
        url_str.parse().unwrap()
    }

    pub fn create_url_with_params<'p, PT, TS>(url_str: &str, params: &'p PT) -> Url
    where
        PT: std::iter::IntoIterator<Item = (&'p str, Option<&'p TS>)> + Clone,
        TS: std::string::ToString + 'p,
    {
        let url_query_params: Vec<(String, String)> = params
            .clone()
            .into_iter()
            .filter_map(|(k, vo)| vo.map(|v| (k.to_string(), v.to_string())))
            .collect();

        Url::parse_with_params(url_str, url_query_params)
            .unwrap()
            .as_str()
            .parse()
            .unwrap()
    }
}

impl<SCHC> SlackClient<SCHC>
where
    SCHC: SlackClientHttpConnector + Send + Sync,
{
    pub fn new(http_connector: SCHC) -> Self {
        Self {
            http_api: SlackClientHttpApi::new(Arc::new(http_connector)),
        }
    }

    pub fn open_session<'a>(&'a self, token: &'a SlackApiToken) -> SlackClientSession<'a, SCHC> {
        let http_session_span = span!(
            Level::DEBUG,
            "Slack API request",
            "/slack/team_id" = token
                .team_id
                .as_ref()
                .map(|team_id| team_id.value().as_str())
                .unwrap_or_else(|| "-")
        );

        let http_session_api = SlackClientHttpSessionApi {
            client: self,
            token,
            span: http_session_span,
        };

        SlackClientSession { http_session_api }
    }

    pub async fn run_in_session<'a, FN, F, T>(&'a self, token: &'a SlackApiToken, pred: FN) -> T
    where
        FN: Fn(SlackClientSession<'a, SCHC>) -> F,
        F: Future<Output = T>,
    {
        let session = self.open_session(token);
        pred(session).await
    }
}

impl<'a, SCHC> SlackClientHttpSessionApi<'a, SCHC>
where
    SCHC: SlackClientHttpConnector + Send,
{
    pub async fn http_get_uri<RS, PT, TS>(
        &self,
        full_uri: Url,
        rate_control_params: Option<&'a SlackApiMethodRateControlConfig>,
    ) -> ClientResult<RS>
    where
        RS: for<'de> serde::de::Deserialize<'de> + Send,
    {
        let context = SlackClientApiCallContext {
            rate_control_params,
            token: Some(self.token),
            tracing_span: &self.span,
            is_sensitive_url: false,
        };

        self.client
            .http_api
            .connector
            .http_get_uri(full_uri, context)
            .await
    }

    pub async fn http_get<'p, RS, PT, TS>(
        &self,
        method_relative_uri: &str,
        params: &'p PT,
        rate_control_params: Option<&'a SlackApiMethodRateControlConfig>,
    ) -> ClientResult<RS>
    where
        RS: for<'de> serde::de::Deserialize<'de> + Send,
        PT: std::iter::IntoIterator<Item = (&'p str, Option<&'p TS>)> + Clone,
        TS: std::string::ToString + 'p + Send,
    {
        let context = SlackClientApiCallContext {
            rate_control_params,
            token: Some(self.token),
            tracing_span: &self.span,
            is_sensitive_url: false,
        };

        self.client
            .http_api
            .connector
            .http_get(method_relative_uri, params, context)
            .await
    }

    pub async fn http_post<RQ, RS>(
        &self,
        method_relative_uri: &str,
        request: &RQ,
        rate_control_params: Option<&'a SlackApiMethodRateControlConfig>,
    ) -> ClientResult<RS>
    where
        RQ: serde::ser::Serialize + Send + Sync,
        RS: for<'de> serde::de::Deserialize<'de> + Send,
    {
        let context = SlackClientApiCallContext {
            rate_control_params,
            token: Some(self.token),
            tracing_span: &self.span,
            is_sensitive_url: false,
        };

        self.client
            .http_api
            .connector
            .http_post(method_relative_uri, &request, context)
            .await
    }

    pub async fn http_post_uri<RQ, RS>(
        &self,
        full_uri: Url,
        request: &RQ,
        rate_control_params: Option<&'a SlackApiMethodRateControlConfig>,
    ) -> ClientResult<RS>
    where
        RQ: serde::ser::Serialize + Send + Sync,
        RS: for<'de> serde::de::Deserialize<'de> + Send,
    {
        let context = SlackClientApiCallContext {
            rate_control_params,
            token: Some(self.token),
            tracing_span: &self.span,
            is_sensitive_url: false,
        };

        self.client
            .http_api
            .connector
            .http_post_uri(full_uri, &request, context)
            .await
    }

    pub async fn http_post_form_urlencoded<RQ, RS>(
        &self,
        method_relative_uri: &str,
        request: &RQ,
        rate_control_params: Option<&'a SlackApiMethodRateControlConfig>,
    ) -> ClientResult<RS>
    where
        RQ: serde::ser::Serialize + Send + Sync,
        RS: for<'de> serde::de::Deserialize<'de> + Send,
    {
        let context = SlackClientApiCallContext {
            rate_control_params,
            token: Some(self.token),
            tracing_span: &self.span,
            is_sensitive_url: false,
        };

        self.client
            .http_api
            .connector
            .http_post_form_urlencoded(method_relative_uri, &request, context)
            .await
    }

    pub async fn http_post_multipart<'p, RS, PT, TS>(
        &self,
        method_relative_uri: &str,
        file_name: String,
        file_content_type: String,
        file_content: &'p [u8],
        params: &'p PT,
        rate_control_params: Option<&'a SlackApiMethodRateControlConfig>,
    ) -> ClientResult<RS>
    where
        PT: std::iter::IntoIterator<Item = (&'p str, Option<&'p TS>)> + Clone,
        RS: for<'de> serde::de::Deserialize<'de> + Send,
        TS: std::string::ToString + 'p + Send,
    {
        let context = SlackClientApiCallContext {
            rate_control_params,
            token: Some(self.token),
            tracing_span: &self.span,
            is_sensitive_url: false,
        };

        self.client
            .http_api
            .connector
            .http_post_multipart(
                method_relative_uri,
                file_name,
                file_content_type,
                file_content,
                params,
                context,
            )
            .await
    }
}
