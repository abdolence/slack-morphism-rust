use crate::*;

use crate::SlackUserId;
use rsb_derive::Builder;
use rvstruct::*;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackUser {
    pub id: SlackUserId,
    pub team_id: SlackTeamId,
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
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackAvatarHash(pub String);
