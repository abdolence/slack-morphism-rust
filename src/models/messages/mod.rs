use crate::blocks::*;
use crate::events::SlackMessageEventType;
use crate::*;
use rsb_derive::Builder;
use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use url::Url;

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
pub struct SlackMessageMetadata {
    pub event_type: String,
    pub event_payload: Option<std::collections::HashMap<String, String>>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackMessageContent {
    pub text: Option<String>,
    /// Standard markdown rendered by Slack; not combinable with `text`/`blocks`; ≤ 12 000 chars.
    /// https://docs.slack.dev/messaging/formatting-message-text#markdown-text
    ///
    /// Write-only: Slack never returns it on received messages; it appears in inbound
    /// message models only because the content struct is shared.
    pub markdown_text: Option<String>,
    pub blocks: Option<Vec<SlackBlock>>,
    pub attachments: Option<Vec<SlackMessageAttachment>>,
    pub upload: Option<bool>,
    pub files: Option<Vec<SlackFile>>,
    pub reactions: Option<Vec<SlackReaction>>,
    pub metadata: Option<SlackMessageMetadata>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackMessageSender {
    pub user: Option<SlackUserId>,
    pub bot_id: Option<SlackBotId>,
    pub username: Option<String>,
    pub display_as_bot: Option<bool>,
    pub user_profile: Option<SlackUserProfile>,
    pub bot_profile: Option<SlackBotInfo>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackParentMessageParams {
    pub reply_count: Option<usize>,
    pub reply_users_count: Option<usize>,
    pub latest_reply: Option<SlackTs>,
    pub reply_users: Option<Vec<SlackUserId>>,
    pub subscribed: Option<bool>,
    pub last_read: Option<SlackTs>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackMessage {
    #[serde(flatten)]
    pub origin: SlackMessageOrigin,
    #[serde(flatten)]
    pub content: SlackMessageContent,
    #[serde(flatten)]
    pub sender: SlackMessageSender,
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
    pub edited: Option<SlackMessageEdited>,
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

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackMessageAttachmentId(i64);

// This model is not well typed since Slack message attachments are deprecated
// Please avoid using this if you can
//
// Forwarded (`is_share`) and unfurled (`is_msg_unfurl`) messages still arrive as
// attachments, carrying the original's author, channel, ts and permalink.
// Link fields an app can set on a legacy attachment stay `String`, so one bad
// URL from a third-party app can't fail the whole message; `from_url` and
// `original_url` are Slack-generated and typed as `Url`.
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackMessageAttachment {
    pub id: Option<SlackMessageAttachmentId>,
    pub color: Option<String>,
    pub fallback: Option<String>,
    pub title: Option<String>,
    pub title_link: Option<String>,
    pub pretext: Option<String>,
    pub fields: Option<Vec<SlackMessageAttachmentFieldObject>>,
    pub mrkdwn_in: Option<Vec<String>>,
    pub text: Option<String>,
    pub blocks: Option<Vec<SlackBlock>>,
    pub is_share: Option<bool>,
    pub is_msg_unfurl: Option<bool>,
    pub author_name: Option<String>,
    pub author_id: Option<SlackUserId>,
    pub author_subname: Option<String>,
    pub author_link: Option<String>,
    pub author_icon: Option<String>,
    pub channel_id: Option<SlackChannelId>,
    pub channel_name: Option<String>,
    pub channel_team: Option<SlackTeamId>,
    /// The original message's ts on shares/unfurls; legacy app attachments send an
    /// integer Unix time here, which is kept as its decimal string.
    #[serde(default, deserialize_with = "deserialize_attachment_ts")]
    pub ts: Option<SlackTs>,
    pub from_url: Option<Url>,
    pub original_url: Option<Url>,
    pub footer: Option<String>,
    pub footer_icon: Option<String>,
    pub service_name: Option<String>,
    pub message_blocks: Option<Vec<SlackMessageAttachmentMessageBlock>>,
    pub files: Option<Vec<SlackFile>>,
}

fn deserialize_attachment_ts<'de, D>(deserializer: D) -> Result<Option<SlackTs>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrNumber {
        String(String),
        Number(serde_json::Number),
    }
    Ok(
        Option::<StringOrNumber>::deserialize(deserializer)?.map(|v| match v {
            StringOrNumber::String(s) => SlackTs(s),
            StringOrNumber::Number(n) => SlackTs(n.to_string()),
        }),
    )
}

/// One `message_blocks` entry of a shared/unfurled message: where the original lives
/// and its Block Kit content.
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackMessageAttachmentMessageBlock {
    pub team: Option<SlackTeamId>,
    pub channel: Option<SlackChannelId>,
    pub ts: Option<SlackTs>,
    pub message: Option<SlackMessageAttachmentMessageBlockMessage>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackMessageAttachmentMessageBlockMessage {
    pub blocks: Option<Vec<SlackBlock>>,
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
