use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::events::{
    SlackCommandEvent, SlackCommandEventResponse, SlackInteractionEvent, SlackPushEventCallback,
};
use crate::*;
use rvstruct::*;

#[allow(clippy::large_enum_variant)]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SlackSocketModeEvent {
    #[serde(rename = "hello")]
    Hello(SlackSocketModeHelloEvent),
    #[serde(rename = "disconnect")]
    Disconnect(SlackSocketModeDisconnectEvent),
    #[serde(rename = "interactive")]
    Interactive(SlackSocketModeInteractiveEvent),
    #[serde(rename = "events_api")]
    EventsApi(SlackSocketModeEventsApiEvent),
    #[serde(rename = "slash_commands")]
    SlashCommands(SlackSocketModeCommandEvent),
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
    pub build_number: Option<u64>,
    pub approximate_connection_time: Option<u64>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackSocketModeDisconnectEvent {
    pub reason: String,
    pub debug_info: SlackSocketModeDebugInfo,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackSocketModeEnvelopeId(pub String);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackSocketModeEventEnvelopeParams {
    pub envelope_id: SlackSocketModeEnvelopeId,
    pub accepts_response_payload: bool,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackSocketModeEventCommonAcknowledge {
    pub envelope_id: SlackSocketModeEnvelopeId,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackSocketModeInteractiveEvent {
    #[serde(flatten)]
    pub envelope_params: SlackSocketModeEventEnvelopeParams,
    pub payload: SlackInteractionEvent,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackSocketModeEventsApiEvent {
    #[serde(flatten)]
    pub envelope_params: SlackSocketModeEventEnvelopeParams,
    pub payload: SlackPushEventCallback,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackSocketModeCommandEvent {
    #[serde(flatten)]
    pub envelope_params: SlackSocketModeEventEnvelopeParams,
    pub payload: SlackCommandEvent,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackSocketModeCommandEventAck {
    #[serde(flatten)]
    pub envelope_ack_params: SlackSocketModeEventCommonAcknowledge,
    pub payload: Option<SlackCommandEventResponse>,
}
