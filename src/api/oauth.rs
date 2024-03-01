//!
//! Support for Slack OAuth v2 API methods
//!

use rsb_derive::Builder;
use rvstruct::*;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::fmt;

use crate::client::*;
use crate::models::*;
use crate::token::*;
use url::Url;

impl<SCHC> SlackClient<SCHC>
where
    SCHC: SlackClientHttpConnector + Send,
{
    ///
    /// https://api.slack.com/methods/oauth.v2.access
    ///
    pub async fn oauth2_access(
        &self,
        req: &SlackOAuthV2AccessTokenRequest,
    ) -> ClientResult<SlackOAuthV2AccessTokenResponse> {
        let full_uri: Url = SlackClientHttpApiUri::create_url_with_params(
            self.http_api
                .connector
                .create_method_uri_path("oauth.v2.access")?,
            &vec![
                ("code", Some(req.code.value())),
                (
                    "redirect_uri",
                    req.redirect_uri
                        .as_ref()
                        .map(|url| url.as_str().to_string())
                        .as_ref(),
                ),
            ],
        )?;

        self.http_api
            .connector
            .http_get_with_client_secret(full_uri, &req.client_id, &req.client_secret)
            .await
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackOAuthV2AccessTokenRequest {
    pub client_id: SlackClientId,
    pub client_secret: SlackClientSecret,
    pub code: SlackOAuthCode,
    pub redirect_uri: Option<Url>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackOAuthV2AccessTokenResponse {
    pub access_token: SlackApiTokenValue,
    pub token_type: SlackApiTokenType,
    pub scope: SlackApiTokenScope,
    pub bot_user_id: Option<SlackUserId>,
    pub app_id: SlackAppId,
    pub team: SlackTeamInfo,
    pub authed_user: SlackOAuthV2AuthedUser,
    pub incoming_webhook: Option<SlackOAuthIncomingWebHook>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackOAuthV2AuthedUser {
    pub id: SlackUserId,
    pub scope: Option<SlackApiTokenScope>,
    pub access_token: Option<SlackApiTokenValue>,
    pub token_type: Option<SlackApiTokenType>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackOAuthIncomingWebHook {
    pub channel: String,
    pub channel_id: SlackChannelId,
    pub configuration_url: Url,
    pub url: Url,
}

#[derive(Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackOAuthCode(pub String);

impl fmt::Debug for SlackOAuthCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SlackOAuthCode(len:{})", self.value().len())
    }
}
