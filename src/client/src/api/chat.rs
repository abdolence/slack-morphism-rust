//!
//! Support for Slack Chat API methods
//!

use rsb_derive::Builder;
use rvstruct::*;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::ratectl::*;
use crate::scroller::*;
use crate::SlackClientSession;
use crate::{ClientResult, SlackClientHttpConnector};
use futures::future::{BoxFuture, FutureExt};
use lazy_static::lazy_static;
use slack_morphism_models::*;
use std::collections::HashMap;

impl<'a, SCHC> SlackClientSession<'a, SCHC>
where
    SCHC: SlackClientHttpConnector + Send,
{
    ///
    /// https://api.slack.com/methods/chat.delete
    ///
    pub async fn chat_delete(
        &self,
        req: &SlackApiChatDeleteRequest,
    ) -> ClientResult<SlackApiChatDeleteResponse> {
        self.http_session_api
            .http_post("chat.delete", req, Some(&SLACK_TIER3_METHOD_CONFIG))
            .await
    }

    ///
    /// https://api.slack.com/methods/chat.deleteScheduledMessage
    ///
    pub async fn chat_delete_scheduled_message(
        &self,
        req: &SlackApiChatDeleteScheduledMessageRequest,
    ) -> ClientResult<SlackApiChatDeleteScheduledMessageResponse> {
        self.http_session_api
            .http_post(
                "chat.deleteScheduledMessage",
                req,
                Some(&SLACK_TIER3_METHOD_CONFIG),
            )
            .await
    }

    ///
    /// https://api.slack.com/methods/chat.getPermalink
    ///
    pub async fn chat_get_permalink(
        &self,
        req: &SlackApiChatGetPermalinkRequest,
    ) -> ClientResult<SlackApiChatGetPermalinkResponse> {
        self.http_session_api
            .http_get(
                "chat.getPermalink",
                &vec![
                    ("channel", Some(&req.channel.value())),
                    ("message_ts", Some(&req.message_ts.value())),
                ],
                Some(&CHAT_GET_PERMLINK_SPECIAL_LIMIT_RATE_CTL),
            )
            .await
    }

    ///
    /// https://api.slack.com/methods/chat.postEphemeral
    ///
    pub async fn chat_post_ephemeral(
        &self,
        req: &SlackApiChatPostEphemeralRequest,
    ) -> ClientResult<SlackApiChatPostEphemeralResponse> {
        self.http_session_api
            .http_post("chat.postEphemeral", req, Some(&SLACK_TIER4_METHOD_CONFIG))
            .await
    }

    ///
    /// https://api.slack.com/methods/chat.postMessage
    ///
    pub async fn chat_post_message(
        &self,
        req: &SlackApiChatPostMessageRequest,
    ) -> ClientResult<SlackApiChatPostMessageResponse> {
        self.http_session_api
            .http_post(
                "chat.postMessage",
                req,
                Some(&CHAT_POST_MESSAGE_SPECIAL_LIMIT_RATE_CTL),
            )
            .await
    }

    ///
    /// https://api.slack.com/methods/chat.scheduleMessage
    ///
    pub async fn chat_schedule_message(
        &self,
        req: &SlackApiChatScheduleMessageRequest,
    ) -> ClientResult<SlackApiChatScheduleMessageResponse> {
        self.http_session_api
            .http_post(
                "chat.scheduleMessage",
                req,
                Some(&SLACK_TIER3_METHOD_CONFIG),
            )
            .await
    }

    ///
    /// https://api.slack.com/methods/chat.unfurl
    ///
    pub async fn chat_unfurl(
        &self,
        req: &SlackApiChatUnfurlRequest,
    ) -> ClientResult<SlackApiChatUnfurlResponse> {
        self.http_session_api
            .http_post("chat.unfurl", req, Some(&SLACK_TIER3_METHOD_CONFIG))
            .await
    }

    ///
    /// https://api.slack.com/methods/chat.update
    ///
    pub async fn chat_update(
        &self,
        req: &SlackApiChatUpdateRequest,
    ) -> ClientResult<SlackApiChatUpdateResponse> {
        self.http_session_api
            .http_post("chat.update", req, Some(&SLACK_TIER3_METHOD_CONFIG))
            .await
    }

    ///
    /// https://api.slack.com/methods/chat.scheduledMessages.list
    ///
    pub async fn chat_scheduled_messages_list(
        &self,
        req: &SlackApiChatScheduledMessagesListRequest,
    ) -> ClientResult<SlackApiChatScheduledMessagesListResponse> {
        self.http_session_api
            .http_post(
                "chat.scheduledMessages.list",
                req,
                Some(&SLACK_TIER3_METHOD_CONFIG),
            )
            .await
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiChatDeleteRequest {
    pub channel: SlackChannelId,
    pub ts: SlackTs,
    pub as_user: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiChatDeleteResponse {
    pub channel: SlackChannelId,
    pub ts: SlackTs,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiChatDeleteScheduledMessageRequest {
    pub channel: SlackChannelId,
    pub scheduled_message: SlackScheduledMid,
    pub as_user: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiChatDeleteScheduledMessageResponse {}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiChatGetPermalinkRequest {
    pub channel: SlackChannelId,
    pub message_ts: SlackTs,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiChatGetPermalinkResponse {
    pub channel: SlackChannelId,
    pub permalink: String,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiChatPostEphemeralRequest {
    pub channel: SlackChannelId,
    pub user: SlackUserId,
    #[serde(flatten)]
    pub content: SlackMessageContent,
    pub as_user: Option<bool>,
    pub icon_emoji: Option<String>,
    pub icon_url: Option<String>,
    pub link_names: Option<bool>,
    pub parse: Option<String>,
    pub thread_ts: Option<SlackTs>,
    pub username: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiChatPostEphemeralResponse {}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiChatPostMessageRequest {
    pub channel: SlackChannelId,
    #[serde(flatten)]
    pub content: SlackMessageContent,
    pub as_user: Option<bool>,
    pub icon_emoji: Option<String>,
    pub icon_url: Option<String>,
    pub link_names: Option<bool>,
    pub parse: Option<String>,
    pub thread_ts: Option<SlackTs>,
    pub username: Option<String>,
    pub reply_broadcast: Option<bool>,
    pub unfurl_links: Option<bool>,
    pub unfurl_media: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiChatPostMessageResponse {
    pub ts: SlackTs,
    pub message: SlackMessage,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiChatScheduleMessageRequest {
    pub channel: SlackChannelId,
    #[serde(flatten)]
    pub content: SlackMessageContent,
    pub post_at: SlackDateTime,
    pub as_user: Option<bool>,
    pub icon_emoji: Option<String>,
    pub icon_url: Option<String>,
    pub link_names: Option<bool>,
    pub parse: Option<String>,
    pub thread_ts: Option<SlackTs>,
    pub username: Option<String>,
    pub reply_broadcast: Option<bool>,
    pub unfurl_links: Option<bool>,
    pub unfurl_media: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiChatScheduleMessageResponse {
    pub channel: SlackChannelId,
    pub scheduled_message_id: SlackScheduledMid,
    pub post_at: SlackDateTime,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiChatUnfurlRequest {
    pub channel: SlackChannelId,
    pub ts: SlackTs,
    pub unfurls: HashMap<String, SlackApiChatUnfurlMapItem>,
    pub user_auth_message: Option<String>,
    pub user_auth_required: Option<bool>,
    pub user_auth_url: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiChatUnfurlResponse {}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiChatUnfurlMapItem {
    pub text: String,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiChatUpdateRequest {
    pub channel: SlackChannelId,
    #[serde(flatten)]
    pub content: SlackMessageContent,
    pub ts: SlackTs,
    pub as_user: Option<bool>,
    pub link_names: Option<bool>,
    pub parse: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiChatUpdateResponse {
    pub channel: SlackChannelId,
    pub ts: SlackTs,
    pub thread_ts: Option<SlackTs>,
    pub message: SlackUpdatedMessage,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiChatScheduledMessagesListRequest {
    pub channel: Option<SlackChannelId>,
    pub cursor: Option<SlackCursorId>,
    pub latest: Option<SlackTs>,
    pub limit: Option<u16>,
    pub oldest: Option<SlackTs>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiChatScheduledMessagesListResponse {
    pub scheduled_messages: Vec<SlackApiChatScheduledMessageInfo>,
    pub response_metadata: Option<SlackResponseMetadata>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiChatScheduledMessageInfo {
    pub id: SlackScheduledMid,
    pub channel_id: SlackChannelId,
    pub post_at: SlackDateTime,
    pub date_created: SlackDateTime,
}

impl<SCHC> SlackApiScrollableRequest<SCHC> for SlackApiChatScheduledMessagesListRequest
where
    SCHC: SlackClientHttpConnector + Send + Sync + Clone + 'static,
{
    type ResponseType = SlackApiChatScheduledMessagesListResponse;
    type CursorType = SlackCursorId;
    type ResponseItemType = SlackApiChatScheduledMessageInfo;

    fn with_new_cursor(&self, new_cursor: Option<&Self::CursorType>) -> Self {
        self.clone().opt_cursor(new_cursor.cloned())
    }

    fn scroll<'a, 's>(
        &'a self,
        session: &'a SlackClientSession<'s, SCHC>,
    ) -> BoxFuture<'a, ClientResult<Self::ResponseType>> {
        async move { session.chat_scheduled_messages_list(self).await }.boxed()
    }
}

impl SlackApiScrollableResponse for SlackApiChatScheduledMessagesListResponse {
    type CursorType = SlackCursorId;
    type ResponseItemType = SlackApiChatScheduledMessageInfo;

    fn next_cursor(&self) -> Option<&Self::CursorType> {
        self.response_metadata
            .as_ref()
            .map(|rm| rm.next_cursor.as_ref())
            .flatten()
    }

    fn scrollable_items<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Self::ResponseItemType> + 'a> {
        Box::new(self.scheduled_messages.iter())
    }
}

lazy_static! {
    pub static ref CHAT_GET_PERMLINK_SPECIAL_LIMIT_RATE_CTL: SlackApiMethodRateControlConfig =
        SlackApiMethodRateControlConfig::new().with_special_rate_limit(
            SlackApiRateControlSpecialLimit::new(
                "chat.getPermalink".into(),
                SlackApiRateControlLimit::new(100, std::time::Duration::from_secs(1))
            )
        );
    pub static ref CHAT_POST_MESSAGE_SPECIAL_LIMIT_RATE_CTL: SlackApiMethodRateControlConfig =
        SlackApiMethodRateControlConfig::new().with_special_rate_limit(
            SlackApiRateControlSpecialLimit::new(
                "chat.postMessage".into(),
                SlackApiRateControlLimit::new(1, std::time::Duration::from_secs(1))
            )
        );
}
