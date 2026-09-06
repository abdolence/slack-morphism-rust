use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::events::{
    SlackCommandEvent, SlackCommandEventResponse, SlackInteractionEvent, SlackInteractionResponse,
    SlackPushEventCallback,
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

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackSocketModeInteractiveEventAck {
    #[serde(flatten)]
    pub envelope_ack_params: SlackSocketModeEventCommonAcknowledge,
    pub payload: Option<SlackInteractionResponse>,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::blocks::{SlackBlockChoiceItem, SlackBlockPlainTextOnly};
    use crate::events::{
        SlackBlockSuggestionOptions, SlackBlockSuggestionResponse, SlackInteractionResponse,
    };

    #[test]
    fn test_socket_mode_interactive_block_suggestion_event_deserialization() {
        let payload = format!(
            r#"{{"type":"interactive","envelope_id":"57d6a792-4d35-4d0b-b6aa-3361493e1caf","accepts_response_payload":true,"payload":{}}}"#,
            include_str!("../events/fixtures/interaction_block_suggestion_view.json")
        );

        let event: SlackSocketModeEvent = serde_json::from_str(&payload).unwrap();

        match event {
            SlackSocketModeEvent::Interactive(interactive) => {
                assert_eq!(
                    interactive.envelope_params.envelope_id,
                    SlackSocketModeEnvelopeId("57d6a792-4d35-4d0b-b6aa-3361493e1caf".into())
                );
                assert!(interactive.envelope_params.accepts_response_payload);
                assert!(matches!(
                    interactive.payload,
                    SlackInteractionEvent::BlockSuggestion(_)
                ));
            }
            other => panic!("Unexpected socket mode event: {:?}", other),
        }
    }

    #[test]
    fn test_socket_mode_interactive_ack_with_payload_serialization() {
        let ack =
            SlackSocketModeInteractiveEventAck::new(SlackSocketModeEventCommonAcknowledge::new(
                SlackSocketModeEnvelopeId("57d6a792-4d35-4d0b-b6aa-3361493e1caf".into()),
            ))
            .with_payload(SlackInteractionResponse::BlockSuggestion(
                SlackBlockSuggestionResponse::Options(SlackBlockSuggestionOptions::new(vec![
                    SlackBlockChoiceItem::new(
                        SlackBlockPlainTextOnly::from("Unexpected sentience"),
                        "AI-2323".to_string(),
                    ),
                ])),
            ));

        assert_eq!(
            serde_json::to_string(&ack).unwrap(),
            r#"{"envelope_id":"57d6a792-4d35-4d0b-b6aa-3361493e1caf","payload":{"options":[{"text":{"type":"plain_text","text":"Unexpected sentience"},"value":"AI-2323"}]}}"#
        );
    }

    #[test]
    fn test_socket_mode_interactive_ack_without_payload_serialization() {
        let ack =
            SlackSocketModeInteractiveEventAck::new(SlackSocketModeEventCommonAcknowledge::new(
                SlackSocketModeEnvelopeId("57d6a792-4d35-4d0b-b6aa-3361493e1caf".into()),
            ));

        assert_eq!(
            serde_json::to_string(&ack).unwrap(),
            r#"{"envelope_id":"57d6a792-4d35-4d0b-b6aa-3361493e1caf"}"#
        );
    }
}
