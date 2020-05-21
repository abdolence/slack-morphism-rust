use rvs_derive::ValueStruct;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, TimeZone};
use chrono::serde::ts_seconds;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackTs(pub String);

impl SlackTs {
    pub fn to_date_time(&self) -> Result<DateTime<Utc>,std::num::ParseIntError> {
        let parts : Vec<&str> = self.value().split('.').collect();
        let ts_int : i64 = parts[0].parse()?;
        Ok(Utc.timestamp_millis(ts_int))
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackChannelId(pub String);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackChannelType(pub String);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackConversationId(pub String);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackActionId(pub String);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackDateTime(#[serde(with = "ts_seconds")] pub DateTime<Utc>);
