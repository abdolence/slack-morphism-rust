//!
//! Support for Slack Auth API methods
//!

use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SlackClientSession;
use crate::{ClientResult, SlackClientHttpConnector};
use slack_morphism_models::*;

impl<'a, SCHC> SlackClientSession<'a, SCHC>
where
    SCHC: SlackClientHttpConnector + Send,
{
    ///
    /// https://api.slack.com/methods/auth.test
    ///
    pub async fn auth_test(&self) -> ClientResult<SlackApiAuthTestResponse> {
        self.http_session_api
            .http_get(
                "auth.test",
                &crate::client::SLACK_HTTP_EMPTY_GET_PARAMS.clone(),
            )
            .await
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiAuthTestResponse {
    pub user_id: SlackUserId,
    pub team_id: SlackTeamId,
    pub user: Option<String>,
    pub team: String,
    pub bot_id: Option<SlackBotId>,
    pub url: SlackTeamUrl,
}
