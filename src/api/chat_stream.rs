//!
//! Support for Slack chat.startStream / chat.appendStream / chat.stopStream methods
//!

use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use url::Url;

use crate::api::SlackAgentSessionStatus;
use crate::models::blocks::*;
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
    pub task_display_mode: Option<SlackStreamTaskDisplayMode>,
    pub icon_emoji: Option<SlackEmoji>,
    pub icon_url: Option<Url>,
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
    pub channel: SlackChannelId,
    pub ts: SlackTs,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiChatStopStreamRequest {
    pub channel: SlackChannelId,
    pub ts: SlackTs,
    #[serde(flatten)]
    pub content: SlackMessageContent,
    pub chunks: Option<Vec<SlackStreamChunk>>,
    pub session_status: Option<SlackAgentSessionStatus>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiChatStopStreamResponse {
    pub channel: SlackChannelId,
    pub ts: SlackTs,
    pub message: Option<SlackMessage>,
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
        id: SlackTaskId,
        title: String,
        hide_title: Option<bool>,
        icon: Option<SlackTaskCardIcon>,
        status: SlackTaskCardStatus,
        details: Option<String>,
        output: Option<String>,
        /// Slack rejects a source without `text`.
        sources: Option<Vec<SlackTaskCardSource>>,
    },
    PlanUpdate {
        title: String,
    },
    Blocks {
        blocks: Vec<SlackBlock>,
    },
}

/// How task updates are rendered in a streamed message.
/// https://docs.slack.dev/reference/methods/chat.startStream#arg_task_display_mode
#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SlackStreamTaskDisplayMode {
    Timeline,
    Plan,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_slack_api_chat_stream_task_update_chunk_round_trip() {
        let json = serde_json::json!({
            "type": "task_update",
            "id": "t1",
            "title": "Searching",
            "hide_title": true,
            "icon": { "type": "icon", "name": "check" },
            "status": "in_progress",
            "sources": [{ "type": "url", "url": "https://example.com/", "text": "Example" }]
        });
        let chunk: SlackStreamChunk = serde_json::from_value(json.clone()).unwrap();
        assert_eq!(
            chunk,
            SlackStreamChunk::TaskUpdate {
                id: "t1".into(),
                title: "Searching".into(),
                hide_title: Some(true),
                icon: Some(SlackTaskCardIcon::new("check".into())),
                status: SlackTaskCardStatus::InProgress,
                details: None,
                output: None,
                sources: Some(vec![SlackTaskCardSource::Url(SlackUrlSourceElement::new(
                    Url::parse("https://example.com").unwrap(),
                    "Example".into()
                ))]),
            }
        );
        assert_eq!(serde_json::to_value(&chunk).unwrap(), json);
    }
}
