use rsb_derive::Builder;
use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none};
use std::hash::Hash;
use std::*;
use url::Url;

pub(crate) mod datetime;

pub use datetime::*;

use datetime::unix_seconds;

mod user;

pub use user::*;

mod team;

pub use team::*;

mod channel;

pub use channel::*;

mod pin;

pub use pin::*;

mod reaction;

pub use reaction::*;

mod star;

pub use star::*;

mod bot;

pub use bot::*;

mod icon;

pub use icon::*;

mod formatters;

pub use formatters::*;

mod emoji;

pub use emoji::*;
mod assistant;
pub use assistant::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackTs(pub String);

impl SlackTs {
    /// Converts a Slack timestamp (`<seconds>.<microseconds>`) to an instant.
    ///
    /// The fractional part is optional and is interpreted as microseconds,
    /// padded or truncated to six digits.
    pub fn to_date_time_opt(&self) -> Option<SlackUtcDateTime> {
        let value = self.value();
        let (seconds_part, micros_part) = match value.split_once('.') {
            Some((seconds, micros)) => (seconds, Some(micros)),
            None => (value.as_str(), None),
        };

        let seconds: i64 = seconds_part.parse().ok()?;

        let micros: u32 = match micros_part {
            Some(micros_part) => {
                let mut digits: String = micros_part.chars().take(6).collect();
                while digits.len() < 6 {
                    digits.push('0');
                }
                digits.parse().ok()?
            }
            None => 0,
        };

        datetime::from_unix_seconds_micros(seconds, micros)
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

impl SlackTextFormat for SlackUserGroupId {
    fn to_slack_format(&self) -> String {
        format!("<!subteam^{}>", self.value())
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackBotId(pub String);

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackDateTime(#[serde(with = "unix_seconds")] pub SlackUtcDateTime);

impl SlackDateTime {
    pub fn now() -> Self {
        Self(datetime::now())
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackDate(pub String);

impl SlackDate {
    pub fn to_naive_date(&self) -> Option<SlackCivilDate> {
        datetime::parse_civil_date(self.value())
    }
}

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

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct PhoneNumber(pub String);

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

#[allow(clippy::to_string_trait_impl)]
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

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackEnterpriseId(pub String);

/// This type is needed since Slack allowes invalid URLs in some places like Rich sections
/// and we still need to read them on the client side, so we store it as a string
/// but provide convinent messages to convert to Url if possible.
#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackRelaxedUrl(pub String);

impl SlackRelaxedUrl {
    pub fn to_url(&self) -> Result<Url, url::ParseError> {
        self.try_into()
    }
}

impl TryFrom<&SlackRelaxedUrl> for Url {
    type Error = url::ParseError;

    fn try_from(relaxed_url: &SlackRelaxedUrl) -> Result<Self, Self::Error> {
        Url::parse(relaxed_url.value())
    }
}

impl TryFrom<SlackRelaxedUrl> for Url {
    type Error = url::ParseError;

    fn try_from(relaxed_url: SlackRelaxedUrl) -> Result<Self, Self::Error> {
        Url::parse(relaxed_url.value())
    }
}

impl From<Url> for SlackRelaxedUrl {
    fn from(url: Url) -> Self {
        SlackRelaxedUrl(url.to_string())
    }
}

impl From<&Url> for SlackRelaxedUrl {
    fn from(url: &Url) -> Self {
        SlackRelaxedUrl(url.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;

    fn test_date_time() -> SlackUtcDateTime {
        "2020-01-01T00:42:42Z".parse::<SlackUtcDateTime>().unwrap()
    }

    #[test]
    fn test_slack_date_time() {
        let dt = SlackDateTime(test_date_time());
        let json = serde_json::to_value(&dt).unwrap();
        assert_eq!(json.as_u64().unwrap(), 1577839362);

        let parsed: SlackDateTime = serde_json::from_value(json!(1577839362)).unwrap();
        assert_eq!(parsed, dt);
        assert_eq!(serde_json::from_value::<SlackDateTime>(json).unwrap(), dt);
    }

    #[test]
    fn test_slack_ts_to_date_time_with_micros() {
        let ts = SlackTs("1577839362.000400".to_string());
        let dt = ts.to_date_time_opt().unwrap();

        assert_eq!(datetime::unix_seconds(&dt), 1577839362);
        assert_eq!(
            Some(dt),
            datetime::from_unix_seconds_micros(1577839362, 400)
        );
        assert_ne!(Some(dt), datetime::from_unix_seconds_micros(1577839362, 0));
    }

    #[test]
    fn test_slack_ts_to_date_time_without_fraction() {
        let ts = SlackTs("1577839362".to_string());
        let dt = ts.to_date_time_opt().unwrap();

        assert_eq!(datetime::unix_seconds(&dt), 1577839362);
        assert_eq!(Some(dt), datetime::from_unix_seconds_micros(1577839362, 0));
    }

    #[test]
    fn test_slack_ts_to_date_time_invalid() {
        assert_eq!(SlackTs("garbage".to_string()).to_date_time_opt(), None);
    }

    #[test]
    fn test_slack_date_to_naive_date() {
        assert!(SlackDate("2020-01-01".to_string())
            .to_naive_date()
            .is_some());
        assert_eq!(SlackDate("2020-13-01".to_string()).to_naive_date(), None);
        assert_eq!(SlackDate("garbage".to_string()).to_naive_date(), None);
    }
}
