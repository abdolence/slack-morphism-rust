use crate::common::*;

use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

pub trait HasChannelInfo {
    fn get_channel_id(&self) -> &SlackChannelId;
}

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
    pub priority: Option<SlackChannelPriority>,
    pub num_members: Option<u64>,
    pub locale: Option<SlackLocale>,
    #[serde(flatten)]
    pub flags: SlackChannelFlags,
    #[serde(flatten)]
    pub last_state: SlackChannelCurrentState,
}

impl HasChannelInfo for SlackChannelInfo {
    fn get_channel_id(&self) -> &SlackChannelId {
        &self.id
    }
}

pub type SlackChannelTopicInfo = SlackChannelDetails;
pub type SlackChannelPurposeInfo = SlackChannelDetails;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackChannelDetails {
    pub value: String,
    pub creator: Option<SlackUserId>,
    pub last_set: Option<SlackDateTime>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackChannelFlags {
    pub is_channel: Option<bool>,
    pub is_group: Option<bool>,
    pub is_im: Option<bool>,
    pub is_archived: Option<bool>,
    pub is_general: Option<bool>,
    pub is_shared: Option<bool>,
    pub is_org_shared: Option<bool>,
    pub is_member: Option<bool>,
    pub is_private: Option<bool>,
    pub is_mpim: Option<bool>,
    pub is_user_deleted: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackChannelCurrentState {
    pub last_read: Option<SlackTs>,
    pub unread_count: Option<u64>,
    pub unread_count_display: Option<u64>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBasicChannelInfo {
    pub id: SlackChannelId,
    pub name: Option<String>,
}

impl HasChannelInfo for SlackBasicChannelInfo {
    fn get_channel_id(&self) -> &SlackChannelId {
        &self.id
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackChannelPriority(pub f64);
