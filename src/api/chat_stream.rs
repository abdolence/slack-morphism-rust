//!
//! Support for Slack chat.startStream / chat.appendStream / chat.stopStream methods
//!

use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::blocks::SlackBlock;
use crate::models::*;
use crate::ratectl::*;
use crate::SlackClientSession;
use crate::{ClientResult, SlackClientHttpConnector};

impl<'a, SCHC> SlackClientSession<'a, SCHC>
where
    SCHC: SlackClientHttpConnector + Send,
{
    ///
    /// https://docs.slack.dev/reference/methods/chat.startStream
    ///
    pub async fn chat_start_stream(
        &self,
        req: &SlackApiChatStartStreamRequest,
    ) -> ClientResult<SlackApiChatStartStreamResponse> {
        self.http_session_api
            .http_post("chat.startStream", req, Some(&SLACK_TIER2_METHOD_CONFIG))
            .await
    }

    ///
    /// https://docs.slack.dev/reference/methods/chat.appendStream
    ///
    pub async fn chat_append_stream(
        &self,
        req: &SlackApiChatAppendStreamRequest,
    ) -> ClientResult<SlackApiChatAppendStreamResponse> {
        self.http_session_api
            .http_post("chat.appendStream", req, Some(&SLACK_TIER4_METHOD_CONFIG))
            .await
    }

    ///
    /// https://docs.slack.dev/reference/methods/chat.stopStream
    ///
    pub async fn chat_stop_stream(
        &self,
        req: &SlackApiChatStopStreamRequest,
    ) -> ClientResult<SlackApiChatStopStreamResponse> {
        self.http_session_api
            .http_post("chat.stopStream", req, Some(&SLACK_TIER2_METHOD_CONFIG))
            .await
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiChatStartStreamRequest {
    pub channel: SlackChannelId,
    pub thread_ts: Option<SlackTs>,
    pub recipient_user_id: Option<SlackUserId>,
    pub recipient_team_id: Option<SlackTeamId>,
    pub markdown_text: Option<String>,
    pub chunks: Option<Vec<SlackStreamChunk>>,
    pub task_display_mode: Option<String>,
    pub icon_emoji: Option<String>,
    pub icon_url: Option<String>,
    pub username: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiChatStartStreamResponse {
    pub channel: SlackChannelId,
    pub ts: SlackTs,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiChatAppendStreamRequest {
    pub channel: SlackChannelId,
    pub ts: SlackTs,
    pub markdown_text: Option<String>,
    pub chunks: Option<Vec<SlackStreamChunk>>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiChatAppendStreamResponse {
    pub channel: Option<SlackChannelId>,
    pub ts: Option<SlackTs>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiChatStopStreamRequest {
    pub channel: SlackChannelId,
    pub ts: SlackTs,
    pub markdown_text: Option<String>,
    pub chunks: Option<Vec<SlackStreamChunk>>,
    pub blocks: Option<Vec<SlackBlock>>,
    pub metadata: Option<SlackMessageMetadata>,
    pub session_status: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiChatStopStreamResponse {
    pub channel: Option<SlackChannelId>,
    pub ts: Option<SlackTs>,
}

/// A streamed content chunk for `chat.startStream` / `chat.appendStream` / `chat.stopStream`.
/// https://docs.slack.dev/reference/methods/chat.appendStream#chunks
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SlackStreamChunk {
    MarkdownText {
        text: String,
    },
    TaskUpdate {
        id: String,
        title: String,
        status: String,
        details: Option<String>,
        output: Option<String>,
        sources: Option<Vec<SlackStreamSource>>,
    },
    PlanUpdate {
        title: String,
    },
    Blocks {
        blocks: Vec<SlackBlock>,
    },
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackStreamSource {
    #[serde(rename = "type")]
    pub source_type: String,
    pub url: String,
    pub text: Option<String>,
}
