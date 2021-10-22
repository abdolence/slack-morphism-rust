use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::*;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackCommandEvent {
    pub team_id: SlackTeamId,
    pub channel_id: SlackChannelId,
    pub user_id: SlackUserId,
    pub command: SlackCommandId,
    pub text: Option<String>,
    pub response_url: SlackResponseUrl,
    pub trigger_id: SlackTriggerId,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackCommandEventResponse {
    #[serde(flatten)]
    pub content: SlackMessageContent,
    pub response_type: Option<SlackMessageResponseType>,
}
