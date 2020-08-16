use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::blocks::*;
use crate::common::*;
use crate::messages::*;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SlackPushEvent {
    #[serde(rename = "url_verification")]
    UrlVerification(SlackUrlVerificationEvent),
    #[serde(rename = "event_callback")]
    EventCallback(SlackEventCallback),
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
pub struct SlackEventCallback {
    pub team_id: SlackTeamId,
    pub api_app_id: SlackAppId,
    pub event: SlackEventCallbackBody,
    pub event_id: SlackEventId,
    pub event_time: SlackDateTime,
    pub authed_users: Option<Vec<SlackUserId>>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SlackEventCallbackBody {
    #[serde(rename = "message")]
    Message(SlackMessageEvent),
    #[serde(rename = "app_home_opened")]
    AppHomeOpened(SlackAppHomeOpenedEvent),
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackMessageEvent {
    #[serde(flatten)]
    pub origin: SlackMessageOrigin,
    #[serde(flatten)]
    pub content: SlackMessageContent,
    #[serde(flatten)]
    pub sender: SlackMessageSender,
    pub subtype: Option<SlackMessageEventType>,
    pub hidden: Option<bool>,
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
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackAppHomeOpenedEvent {
    pub user: SlackUserId,
    pub channel: SlackChannelId,
    pub tab: String,
    pub view: Option<SlackView>,
}
