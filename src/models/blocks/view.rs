use crate::blocks::kit::SlackBlock;
use crate::blocks::{SlackBlockId, SlackBlockPlainText, SlackBlockPlainTextOnly};
use crate::SlackCallbackId;
use crate::*;
use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SlackView {
    #[serde(rename = "home")]
    Home(SlackHomeView),
    #[serde(rename = "modal")]
    Modal(SlackModalView),
}

#[serde_as]
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackHomeView {
    pub blocks: Vec<SlackBlock>,
    #[serde(default)]
    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    pub private_metadata: Option<String>,
    #[serde(default)]
    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    pub callback_id: Option<SlackCallbackId>,
    #[serde(default)]
    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    pub external_id: Option<String>,
}

#[serde_as]
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackModalView {
    pub title: SlackBlockPlainTextOnly,
    pub blocks: Vec<SlackBlock>,
    pub close: Option<SlackBlockPlainTextOnly>,
    pub submit: Option<SlackBlockPlainTextOnly>,
    #[serde(default)]
    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    pub private_metadata: Option<String>,
    #[serde(default)]
    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    pub callback_id: Option<SlackCallbackId>,
    pub clear_on_close: Option<bool>,
    pub notify_on_close: Option<bool>,
    pub hash: Option<String>,
    #[serde(default)]
    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    pub external_id: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackStatefulView {
    #[serde(flatten)]
    pub state_params: SlackStatefulStateParams,
    #[serde(flatten)]
    pub view: SlackView,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackStatefulStateParams {
    pub id: SlackViewId,
    pub team_id: SlackTeamId,
    pub state: Option<SlackViewState>,
    pub hash: String,
    pub previous_view_id: Option<SlackViewId>,
    pub root_view_id: Option<SlackViewId>,
    pub app_id: Option<SlackAppId>,
    pub bot_id: Option<SlackBotId>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackViewState {
    pub values: HashMap<SlackBlockId, HashMap<SlackActionId, SlackViewStateValue>>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackViewStateValue {
    #[serde(rename = "type")]
    pub action_type: SlackActionType,
    pub value: Option<String>,
    pub selected_date: Option<String>,
    pub selected_time: Option<String>,
    pub selected_date_time: Option<SlackDateTime>,
    pub selected_conversation: Option<SlackConversationId>,
    pub selected_channel: Option<SlackChannelId>,
    pub selected_user: Option<SlackUserId>,
    pub selected_option: Option<SlackViewStateValueSelectedOption>,
    pub selected_conversations: Option<Vec<SlackConversationId>>,
    pub selected_users: Option<Vec<SlackUserId>>,
    pub selected_options: Option<Vec<SlackViewStateValueSelectedOption>>,
}

pub type SlackActionState = SlackViewState;
pub type SlackActionStateValue = SlackViewStateValue;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackViewStateValueSelectedOption {
    pub text: SlackBlockPlainText,
    pub value: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "response_action")]
pub enum SlackViewSubmissionResponse {
    #[serde(rename = "clear")]
    Clear(SlackViewSubmissionClearResponse),
    #[serde(rename = "update")]
    Update(SlackViewSubmissionUpdateResponse),
    #[serde(rename = "push")]
    Push(SlackViewSubmissionPushResponse),
    #[serde(rename = "errors")]
    Errors(SlackViewSubmissionErrorsResponse),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackViewSubmissionClearResponse {}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackViewSubmissionUpdateResponse {
    pub view: SlackView,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackViewSubmissionPushResponse {
    pub view: SlackView,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackViewSubmissionErrorsResponse {
    pub errors: HashMap<String, String>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_slack_view_submission_clear_response_serialization() {
        let output = serde_json::to_string(&SlackViewSubmissionResponse::Clear(
            SlackViewSubmissionClearResponse::new(),
        ))
        .unwrap();
        assert_eq!(output, r#"{"response_action":"clear"}"#);
    }

    #[test]
    fn test_slack_api_apps_manifest_create_request() {
        let payload = include_str!("./fixtures/slack_home_view.json");
        let model: SlackHomeView = serde_json::from_str(payload).unwrap();
        assert!(model.private_metadata.is_none());
        assert_eq!(
            model.callback_id,
            Some(SlackCallbackId::from("test-callback-id"))
        );
    }
}
