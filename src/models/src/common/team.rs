use crate::common::*;

use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackTeamInfo {
    pub id: SlackTeamId,
    pub name: Option<String>,
    pub domain: Option<String>,
    pub email_domain: Option<String>,
    pub icon: Option<SlackIcon>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBasicTeamInfo {
    pub id: SlackTeamId,
    pub name: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackTeamProfile {
    pub fields: Vec<SlackTeamProfileField>,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackTeamProfileFieldId(pub String);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackTeamProfileField {
    id: SlackTeamProfileFieldId,
    ordering: i64,
    label: String,
    hint: Option<String>,
    #[serde(rename = "type")]
    field_type: Option<String>,
    possible_values: Option<Vec<String>>,
    options: Option<serde_json::Value>,
}
