use crate::blocks::*;
use crate::common::*;
use crate::events::SlackMessageEventType;
use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackMessageOrigin {
    pub ts: SlackTs,
    pub channel: Option<SlackChannelId>,
    pub channel_type: Option<SlackChannelType>,
    pub thread_ts: Option<SlackTs>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackMessageContent {
    pub text: Option<String>,
    pub blocks: Option<Vec<SlackBlock>>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackMessage {
    #[serde(flatten)]
    pub origin: SlackMessageOrigin,
    #[serde(flatten)]
    pub content: SlackMessageContent,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackHistoryMessage {
    #[serde(flatten)]
    pub origin: SlackMessageOrigin,
    #[serde(flatten)]
    pub content: SlackMessageContent,
    pub subtype: Option<SlackMessageEventType>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum SlackMessageResponseType {
    #[serde(rename = "in_channel")]
    InChannel,
    #[serde(rename = "ephemeral")]
    Ephemeral,
}
