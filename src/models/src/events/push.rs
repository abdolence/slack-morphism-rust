use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::blocks::*;
use crate::common::*;
use crate::events::*;
use crate::messages::*;

#[allow(clippy::large_enum_variant)]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SlackPushEvent {
    #[serde(rename = "url_verification")]
    UrlVerification(SlackUrlVerificationEvent),
    #[serde(rename = "event_callback")]
    EventCallback(SlackPushEventCallback),
    #[serde(rename = "app_rate_limited")]
    AppRateLimited(SlackAppRateLimitedEvent),
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackUrlVerificationEvent {
    pub challenge: String,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackAppRateLimitedEvent {
    pub team_id: String,
    pub minute_rate_limited: SlackDateTime,
    pub api_app_id: String,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackPushEventCallback {
    pub team_id: SlackTeamId,
    pub api_app_id: SlackAppId,
    pub event: SlackEventCallbackBody,
    pub event_id: SlackEventId,
    pub event_time: SlackDateTime,
    pub event_context: Option<SlackEventContext>,
    pub authed_users: Option<Vec<SlackUserId>>,
    pub authorizations: Option<Vec<SlackEventAuthorization>>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SlackEventCallbackBody {
    Message(SlackMessageEvent),
    AppHomeOpened(SlackAppHomeOpenedEvent),
    AppMention(SlackAppMentionEvent),
    AppUninstalled(SlackAppUninstalledEvent),
    LinkShared(SlackLinkSharedEvent),
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackMessageEvent {
    #[serde(flatten)]
    pub origin: SlackMessageOrigin,
    #[serde(flatten)]
    pub content: Option<SlackMessageContent>,
    #[serde(flatten)]
    pub sender: SlackMessageSender,
    pub subtype: Option<SlackMessageEventType>,
    pub hidden: Option<bool>,
    pub edited: Option<SlackMessageEventEdited>,
    pub deleted_ts: Option<SlackTs>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum SlackMessageEventType {
    #[serde(rename = "bot_message")]
    BotMessage,
    #[serde(rename = "me_message")]
    MeMessage,
    #[serde(rename = "channel_join")]
    ChannelJoin,
    #[serde(rename = "bot_add")]
    BotAdd,
    #[serde(rename = "bot_remove")]
    BotRemove,
    #[serde(rename = "channel_topic")]
    ChannelTopic,
    #[serde(rename = "channel_purpose")]
    ChannelPurpose,
    #[serde(rename = "channel_name")]
    ChannelName,
    #[serde(rename = "message_changed")]
    MessageChanged,
    #[serde(rename = "message_deleted")]
    MessageDeleted,
    #[serde(rename = "tombstone")]
    Tombstone,
    #[serde(rename = "joiner_notification")]
    JoinerNotification,
    #[serde(rename = "slackbot_response")]
    SlackbotResponse,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackAppHomeOpenedEvent {
    pub user: SlackUserId,
    pub channel: SlackChannelId,
    pub tab: String,
    pub view: Option<SlackView>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackAppMentionEvent {
    pub user: SlackUserId,
    pub channel: SlackChannelId,
    #[serde(flatten)]
    pub content: SlackMessageContent,
    #[serde(flatten)]
    pub origin: SlackMessageOrigin,
}

type SlackMessageEventEdited = SlackMessageEdited;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackAppUninstalledEvent {}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackLinkSharedEvent {
    pub channel: SlackChannelId,
    pub event_ts: SlackTs,
    pub is_bot_user_member: bool,
    pub links: Vec<SlackLinkObject>,
    pub message_ts: SlackTs,
    pub source: String,
    pub unfurl_id: SlackUnfurlId,
    pub user: SlackUserId,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackLinkObject {
    pub domain: String,
    pub url: String,
}
