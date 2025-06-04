use crate::{SlackChannelId, SlackEnterpriseId, SlackTeamId, SlackTs, SlackUserId};
use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackAssistantThread {
    pub user_id: SlackUserId,
    pub context: SlackAssistantThreadContext,
    pub channel_id: SlackChannelId,
    pub thread_ts: SlackTs,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackAssistantThreadContext {
    pub channel_id: Option<SlackChannelId>,
    pub team_id: Option<SlackTeamId>,
    pub enterprise_id: Option<SlackEnterpriseId>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackAssistantPrompt {
    pub title: String,
    pub message: String,
}
