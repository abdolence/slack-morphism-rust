use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use url::Url;

use crate::blocks::*;
use crate::events::*;
use crate::models::messages::*;
use crate::*;

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
    EmojiChanged(SlackEmojiChangedEvent),
    MemberJoinedChannel(SlackMemberJoinedChannelEvent),
    MemberLeftChannel(SlackMemberLeftChannelEvent),
    ChannelCreated(SlackChannelCreatedEvent),
    ChannelDeleted(SlackChannelDeletedEvent),
    ChannelArchive(SlackChannelArchiveEvent),
    ChannelRename(SlackChannelRenameEvent),
    ChannelUnarchive(SlackChannelUnarchiveEvent),
    TeamJoin(SlackTeamJoinEvent),
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
    #[serde(rename = "channel_leave")]
    ChannelLeave,
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
    #[serde(rename = "file_share")]
    FileShare,
    #[serde(rename = "message_changed")]
    MessageChanged,
    #[serde(rename = "message_deleted")]
    MessageDeleted,
    #[serde(rename = "thread_broadcast")]
    ThreadBroadcast,
    #[serde(rename = "tombstone")]
    Tombstone,
    #[serde(rename = "joiner_notification")]
    JoinerNotification,
    #[serde(rename = "slackbot_response")]
    SlackbotResponse,
    #[serde(rename = "emoji_changed")]
    EmojiChanged,
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
pub struct SlackEmojiChangedEvent {
    pub subtype: SlackEmojiEventType,
    pub name: Option<String>,
    pub names: Option<Vec<String>>,
    pub old_name: Option<String>,
    pub new_name: Option<String>,
    pub value: Option<Url>,
    pub event_ts: SlackTs,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum SlackEmojiEventType {
    #[serde(rename = "remove")]
    EmojiRemoved,
    #[serde(rename = "add")]
    EmojiAdded,
    #[serde(rename = "rename")]
    EmojiRenamed,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackLinkObject {
    pub domain: String,
    pub url: Url,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackMemberJoinedChannelEvent {
    pub user: SlackUserId,
    pub channel: SlackChannelId,
    pub channel_type: SlackChannelType,
    pub team: SlackTeamId,
    pub inviter: Option<SlackUserId>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackMemberLeftChannelEvent {
    pub user: SlackUserId,
    pub channel: SlackChannelId,
    pub channel_type: SlackChannelType,
    pub team: SlackTeamId,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackChannelCreatedEvent {
    pub channel: SlackChannelInfo,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackChannelDeletedEvent {
    pub channel: SlackChannelId,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackChannelArchiveEvent {
    pub channel: SlackChannelId,
    pub user: SlackUserId,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackChannelRenameEvent {
    pub channel: SlackChannelInfo,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackChannelUnarchiveEvent {
    pub channel: SlackChannelId,
    pub user: SlackUserId,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackTeamJoinEvent {
    pub user: SlackUser,
}
