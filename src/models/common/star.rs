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
    message: SlackHistoryMessage,
    channel: SlackChannelId,
    date_create: SlackDateTime,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackStarsItemFile {
    file: SlackFile,
    channel: SlackChannelId,
    date_create: SlackDateTime,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackStarsItemFileComment {
    file: SlackFile,
    comment: String,
    channel: SlackChannelId,
    date_create: SlackDateTime,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackStarsItemChannel {
    channel: SlackChannelId,
    date_create: SlackDateTime,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackStarsItemIm {
    channel: SlackChannelId,
    date_create: SlackDateTime,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackStarsItemGroup {
    group: SlackChannelId,
    date_create: SlackDateTime,
}
