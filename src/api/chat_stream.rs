//!
//! Support for Slack chat.startStream / chat.appendStream / chat.stopStream methods
//!

use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::api::SlackAgentSessionStatus;
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
    pub task_display_mode: Option<SlackStreamTaskDisplayMode>,
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
    pub session_status: Option<SlackAgentSessionStatus>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiChatStopStreamResponse {
    pub channel: Option<SlackChannelId>,
    pub ts: Option<SlackTs>,
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
        id: String,
        title: String,
        hide_title: Option<bool>,
        icon: Option<SlackStreamTaskIcon>,
        status: SlackStreamTaskStatus,
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

/// Status of a `task_update` chunk.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SlackStreamTaskStatus {
    /// Accepted by the API schema although absent from the docs.
    Pending,
    InProgress,
    Complete,
    Error,
}

/// How task updates are rendered in a streamed message.
/// https://docs.slack.dev/reference/methods/chat.startStream#arg_task_display_mode
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SlackStreamTaskDisplayMode {
    Timeline,
    Plan,
}

/// Icon shown next to a `task_update` chunk, serialised as `{"type":"icon","name":"<icon name>"}`.
///
/// `name` is an icon name (e.g. `check`), not a URL: the docs example with a URL in `name`
/// is rejected by the API, as are `type: image` / `type: emoji`.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize, Builder)]
#[serde(from = "SlackStreamTaskIconRepr", into = "SlackStreamTaskIconRepr")]
pub struct SlackStreamTaskIcon {
    pub name: String,
}

/// Wire shape of `SlackStreamTaskIcon`; `type` is always `icon`.
#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum SlackStreamTaskIconRepr {
    Icon { name: String },
}

impl From<SlackStreamTaskIconRepr> for SlackStreamTaskIcon {
    fn from(SlackStreamTaskIconRepr::Icon { name }: SlackStreamTaskIconRepr) -> Self {
        Self { name }
    }
}

impl From<SlackStreamTaskIcon> for SlackStreamTaskIconRepr {
    fn from(icon: SlackStreamTaskIcon) -> Self {
        Self::Icon { name: icon.name }
    }
}

/// A source reference attached to a `task_update` chunk.
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackStreamSource {
    /// Open set (`url`, `file`, ...): the API accepts arbitrary strings here.
    #[serde(rename = "type")]
    pub source_type: String,
    pub url: String,
    /// Required: the API rejects a source without `text`.
    pub text: String,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_task_update_chunk_round_trip() {
        let json = serde_json::json!({
            "type": "task_update",
            "id": "t1",
            "title": "Searching",
            "hide_title": true,
            "icon": { "type": "icon", "name": "check" },
            "status": "in_progress",
            "sources": [{ "type": "url", "url": "https://example.com", "text": "Example" }]
        });
        let chunk: SlackStreamChunk = serde_json::from_value(json.clone()).unwrap();
        assert_eq!(
            chunk,
            SlackStreamChunk::TaskUpdate {
                id: "t1".into(),
                title: "Searching".into(),
                hide_title: Some(true),
                icon: Some(SlackStreamTaskIcon::new("check".into())),
                status: SlackStreamTaskStatus::InProgress,
                details: None,
                output: None,
                sources: Some(vec![SlackStreamSource::new(
                    "url".into(),
                    "https://example.com".into(),
                    "Example".into()
                )]),
            }
        );
        assert_eq!(serde_json::to_value(&chunk).unwrap(), json);
    }

    #[test]
    fn test_task_icon_rejects_non_icon_type() {
        assert!(
            serde_json::from_str::<SlackStreamTaskIcon>(r#"{"type":"image","name":"x"}"#).is_err()
        );
    }
}
