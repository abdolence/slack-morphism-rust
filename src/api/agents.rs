//!
//! Support for Slack agents.sessions.* methods
//!

use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::*;
use crate::ratectl::*;
use crate::SlackClientSession;
use crate::{ClientResult, SlackClientHttpConnector};

impl<'a, SCHC> SlackClientSession<'a, SCHC>
where
    SCHC: SlackClientHttpConnector + Send,
{
    ///
    /// https://docs.slack.dev/reference/methods/agents.sessions.setStatus
    ///
    pub async fn agents_sessions_set_status(
        &self,
        req: &SlackApiAgentsSessionsSetStatusRequest,
    ) -> ClientResult<SlackApiAgentsSessionsSetStatusResponse> {
        self.http_session_api
            .http_post(
                "agents.sessions.setStatus",
                req,
                Some(&SLACK_TIER3_METHOD_CONFIG),
            )
            .await
    }

    ///
    /// https://docs.slack.dev/reference/methods/agents.sessions.rename
    ///
    pub async fn agents_sessions_rename(
        &self,
        req: &SlackApiAgentsSessionsRenameRequest,
    ) -> ClientResult<SlackApiAgentsSessionsRenameResponse> {
        self.http_session_api
            .http_post(
                "agents.sessions.rename",
                req,
                Some(&SLACK_TIER3_METHOD_CONFIG),
            )
            .await
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiAgentsSessionsSetStatusRequest {
    pub status: String,
    pub channel_id: Option<SlackChannelId>,
    pub thread_ts: Option<SlackTs>,
    pub title: Option<String>,
    pub initiator_user_id: Option<SlackUserId>,
    pub icon_emoji: Option<String>,
    pub icon_url: Option<String>,
    pub username: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiAgentsSessionsSetStatusResponse {
    pub status: Option<String>,
    pub agent_status: Option<String>,
    pub title: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiAgentsSessionsRenameRequest {
    pub channel_id: SlackChannelId,
    pub thread_ts: SlackTs,
    pub title: String,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiAgentsSessionsRenameResponse {}
