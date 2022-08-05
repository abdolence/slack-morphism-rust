use std::collections::HashMap;

use rsb_derive::Builder;
use serde::{Deserialize, Serialize};

use crate::blocks::SlackView;

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
}
