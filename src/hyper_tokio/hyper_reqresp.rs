use crate::signature_verifier::*;
use crate::{AnyStdResult, SlackApiToken};
use bytes::Buf;
use futures_util::TryFutureExt;
use http::{Request, Response, Uri};
use hyper::body::HttpBody;
use hyper::Body;
use mime::Mime;
use rvstruct::ValueStruct;
use std::collections::HashMap;
use std::io::Read;
use url::Url;

pub struct HyperReqRespUtils;

impl HyperReqRespUtils {
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

    pub fn setup_token_auth_header(
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

    pub fn setup_basic_auth_header(
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

    pub fn create_http_request(
        url: Url,
        method: hyper::http::Method,
    ) -> hyper::http::request::Builder {
        let uri: Uri = url.as_str().parse().unwrap();
        hyper::http::request::Builder::new()
            .method(method)
            .uri(uri)
            .header("accept-charset", "utf-8")
    }

    pub async fn http_body_to_string<T>(body: T) -> AnyStdResult<String>
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

    pub fn http_response_content_type<RS>(response: &Response<RS>) -> Option<Mime> {
        let http_headers = response.headers();
        http_headers.get(hyper::header::CONTENT_TYPE).map(|hv| {
            let hvs = hv.to_str().unwrap();
            hvs.parse::<Mime>().unwrap()
        })
    }

    pub async fn decode_signed_response(
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
}
