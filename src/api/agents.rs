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
    pub status: SlackAgentSessionStatus,
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
    pub status: Option<SlackAgentSessionStatus>,
    pub agent_status: Option<SlackAgentSessionStatus>,
    pub title: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiAgentsSessionsRenameRequest {
    /// Required together with `thread_ts` for thread sessions in DMs/channels;
    /// must be omitted for session channels.
    pub channel_id: Option<SlackChannelId>,
    /// See `channel_id`.
    pub thread_ts: Option<SlackTs>,
    pub title: String,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiAgentsSessionsRenameResponse {
    pub title: Option<String>,
}

/// Agent session status for `agents.sessions.setStatus` and `chat.stopStream`'s `session_status`.
/// https://docs.slack.dev/reference/methods/agents.sessions.setStatus#arg_status
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SlackAgentSessionStatus {
    Active,
    Processing,
    Suspended,
    Closed,
    /// A value this crate does not model yet, so a new status does not fail the response.
    #[serde(other)]
    Other,
}
