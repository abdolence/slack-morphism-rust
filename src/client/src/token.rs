use rsb_derive::Builder;
use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};
use slack_morphism_models::SlackTeamId;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackApiTokenValue(pub String);

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackApiTokenScope(pub String);

#[derive(Debug, PartialEq, Clone, Builder)]
pub struct SlackApiToken {
    pub token_value: SlackApiTokenValue,
    pub team_id: Option<SlackTeamId>,
    pub scope: Option<SlackApiTokenScope>,
}
