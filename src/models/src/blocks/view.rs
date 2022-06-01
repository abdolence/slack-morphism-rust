use crate::blocks::kit::SlackBlock;
use crate::blocks::{SlackBlockId, SlackBlockPlainTextOnly};
use crate::common::SlackCallbackId;
use crate::*;
use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SlackView {
    #[serde(rename = "home")]
    Home(SlackHomeView),
    #[serde(rename = "modal")]
    Modal(SlackModalView),
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackHomeView {
    pub blocks: Vec<SlackBlock>,
    #[serde(with = "serde_with::rust::string_empty_as_none")]
    pub private_metadata: Option<String>,
    #[serde(with = "serde_with::rust::string_empty_as_none")]
    pub callback_id: Option<SlackCallbackId>,
    #[serde(with = "serde_with::rust::string_empty_as_none")]
    pub external_id: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackModalView {
    pub title: SlackBlockPlainTextOnly,
    pub blocks: Vec<SlackBlock>,
    pub close: Option<SlackBlockPlainTextOnly>,
    pub submit: Option<SlackBlockPlainTextOnly>,
    #[serde(with = "serde_with::rust::string_empty_as_none")]
    pub private_metadata: Option<String>,
    #[serde(with = "serde_with::rust::string_empty_as_none")]
    pub callback_id: Option<SlackCallbackId>,
    pub clear_on_close: Option<bool>,
    pub notify_on_close: Option<bool>,
    pub hash: Option<String>,
    #[serde(with = "serde_with::rust::string_empty_as_none")]
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
    pub callback_id: Option<SlackCallbackId>,
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
}
