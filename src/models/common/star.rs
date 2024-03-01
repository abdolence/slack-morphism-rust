use crate::*;

use rsb_derive::Builder;
use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
#[allow(clippy::large_enum_variant)]
pub enum SlackStarsItem {
    Message(SlackStarsItemMessage),
    File(SlackStarsItemFile),
    #[serde(rename = "file_comment")]
    FileComment(SlackStarsItemFileComment),
    Channel(SlackStarsItemChannel),
    Im(SlackStarsItemIm),
    Group(SlackStarsItemGroup),
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackStarsItemMessage {
    pub message: SlackHistoryMessage,
    pub channel: SlackChannelId,
    pub date_create: SlackDateTime,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackStarsItemFile {
    pub file: SlackFile,
    pub channel: SlackChannelId,
    pub date_create: SlackDateTime,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackStarsItemFileComment {
    pub file: SlackFile,
    pub comment: String,
    pub channel: SlackChannelId,
    pub date_create: SlackDateTime,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackStarsItemChannel {
    pub channel: SlackChannelId,
    pub date_create: SlackDateTime,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackStarsItemIm {
    pub channel: SlackChannelId,
    pub date_create: SlackDateTime,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackStarsItemGroup {
    pub group: SlackChannelId,
    pub date_create: SlackDateTime,
}
