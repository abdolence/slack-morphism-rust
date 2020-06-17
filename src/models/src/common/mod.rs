use chrono::serde::ts_seconds;
use chrono::{DateTime, TimeZone, Utc};
use rsb_derive::Builder;
use rvs_derive::ValueStruct;
use serde::de::{MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::skip_serializing_none;
use std::*;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackTs(pub String);

impl SlackTs {
    pub fn to_date_time(&self) -> Result<DateTime<Utc>, num::ParseIntError> {
        let parts: Vec<&str> = self.value().split('.').collect();
        let ts_int: i64 = parts[0].parse()?;
        Ok(Utc.timestamp_millis(ts_int * 1000))
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackTeamId(pub String);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackAppId(pub String);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackChannelId(pub String);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackChannelType(pub String);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackConversationId(pub String);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackActionId(pub String);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackEventId(pub String);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackUserId(pub String);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackDateTime(#[serde(with = "ts_seconds")] pub DateTime<Utc>);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackLocale(pub String);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackCursorId(pub String);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackColor(pub String);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackCallbackId(pub String);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackResponseMetadata {
    #[serde(with = "serde_with::rust::string_empty_as_none")]
    pub next_cursor: Option<SlackCursorId>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackUser {
    pub id: SlackUserId,
    pub team_id: SlackTeamId,
    pub name: Option<String>,
    pub locale: Option<SlackLocale>,
    pub profile: Option<SlackUserProfile>,
    #[serde(flatten)]
    pub flags: SlackUserFlags,
    pub tz: Option<String>,
    pub tz_label: Option<String>,
    pub tz_offset: Option<i16>,
    pub updated: Option<SlackDateTime>,
    pub deleted: Option<bool>,
    pub color: Option<SlackColor>,
    pub real_name: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackUserProfile {
    pub id: Option<SlackUserId>,
    pub display_name: Option<String>,
    pub real_name: Option<String>,
    pub real_name_normalized: Option<String>,
    pub avatar_hash: Option<String>,
    pub status_text: Option<String>,
    pub status_expiration: Option<SlackDateTime>,
    pub display_name_normalized: Option<String>,
    pub email: Option<String>,
    #[serde(flatten)]
    pub icon: Option<SlackIcon>,
    pub team: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackUserFlags {
    pub is_admin: Option<bool>,
    pub is_app_user: Option<bool>,
    pub is_bot: Option<bool>,
    pub is_invited_user: Option<bool>,
    pub is_owner: Option<bool>,
    pub is_primary_owner: Option<bool>,
    pub is_restricted: Option<bool>,
    pub is_stranger: Option<bool>,
    pub is_ultra_restricted: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackTeamInfo {
    pub id: String,
    pub name: Option<String>,
    pub domain: Option<String>,
    pub email_domain: Option<String>,
    pub icon: Option<SlackIcon>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackIcon {
    pub image_original: Option<String>,
    pub image_default: Option<bool>,
    #[serde(flatten)]
    pub images: Option<SlackIconImages>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Builder)]
pub struct SlackIconImages {
    pub resolutions: Vec<(u32, String)>,
}

const SLACK_ICON_JSON_PREFIX: &'static str = "image_";

impl Serialize for SlackIconImages {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if self.resolutions.is_empty() {
            serializer.serialize_none()
        } else {
            let mut res_map = serializer.serialize_map(Some(self.resolutions.len()))?;
            for (res, link) in &self.resolutions {
                let key: String = format!("{}{}", SLACK_ICON_JSON_PREFIX, res);
                res_map.serialize_entry(&key, link)?;
            }
            res_map.end()
        }
    }
}

struct SlackIconImagesVisitor {}

impl SlackIconImagesVisitor {
    fn new() -> Self {
        Self {}
    }
}

impl<'de> Visitor<'de> for SlackIconImagesVisitor {
    type Value = SlackIconImages;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a json contains images_ for icon")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut resolutions: Vec<(u32, String)> =
            Vec::with_capacity(access.size_hint().unwrap_or(0));

        while let Some(key) = access.next_key::<String>()? {
            if key.starts_with(SLACK_ICON_JSON_PREFIX) {
                let parsed_key: Vec<_> = key.split("_").collect();
                if parsed_key.len() == 2 {
                    let resolution: u32 = parsed_key[1].parse().unwrap();
                    let value: String = access.next_value()?;
                    resolutions.push((resolution, value).clone());
                }
            }
        }

        Ok(SlackIconImages::new(resolutions))
    }
}

impl<'de> Deserialize<'de> for SlackIconImages {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(SlackIconImagesVisitor::new())
    }
}
