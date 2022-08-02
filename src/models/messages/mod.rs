use crate::blocks::*;
use crate::events::SlackMessageEventType;
use crate::*;
use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

mod templates;

use crate::SlackFile;
pub use templates::*;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackMessageOrigin {
    pub ts: SlackTs,
    pub channel: Option<SlackChannelId>,
    pub channel_type: Option<SlackChannelType>,
    pub thread_ts: Option<SlackTs>,
    pub client_msg_id: Option<SlackClientMessageId>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackMessageContent {
    pub text: Option<String>,
    pub blocks: Option<Vec<SlackBlock>>,
    pub attachments: Option<Vec<SlackMessageAttachment>>,
    pub upload: Option<bool>,
    pub files: Option<Vec<SlackFile>>,
    pub reactions: Option<Vec<SlackReaction>>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackMessageSender {
    pub user: Option<SlackUserId>,
    pub bot_id: Option<SlackBotId>,
    pub username: Option<String>,
    pub display_as_bot: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackParentMessageParams {
    pub reply_count: Option<usize>,
    pub reply_users_count: Option<usize>,
    pub latest_reply: Option<SlackTs>,
    pub reply_users: Option<Vec<SlackUserId>>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackMessage {
    #[serde(flatten)]
    pub origin: SlackMessageOrigin,
    #[serde(flatten)]
    pub content: SlackMessageContent,
    #[serde(flatten)]
    pub parent: SlackParentMessageParams,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackHistoryMessage {
    #[serde(flatten)]
    pub origin: SlackMessageOrigin,
    #[serde(flatten)]
    pub content: SlackMessageContent,
    #[serde(flatten)]
    pub sender: SlackMessageSender,
    #[serde(flatten)]
    pub parent: SlackParentMessageParams,
    pub subtype: Option<SlackMessageEventType>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackUpdatedMessage {
    #[serde(flatten)]
    pub sender: SlackMessageSender,
    #[serde(flatten)]
    pub content: SlackMessageContent,
    pub edited: Option<SlackMessageEdited>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackMessageEdited {
    pub user: SlackUserId,
    pub ts: SlackTs,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum SlackMessageResponseType {
    #[serde(rename = "in_channel")]
    InChannel,
    #[serde(rename = "ephemeral")]
    Ephemeral,
}

// This model is not well typed since Slack message attachments are deprecated
// Please avoid using this if you can
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackMessageAttachment {
    pub id: Option<i64>,
    pub color: Option<String>,
    pub fallback: Option<String>,
    pub title: Option<String>,
    pub fields: Option<Vec<SlackMessageAttachmentFieldObject>>,
    pub mrkdwn_in: Option<Vec<String>>,
}

// This model is not well typed since Slack message attachments are deprecated
// Please avoid using this if you can
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackMessageAttachmentFieldObject {
    pub title: Option<String>,
    pub value: Option<String>,
    pub short: Option<bool>,
}
