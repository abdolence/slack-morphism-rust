use chrono::serde::ts_seconds;
use chrono::{DateTime, TimeZone, Utc};
use rsb_derive::Builder;
use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none};
use std::hash::Hash;
use std::*;
use url::Url;

mod user;
pub use user::*;
mod team;
pub use team::*;
mod channel;
pub use channel::*;
mod reaction;
pub use reaction::*;

mod bot;
pub use bot::*;

mod icon;
pub use icon::*;

mod formatters;
pub use formatters::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackTs(pub String);

impl SlackTs {
    #[deprecated(
        since = "1.3.5",
        note = "Use to_date_time_opt() with more accurate error handling and results from chrono::timestamp_millis_opt"
    )]
    pub fn to_date_time(&self) -> Result<DateTime<Utc>, num::ParseIntError> {
        let parts: Vec<&str> = self.value().split('.').collect();
        let ts_int: i64 = parts[0].parse()?;
        #[allow(deprecated)]
        Ok(Utc.timestamp_millis(ts_int * 1000))
    }

    pub fn to_date_time_opt(&self) -> Option<DateTime<Utc>> {
        let parts: Vec<&str> = self.value().split('.').collect();
        if let Ok(ts_int) = parts[0].parse::<i64>() {
            match Utc.timestamp_millis_opt(ts_int * 1000) {
                chrono::LocalResult::None => None,
                chrono::LocalResult::Single(result) => Some(result),
                chrono::LocalResult::Ambiguous(first, _) => Some(first),
            }
        } else {
            None
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackScheduledMid(pub String);

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackTeamId(pub String);

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackEnterpriseSubteamId(pub String);

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

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackUserGroupId(pub String);

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

#[derive(Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackClientSecret(pub String);

impl fmt::Debug for SlackClientSecret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SlackClientSecret(len:{})", self.value().len())
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackApiTokenScope(pub String);

#[derive(Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackVerificationToken(pub String);

impl fmt::Debug for SlackVerificationToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SlackVerificationToken(len:{})", self.value().len())
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackSigningSecret(pub String);

impl fmt::Debug for SlackSigningSecret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SlackSigningSecret(len:{})", self.value().len())
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct EmailAddress(pub String);

#[serde_as]
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackResponseMetadata {
    #[serde(default)]
    #[serde_as(as = "serde_with::NoneAsEmptyString")]
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

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackEventType(pub String);
