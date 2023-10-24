use crate::*;

use crate::SlackUserId;
use rsb_derive::Builder;
use rvstruct::*;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none, DisplayFromStr};

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackUser {
    pub id: SlackUserId,
    pub team_id: Option<SlackTeamId>,
    pub teams: Option<Vec<SlackTeamId>>,
    pub name: Option<String>,
    pub locale: Option<SlackLocale>,
    pub profile: Option<SlackUserProfile>,
    #[serde(flatten)]
    pub flags: SlackUserFlags,
    pub tz: Option<String>,
    pub tz_label: Option<String>,
    pub tz_offset: Option<i32>,
    pub updated: Option<SlackDateTime>,
    pub deleted: Option<bool>,
    pub color: Option<SlackColor>,
    pub real_name: Option<String>,
    pub enterprise_user: Option<SlackEnterpriseUser>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackUserProfile {
    pub id: Option<SlackUserId>,
    pub display_name: Option<String>,
    pub real_name: Option<String>,
    pub real_name_normalized: Option<String>,
    pub avatar_hash: Option<SlackAvatarHash>,
    pub status_text: Option<String>,
    pub status_expiration: Option<SlackDateTime>,
    pub status_emoji: Option<SlackEmoji>,
    pub display_name_normalized: Option<String>,
    pub email: Option<EmailAddress>,
    #[serde(flatten)]
    pub icon: Option<SlackIcon>,
    pub team: Option<SlackTeamId>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackUserFlags {
    pub is_admin: Option<bool>,
    pub is_app_user: Option<bool>,
    pub is_bot: Option<bool>,
    pub is_invited_user: Option<bool>,
    pub is_owner: Option<bool>,
    pub is_primary_owner: Option<bool>,
    pub is_restricted: Option<bool>,
    pub is_stranger: Option<bool>,
    pub is_ultra_restricted: Option<bool>,
    pub has_2fa: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBasicUserInfo {
    pub id: SlackUserId,
    pub team_id: Option<SlackTeamId>,
    pub username: Option<String>,
    pub name: Option<String>,
}

#[serde_as]
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackUserGroup {
    pub id: SlackUserGroupId,
    pub team_id: SlackTeamId,
    pub enterprise_subteam_id: Option<SlackEnterpriseSubteamId>,
    pub is_usergroup: Option<bool>,
    pub is_subteam: Option<bool>,
    pub name: String,
    pub description: Option<String>,
    pub handle: String,
    pub is_external: bool,
    pub auto_provision: Option<bool>,
    pub date_create: SlackDateTime,
    pub date_update: Option<SlackDateTime>,
    pub date_delete: Option<SlackDateTime>,
    pub auto_type: Option<SlackAutoType>,
    pub created_by: SlackUserId,
    pub updated_by: Option<SlackUserId>,
    pub deleted_by: Option<SlackUserId>,
    pub prefs: SlackUserGroupPrefs,
    pub users: Option<Vec<SlackUserId>>,
    #[serde_as(as = "DisplayFromStr")]
    pub user_count: usize,
    pub channel_count: Option<u64>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SlackAutoType {
    Admin,
    Owner,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackUserGroupPrefs {
    pub channels: Vec<SlackChannelId>,
    pub groups: Vec<SlackUserGroupId>,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackAvatarHash(pub String);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackEnterpriseUser {
    pub id: SlackUserId,
    pub enterprise_id: SlackEnterpriseId,
    pub enterprise_name: Option<String>,
    #[serde(flatten)]
    pub flags: SlackUserFlags,
    pub teams: Option<Vec<SlackTeamId>>,
}
