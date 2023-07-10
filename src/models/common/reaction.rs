use crate::*;

use rsb_derive::Builder;
use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackReaction {
    pub name: SlackReactionName,
    pub count: usize,
    pub users: Vec<SlackUserId>,
}

#[skip_serializing_none]
#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackReactionName(pub String);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
#[allow(clippy::large_enum_variant)]
pub enum SlackReactionsItem {
    Message(SlackHistoryMessage),
    File(SlackFile),
}
