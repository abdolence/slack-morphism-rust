use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none};

use crate::blocks::*;
use crate::models::messages::*;
use crate::*;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SlackInteractionEvent {
    #[serde(rename = "block_actions")]
    BlockActions(SlackInteractionBlockActionsEvent),
    #[serde(rename = "block_suggestion")]
    BlockSuggestion(SlackInteractionBlockSuggestionEvent),
    #[serde(rename = "dialog_submission")]
    DialogSubmission(SlackInteractionDialogueSubmissionEvent),
    #[serde(rename = "message_action")]
    MessageAction(SlackInteractionMessageActionEvent),
    #[serde(rename = "shortcut")]
    Shortcut(SlackInteractionShortcutEvent),
    #[serde(rename = "view_submission")]
    ViewSubmission(SlackInteractionViewSubmissionEvent),
    #[serde(rename = "view_closed")]
    ViewClosed(SlackInteractionViewClosedEvent),
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackInteractionBlockActionsEvent {
    pub team: SlackBasicTeamInfo,
    pub user: Option<SlackBasicUserInfo>,
    pub api_app_id: SlackAppId,
    pub container: SlackInteractionActionContainer,
    pub trigger_id: SlackTriggerId,
    pub channel: Option<SlackBasicChannelInfo>,
    pub message: Option<SlackHistoryMessage>,
    pub view: Option<SlackView>,
    pub response_url: Option<SlackResponseUrl>,
    pub actions: Option<Vec<SlackInteractionActionInfo>>,
    pub state: Option<SlackActionState>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackInteractionBlockSuggestionEvent {
    pub team: SlackBasicTeamInfo,
    pub user: SlackBasicUserInfo,
    pub api_app_id: SlackAppId,
    pub block_id: SlackBlockId,
    pub action_id: SlackActionId,
    pub container: SlackInteractionActionContainer,
    pub channel: Option<SlackBasicChannelInfo>,
    pub view: Option<SlackView>,
    pub value: String,
    pub message: Option<SlackHistoryMessage>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SlackInteractionActionContainer {
    #[serde(rename = "message")]
    Message(SlackInteractionActionMessageContainer),
    #[serde(rename = "message_attachment")]
    MessageAttachment(SlackInteractionActionMessageAttachmentContainer),
    #[serde(rename = "view")]
    View(SlackInteractionActionViewContainer),
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackInteractionActionMessageContainer {
    pub message_ts: SlackTs,
    pub channel_id: Option<SlackChannelId>,
    pub is_ephemeral: Option<bool>,
    pub is_app_unfurl: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackInteractionActionMessageAttachmentContainer {
    pub message_ts: SlackTs,
    pub attachment_id: SlackMessageAttachmentId,
    pub channel_id: Option<SlackChannelId>,
    pub is_ephemeral: Option<bool>,
    pub is_app_unfurl: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackInteractionActionViewContainer {
    pub view_id: SlackViewId,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackInteractionActionInfo {
    #[serde(rename = "type")]
    pub action_type: SlackActionType,
    pub action_id: SlackActionId,
    pub block_id: Option<SlackBlockId>,
    pub text: Option<SlackBlockText>,
    pub value: Option<String>,
    pub selected_option: Option<SlackBlockChoiceItem<SlackBlockText>>,
    pub selected_options: Option<Vec<SlackBlockChoiceItem<SlackBlockText>>>,
    pub action_ts: Option<SlackTs>,
}

#[serde_as]
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackInteractionDialogueSubmissionEvent {
    pub team: SlackBasicTeamInfo,
    pub user: SlackBasicUserInfo,
    pub channel: Option<SlackBasicChannelInfo>,
    #[serde(default)]
    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    pub callback_id: Option<SlackCallbackId>,
    pub state: Option<String>,
    pub submission: HashMap<String, String>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackInteractionMessageActionEvent {
    pub team: SlackBasicTeamInfo,
    pub user: SlackBasicUserInfo,
    pub channel: Option<SlackBasicChannelInfo>,
    pub message: Option<SlackHistoryMessage>,
    pub callback_id: SlackCallbackId,
    pub trigger_id: SlackTriggerId,
    pub response_url: SlackResponseUrl,
    pub actions: Option<Vec<SlackInteractionActionInfo>>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackInteractionShortcutEvent {
    pub team: SlackBasicTeamInfo,
    pub user: SlackBasicUserInfo,
    pub callback_id: SlackCallbackId,
    pub trigger_id: SlackTriggerId,
    pub actions: Option<Vec<SlackInteractionActionInfo>>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackInteractionViewSubmissionEvent {
    pub team: SlackBasicTeamInfo,
    pub user: SlackBasicUserInfo,
    pub view: SlackStatefulView,
    pub trigger_id: Option<SlackTriggerId>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackInteractionViewClosedEvent {
    pub team: SlackBasicTeamInfo,
    pub user: SlackBasicUserInfo,
    pub view: SlackStatefulView,
    pub trigger_id: Option<SlackTriggerId>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SlackBlockSuggestionResponse {
    Options(SlackBlockSuggestionOptions),
    OptionGroups(SlackBlockSuggestionOptionGroups),
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockSuggestionOptions {
    pub options: Vec<SlackBlockChoiceItem<SlackBlockPlainTextOnly>>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockSuggestionOptionGroups {
    pub option_groups: Vec<SlackBlockOptionGroup<SlackBlockPlainTextOnly>>,
}

/// What an interaction handler hands back to Slack.
///
/// A single handler can answer every kind of [`SlackInteractionEvent`] with this:
/// options for `block_suggestion`, a `response_action` for `view_submission`,
/// and a plain acknowledgement for everything else.
///
/// [`SlackInteractionResponse::Empty`] has no wire representation: transports turn it
/// into an empty HTTP 200 or a bare Socket Mode acknowledgement, and serialising it
/// directly is an error rather than a silent `null`.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SlackInteractionResponse {
    #[serde(skip)]
    Empty,
    ViewSubmission(SlackViewSubmissionResponse),
    BlockSuggestion(SlackBlockSuggestionResponse),
}

impl From<()> for SlackInteractionResponse {
    fn from(_: ()) -> Self {
        SlackInteractionResponse::Empty
    }
}

impl From<SlackViewSubmissionResponse> for SlackInteractionResponse {
    fn from(response: SlackViewSubmissionResponse) -> Self {
        SlackInteractionResponse::ViewSubmission(response)
    }
}

impl From<SlackBlockSuggestionResponse> for SlackInteractionResponse {
    fn from(response: SlackBlockSuggestionResponse) -> Self {
        SlackInteractionResponse::BlockSuggestion(response)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_block_suggestion_view_event_deserialization() {
        let payload = include_str!("./fixtures/interaction_block_suggestion_view.json");
        let event: SlackInteractionEvent = serde_json::from_str(payload).unwrap();

        match event {
            SlackInteractionEvent::BlockSuggestion(ev) => {
                assert_eq!(ev.value, "sentien");
                assert_eq!(ev.action_id, "my-external-select-action".into());
                assert_eq!(ev.block_id, "my-external-select-block".into());
                assert!(matches!(
                    ev.container,
                    SlackInteractionActionContainer::View(_)
                ));
                assert!(ev.view.is_some());
                assert_eq!(ev.channel, None);
                assert_eq!(ev.message, None);
            }
            other => panic!("Unexpected interaction event: {:?}", other),
        }
    }

    #[test]
    fn test_block_suggestion_message_event_deserialization() {
        let payload = include_str!("./fixtures/interaction_block_suggestion_message.json");
        let event: SlackInteractionEvent = serde_json::from_str(payload).unwrap();

        match event {
            SlackInteractionEvent::BlockSuggestion(ev) => {
                assert_eq!(ev.value, "sentien");
                assert_eq!(ev.action_id, "my-external-select-action".into());
                assert_eq!(ev.block_id, "my-external-select-block".into());
                assert!(matches!(
                    ev.container,
                    SlackInteractionActionContainer::Message(_)
                ));
                assert_eq!(
                    ev.channel.map(|channel| channel.id),
                    Some("CXXXXXXXXXX".into())
                );
                assert!(ev.message.is_some());
                assert!(ev.view.is_none());
            }
            other => panic!("Unexpected interaction event: {:?}", other),
        }
    }

    #[test]
    fn test_block_suggestion_options_response_serialization() {
        let response =
            SlackBlockSuggestionResponse::Options(SlackBlockSuggestionOptions::new(vec![
                SlackBlockChoiceItem::new(
                    SlackBlockPlainTextOnly::from("Unexpected sentience"),
                    "AI-2323".to_string(),
                )
                .with_description(SlackBlockPlainTextOnly::from("Reported by the night shift")),
            ]));

        assert_eq!(
            serde_json::to_string(&response).unwrap(),
            r#"{"options":[{"text":{"type":"plain_text","text":"Unexpected sentience"},"value":"AI-2323","description":{"type":"plain_text","text":"Reported by the night shift"}}]}"#
        );
    }

    #[test]
    fn test_block_suggestion_option_groups_response_serialization() {
        let response = SlackBlockSuggestionResponse::OptionGroups(
            SlackBlockSuggestionOptionGroups::new(vec![SlackBlockOptionGroup::new(
                SlackBlockPlainTextOnly::from("Open"),
                vec![SlackBlockChoiceItem::new(
                    SlackBlockPlainTextOnly::from("Unexpected sentience"),
                    "AI-2323".to_string(),
                )],
            )]),
        );

        assert_eq!(
            serde_json::to_string(&response).unwrap(),
            r#"{"option_groups":[{"label":{"type":"plain_text","text":"Open"},"options":[{"text":{"type":"plain_text","text":"Unexpected sentience"},"value":"AI-2323"}]}]}"#
        );
    }

    #[test]
    fn test_interaction_response_from_unit_is_empty() {
        assert_eq!(
            SlackInteractionResponse::from(()),
            SlackInteractionResponse::Empty
        );
    }

    #[test]
    fn test_interaction_response_empty_is_not_serializable() {
        assert!(serde_json::to_string(&SlackInteractionResponse::Empty).is_err());
    }

    #[test]
    fn test_interaction_response_block_suggestion_serialization() {
        let inner = SlackBlockSuggestionResponse::Options(SlackBlockSuggestionOptions::new(vec![
            SlackBlockChoiceItem::new(
                SlackBlockPlainTextOnly::from("Unexpected sentience"),
                "AI-2323".to_string(),
            ),
        ]));

        assert_eq!(
            serde_json::to_string(&SlackInteractionResponse::from(inner.clone())).unwrap(),
            serde_json::to_string(&inner).unwrap()
        );
    }

    #[test]
    fn test_interaction_response_view_submission_serialization() {
        let inner = SlackViewSubmissionResponse::Clear(SlackViewSubmissionClearResponse::new());

        assert_eq!(
            serde_json::to_string(&SlackInteractionResponse::from(inner)).unwrap(),
            r#"{"response_action":"clear"}"#
        );
    }
}
