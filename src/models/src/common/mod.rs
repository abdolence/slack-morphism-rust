use rvs_derive::ValueStruct;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackTs(pub String);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackChannelId(pub String);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackChannelType(pub String);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackConversationId(pub String);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackActionId(pub String);
