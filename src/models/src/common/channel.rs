use crate::common::*;

use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackChannelInfo {
    pub id: SlackChannelId,
    pub created: SlackDateTime,
    pub creator: Option<SlackUserId>,
    pub name_normalized: Option<String>,
    pub topic: Option<SlackChannelTopicInfo>,
    pub purpose: Option<SlackChannelPurposeInfo>,
    pub previous_names: Option<Vec<String>>,
    pub priority: Option<u64>,
    pub num_members: Option<u64>,
    pub locale: Option<SlackLocale>,
    #[serde(flatten)]
    pub flags: SlackChannelFlags,
    #[serde(flatten)]
    pub last_state: SlackChannelCurrentState
}


pub type SlackChannelTopicInfo = SlackChannelDetails;
pub type SlackChannelPurposeInfo = SlackChannelDetails;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackChannelDetails {
    pub value: String,
    pub creator: Option<String>,
    pub last_set: Option<SlackDateTime>
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackChannelFlags {
    is_channel: Option<bool>,
    is_group: Option<bool>,
    is_im: Option<bool>,
    is_archived: Option<bool>,
    is_general: Option<bool>,
    is_shared: Option<bool>,
    is_org_shared: Option<bool>,
    is_member: Option<bool>,
    is_private: Option<bool>,
    is_mpim: Option<bool>,
    is_user_deleted: Option<bool>
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackChannelCurrentState {
    last_read: Option<SlackTs>,
    unread_count: Option<u64>,
    unread_count_display: Option<u64>
}