pub mod api;
pub mod errors;
pub mod listener;
pub mod scroller;

use serde::{Deserialize, Serialize};

use bytes::buf::BufExt as _;
use hyper::client::*;
use hyper::http::StatusCode;
use hyper::{Body, Request, Response, Uri};
use hyper_tls::HttpsConnector;
use rsb_derive::Builder;
use std::collections::HashMap;
use std::io::Read;
use url::Url;
use hyper::body::HttpBody;

#[derive(Debug, PartialEq, Clone, Builder)]
pub struct SlackApiToken {
    value: String,
    workspace_id: Option<String>,
    scope: Option<String>,
}

#[derive(Debug)]
pub struct SlackClient {
    pub http_api: SlackClientHttpApi,
}

#[derive(Debug)]
pub struct SlackClientHttpApi {
    connector: Client<HttpsConnector<HttpConnector>>,
}

#[derive(Debug)]
pub struct SlackClientSession<'a> {
    pub http_api: SlackClientHttpSessionApi<'a>,
    client: &'a SlackClient,
    token: &'a SlackApiToken,
}

#[derive(Debug)]
pub struct SlackClientHttpSessionApi<'a> {
    client: &'a SlackClient,
    token: &'a SlackApiToken,
}

pub type ClientResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct SlackEnvelopeMessage {
    ok: bool,
    error: Option<String>,
    warnings: Option<Vec<String>>,
}

impl SlackClientHttpApi {
    const SLACK_API_URI_STR: &'static str = "https://slack.com/api";

    fn new() -> Self {
        let https_connector = HttpsConnector::new();
        Self {
            connector: Client::builder().build(https_connector),
        }
    }

    fn create_method_uri_path(method_relative_uri: &str) -> String {
        format!(
            "{}/{}",
            Self::SLACK_API_URI_STR,
            method_relative_uri
        )
    }

    fn create_url(url_str: &String) -> Uri {
        url_str.parse().unwrap()
    }

    fn create_url_with_params<'p, PT, TS>(url_str: &str, params: &'p PT) -> Uri
    where
        PT: std::iter::IntoIterator<Item = (&'p str, Option<&'p TS>)> + Clone,
        TS: std::string::ToString + 'p,
    {
        let url_query_params: Vec<(String, String)> = params
            .clone()
            .into_iter()
            .map(|(k, vo)| vo.map(|v| (k.to_string(), v.to_string())))
            .flatten()
            .collect();

        Url::parse_with_params(url_str, url_query_params)
            .unwrap()
            .as_str()
            .parse()
            .unwrap()
    }

    pub fn parse_query_params(request: &Request<Body>) -> HashMap<String, String> {
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

    pub fn hyper_redirect_to(
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
            let token_header_value = format!("Bearer {}", token.unwrap().value);
            request_builder.header(hyper::header::AUTHORIZATION, token_header_value)
        }
    }

    fn setup_basic_auth_header(
        request_builder: hyper::http::request::Builder,
        username: &String,
        password: &String,
    ) -> hyper::http::request::Builder {
        let header_value = format!(
            "Basic {}",
            base64::encode(format!("{}:{}", username, password))
        );
        request_builder.header(hyper::header::AUTHORIZATION, header_value)
    }

    fn create_http_request(uri: Uri, method: hyper::http::Method) -> hyper::http::request::Builder {
        hyper::http::request::Builder::new()
            .method(method)
            .uri(uri)
            .header("accept-charset", "utf-8")
    }

    async fn http_body_to_string<T>(body : T) -> ClientResult<String> where T: HttpBody, T::Error : std::error::Error + Sync + Send + 'static {
        let http_body = hyper::body::aggregate(body).await?;
        let mut http_reader = http_body.reader();
        let mut http_body_str = String::new();
        http_reader.read_to_string(&mut http_body_str)?;
        Ok(http_body_str)
    }

    async fn send_webapi_request<RS>(&self, request: Request<Body>) -> ClientResult<RS>
    where
        RS: for<'de> serde::de::Deserialize<'de>,
    {
        let http_res = self.connector.request(request).await?;
        let http_status = http_res.status();
        let http_body_str = Self::http_body_to_string(http_res).await?;

        match http_status {
            StatusCode::OK => {
                let slack_message: SlackEnvelopeMessage =
                    serde_json::from_str(http_body_str.as_str())?;
                if slack_message.error.is_none() {
                    let decoded_body = serde_json::from_str(http_body_str.as_str())?;
                    Ok(decoded_body)
                } else {
                    Err(errors::SlackClientError::ApiError(
                        errors::SlackClientApiError::new(slack_message.error.unwrap())
                            .opt_warnings(slack_message.warnings)
                            .with_http_response_body(http_body_str),
                    )
                    .into())
                }
            }
            _ => Err(errors::SlackClientError::HttpError(
                errors::SlackClientHttpError::new(http_status)
                    .with_http_response_body(http_body_str),
            )
            .into()),
        }
    }

    pub async fn http_get_uri<'a, RS>(
        &self,
        full_uri: Uri,
        token: Option<&'a SlackApiToken>,
    ) -> ClientResult<RS>
    where
        RS: for<'de> serde::de::Deserialize<'de>,
    {
        let base_http_request =
            Self::create_http_request(full_uri, hyper::http::Method::GET);

        let http_request = Self::setup_token_auth_header(base_http_request, token);

        let body = self
            .send_webapi_request(http_request.body(Body::empty())?)
            .await?;

        Ok(body)
    }

    async fn http_get_token<'a, 'p, RS, PT, TS>(
        &self,
        method_relative_uri: &str,
        params: &'p PT,
        token: Option<&'a SlackApiToken>,
    ) -> ClientResult<RS>
    where
        RS: for<'de> serde::de::Deserialize<'de>,
        PT: std::iter::IntoIterator<Item = (&'p str, Option<&'p TS>)> + Clone,
        TS: std::string::ToString + 'p,
    {
        let full_uri = Self::create_url_with_params(
            &Self::create_method_uri_path(&method_relative_uri),
            params,
        );

        self.http_get_uri(full_uri, token).await
    }

    pub async fn http_get<'p, RS, PT, TS>(
        &self,
        method_relative_uri: &str,
        params: &'p PT,
    ) -> ClientResult<RS>
    where
        RS: for<'de> serde::de::Deserialize<'de>,
        PT: std::iter::IntoIterator<Item = (&'p str, Option<&'p TS>)> + Clone,
        TS: std::string::ToString + 'p,
    {
        self.http_get_token(&method_relative_uri, params, None)
            .await
    }

    pub async fn http_post_uri<'a, RQ, RS>(
        &self,
        full_uri: Uri,
        request_body: &RQ,
        token: Option<&'a SlackApiToken>,
    ) -> ClientResult<RS>
    where
        RQ: serde::ser::Serialize,
        RS: for<'de> serde::de::Deserialize<'de>,
    {
        let post_json = serde_json::to_string(&request_body)?;

        let base_http_request =
            Self::create_http_request(full_uri, hyper::http::Method::POST)
                .header("content-type", "application/json; charset=utf-8");

        let http_request = Self::setup_token_auth_header(base_http_request, token);

        let response_body = self
            .send_webapi_request(http_request.body(post_json.into())?)
            .await?;

        Ok(response_body)
    }

    async fn http_post_token<'a, RQ, RS>(
        &self,
        method_relative_uri: &str,
        request: &RQ,
        token: Option<&'a SlackApiToken>,
    ) -> ClientResult<RS>
    where
        RQ: serde::ser::Serialize,
        RS: for<'de> serde::de::Deserialize<'de>,
    {
        let full_uri = Self::create_url(&SlackClientHttpApi::create_method_uri_path(
            &method_relative_uri,
        ));

        self.http_post_uri(full_uri, &request, token).await
    }

    pub async fn http_post<RQ, RS>(
        &self,
        method_relative_uri: &str,
        request: &RQ,
    ) -> ClientResult<RS>
    where
        RQ: serde::ser::Serialize,
        RS: for<'de> serde::de::Deserialize<'de>,
    {
        self.http_post_token(method_relative_uri, &request, None)
            .await
    }
}

