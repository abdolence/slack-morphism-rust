use rsb_derive::Builder;
use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{SlackApiTokenScope, SlackTeamId};

#[derive(Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackApiTokenValue(pub String);

impl std::fmt::Debug for SlackApiTokenValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SlackApiTokenValue(len:{})", self.value().len())
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize)]
pub enum SlackApiTokenType {
    #[serde(rename = "bot")]
    Bot,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "app")]
    App,
}

#[allow(clippy::to_string_trait_impl)]
impl ToString for SlackApiTokenType {
    fn to_string(&self) -> String {
        match self {
            SlackApiTokenType::Bot => "bot".into(),
            SlackApiTokenType::User => "user".into(),
            SlackApiTokenType::App => "app".into(),
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiToken {
    pub token_value: SlackApiTokenValue,
    pub team_id: Option<SlackTeamId>,
    pub scope: Option<SlackApiTokenScope>,
    pub token_type: Option<SlackApiTokenType>,
}
