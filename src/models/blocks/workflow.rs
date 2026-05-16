use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use url::Url;

use crate::SlackActionId;
use super::kit::{SlackAccessibilityLabel, SlackBlockButtonStyle, SlackBlockPlainTextOnly};

/**
 * https://docs.slack.dev/reference/block-kit/composition-objects/workflow-object
 */
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackWorkflowTriggerInputParameter {
    pub name: String,
    pub value: String,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackWorkflowTrigger {
    pub url: Url,
    pub customizable_input_parameters: Option<Vec<SlackWorkflowTriggerInputParameter>>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackWorkflow {
    pub trigger: SlackWorkflowTrigger,
}

/**
 * https://docs.slack.dev/reference/block-kit/block-elements/workflow-button-element
 */
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockWorkflowButtonElement {
    pub action_id: SlackActionId,
    pub text: SlackBlockPlainTextOnly,
    pub workflow: SlackWorkflow,
    pub style: Option<SlackBlockButtonStyle>,
    pub accessibility_label: Option<SlackAccessibilityLabel>,
}