impl SlackClient {
    pub fn new() -> Self {
        Self {
            http_api: SlackClientHttpApi::new(),
        }
    }

    pub fn open_session<'a>(&'a self, token: &'a SlackApiToken) -> SlackClientSession<'a> {
        let http_session_api = SlackClientHttpSessionApi {
            client: self,
            token,
        };

        SlackClientSession {
            client: self,
            token,
            http_api: http_session_api,
        }
    }
}

impl<'a> SlackClientHttpSessionApi<'a> {
    pub async fn http_get_uri<RS, PT, TS>(&self, full_uri: Uri) -> ClientResult<RS>
    where
        RS: for<'de> serde::de::Deserialize<'de>,
    {
        self.client
            .http_api
            .http_get_uri(full_uri, Some(&self.token))
            .await
    }

    pub async fn http_get<'p, RS, PT, TS>(
        &self,
        method_relative_uri: &str,
        params: &'p PT,
    ) -> ClientResult<RS>
    where
        RS: for<'de> serde::de::Deserialize<'de>,
        PT: std::iter::IntoIterator<Item = (&'p str, Option<&'p TS>)> + Clone,
        TS: std::string::ToString + 'p,
    {
        self.client
            .http_api
            .http_get_token(&method_relative_uri, params, Some(&self.token))
            .await
    }

    pub async fn http_post<RQ, RS>(
        &self,
        method_relative_uri: &str,
        request: &RQ,
    ) -> ClientResult<RS>
    where
        RQ: serde::ser::Serialize,
        RS: for<'de> serde::de::Deserialize<'de>,
    {
        self.client
            .http_api
            .http_post_token(&method_relative_uri, &request, Some(&self.token))
            .await
    }

    pub async fn http_post_uri<RQ, RS>(&self, full_uri: Uri, request: &RQ) -> ClientResult<RS>
    where
        RQ: serde::ser::Serialize,
        RS: for<'de> serde::de::Deserialize<'de>,
    {
        self.client
            .http_api
            .http_post_uri(full_uri, &request, Some(&self.token))
            .await
    }
}
