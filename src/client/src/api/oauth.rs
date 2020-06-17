use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{ClientResult, SlackClient, SlackClientHttpApi};
use hyper::Body;
use slack_morphism_models::*;

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
    pub access_token: String,
    pub token_type: String,
    pub scope: String,
    pub bot_user_id: Option<String>,
    pub app_id: String,
    pub team: SlackTeamInfo,
    pub authed_user: SlackOAuthV2AuthedUser,
    pub incoming_webhook: Option<SlackOAuthIncomingWebHook>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackOAuthV2AuthedUser {
    pub id: String,
    pub scope: Option<String>,
    pub access_token: Option<String>,
    pub token_type: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackOAuthIncomingWebHook {
    pub channel: String,
    pub channel_id: SlackChannelId,
    pub configuration_url: String,
    pub url: String,
}

impl SlackClient {
    pub async fn oauth2_access(
        &self,
        req: &SlackOAuthV2AccessTokenRequest,
    ) -> ClientResult<SlackOAuthV2AccessTokenResponse> {
        let full_uri = SlackClientHttpApi::create_url_with_params(
            &SlackClientHttpApi::create_method_uri_path("oauth.v2.access"),
            &vec![
                ("code", Some(&req.code)),
                ("redirect_uri", req.redirect_uri.as_ref()),
            ],
        );

        let http_request = SlackClientHttpApi::setup_basic_auth_header(
            SlackClientHttpApi::create_http_request(full_uri, hyper::http::Method::GET),
            &req.client_id,
            &req.client_secret,
        )
        .body(Body::empty())?;

        self.http_api.send_webapi_request(http_request).await
    }
}
