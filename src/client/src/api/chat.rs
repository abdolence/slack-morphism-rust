//!
//! Support for Slack Chat API methods
//!

use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::scroller::*;
use crate::ClientResult;
use crate::SlackClientSession;
use futures::future::{BoxFuture, FutureExt};
use slack_morphism_models::*;

impl<'a> SlackClientSession<'a> {
    ///
    /// https://api.slack.com/methods/chat.delete
    ///
    pub async fn chat_delete(
        &self,
        req: &SlackApiChatDeleteRequest,
    ) -> ClientResult<SlackApiChatDeleteResponse> {
        self.http_api.http_post("chat.delete", req).await
    }

    ///
    /// https://api.slack.com/methods/chat.deleteScheduledMessage
    ///
    pub async fn chat_delete_scheduled_message(
        &self,
        req: &SlackApiChatDeleteScheduledMessageRequest,
    ) -> ClientResult<SlackApiChatDeleteScheduledMessageResponse> {
        self.http_api
            .http_post("chat.deleteScheduledMessage", req)
            .await
    }

    ///
    /// https://api.slack.com/methods/chat.getPermalink
    ///
    pub async fn chat_get_permalink(
        &self,
        req: &SlackApiChatGetPermalinkRequest,
    ) -> ClientResult<SlackApiChatGetPermalinkResponse> {
        self.http_api
            .http_get(
                "chat.getPermalink",
                &vec![
                    ("channel", Some(&req.channel.value())),
                    ("message_ts", Some(&req.message_ts.value())),
                ],
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
        self.http_api.http_post("chat.postEphemeral", req).await
    }

    ///
    /// https://api.slack.com/methods/chat.postMessage
    ///
    pub async fn chat_post_message(
        &self,
        req: &SlackApiChatPostMessageRequest,
    ) -> ClientResult<SlackApiChatPostMessageResponse> {
        self.http_api.http_post("chat.postMessage", req).await
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
    pub scheduled_message: SlackTs,
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
    pub message_ts: SlackTs,
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
    ts: SlackTs,
    message: SlackMessage,
}
