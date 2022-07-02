use chrono::serde::ts_seconds;
use chrono::{DateTime, TimeZone, Utc};
use rsb_derive::Builder;
use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::hash::Hash;
use std::*;
use url::Url;

mod user;
pub use user::*;
mod team;
pub use team::*;
mod channel;
pub use channel::*;

mod bot;
pub use bot::*;

mod icon;
pub use icon::*;

mod formatters;
pub use formatters::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackTs(pub String);

impl SlackTs {
    pub fn to_date_time(&self) -> Result<DateTime<Utc>, num::ParseIntError> {
        let parts: Vec<&str> = self.value().split('.').collect();
        let ts_int: i64 = parts[0].parse()?;
        Ok(Utc.timestamp_millis(ts_int * 1000))
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackScheduledMid(pub String);

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackTeamId(pub String);

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackAppId(pub String);

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackChannelId(pub String);

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackClientMessageId(pub String);

impl SlackTextFormat for SlackChannelId {
    fn to_slack_format(&self) -> String {
        format!("<#{}>", self.value())
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackChannelType(pub String);

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackConversationId(pub String);

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackActionId(pub String);

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackActionType(pub String);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackUserId(pub String);

impl SlackTextFormat for SlackUserId {
    fn to_slack_format(&self) -> String {
        format!("<@{}>", self.value())
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackBotId(pub String);

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackDateTime(#[serde(with = "ts_seconds")] pub DateTime<Utc>);

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackLocale(pub String);

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackCursorId(pub String);

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackColor(pub String);

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackCallbackId(pub String);

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackTriggerId(pub String);

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackViewId(pub String);

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackCommandId(pub String);

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackClientId(pub String);

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackClientSecret(pub String);

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackVerificationToken(pub String);

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackSigningSecret(pub String);

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct EmailAddress(pub String);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackResponseMetadata {
    #[serde(with = "serde_with::rust::string_empty_as_none")]
    pub next_cursor: Option<SlackCursorId>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum SlackConversationType {
    #[serde(rename = "im")]
    Im,
    #[serde(rename = "mpim")]
    Mpim,
    #[serde(rename = "private_channel")]
    Private,
    #[serde(rename = "public_channel")]
    Public,
}

impl ToString for SlackConversationType {
    fn to_string(&self) -> String {
        match self {
            SlackConversationType::Im => "im".into(),
            SlackConversationType::Mpim => "mpim".into(),
            SlackConversationType::Private => "private_channel".into(),
            SlackConversationType::Public => "public_channel".into(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackResponseUrl(pub Url);

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackWebSocketsUrl(pub Url);

impl SlackWebSocketsUrl {
    pub fn to_debug_url(&self) -> Self {
        Self(Url::parse(format!("{}&debug_reconnects=true", self.value()).as_str()).unwrap())
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackTeamUrl(pub Url);

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackUnfurlId(pub String);

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackMimeType(pub String);

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackEmoji(pub String);

impl SlackEmoji {
    pub const SMILE: &'static str = ":smile:";
    pub const SPEECH_BALLOON: &'static str = ":speech_balloon:";
    pub const HEAVY_CHECK_MARK: &'static str = ":heavy_check_mark:";
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum SlackShortcutType {
    #[serde(rename = "message")]
    Message,
    #[serde(rename = "global")]
    Global,
}
