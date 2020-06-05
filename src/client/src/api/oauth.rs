use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{ClientResult, SlackClient};
use hyper::Body;
use slack_morphism_models::common::*;
use std::collections::HashMap;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackOAuthV2AccessTokenRequest {
    pub client_id: String,
    pub client_secret: String,
    pub code: String,
    pub redirect_uri: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackOAuthV2AccessTokenResponse {
    pub id: String,
    pub deleted: Option<bool>,
    pub name: String,
    pub updated: Option<SlackDateTime>,
    pub app_id: String,
    pub user_id: String,
    pub icons: Option<HashMap<String, String>>,
}

impl SlackClient {
    pub async fn oauth2_access(
        &self,
        req: &SlackOAuthV2AccessTokenRequest,
    ) -> ClientResult<SlackOAuthV2AccessTokenResponse> {
        let full_uri = SlackClient::create_url_with_params(
            &SlackClient::create_method_uri_path("oauth.v2.access"),
            vec![
                ("code", Some(req.code.clone())),
                ("redirect_uri", req.redirect_uri.clone()),
            ],
        );

        let http_request = SlackClient::setup_basic_auth_header(
            SlackClient::create_http_request(full_uri, hyper::http::Method::GET),
            &req.client_id,
            &req.client_secret,
        )
        .body(Body::empty())?;

        self.send_webapi_request(http_request).await
    }
}
