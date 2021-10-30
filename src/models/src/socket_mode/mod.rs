use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::common::*;

#[allow(clippy::large_enum_variant)]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SlackSocketModeEvent {
    #[serde(rename = "hello")]
    Hello(SlackSocketModeHelloEvent),
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackSocketModeHelloEvent {
    pub connection_info: SlackSocketModeConnectionInfo,
    pub num_connections: u32,
    pub debug_info: SlackSocketModeDebugInfo,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackSocketModeConnectionInfo {
    pub app_id: SlackAppId,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackSocketModeDebugInfo {
    pub host: String,
    pub started: Option<String>,
    pub build_number: u64,
    pub approximate_connection_time: u64,
}
