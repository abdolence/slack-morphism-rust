//!
//! Support for Slack Conversations API methods
//!

use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use rvstruct::ValueStruct;

use crate::*;
use slack_morphism_models::*;
use std::collections::HashSet;
use futures::future::{BoxFuture, FutureExt};

impl<'a> SlackClientSession<'a> {
    ///
    /// https://api.slack.com/methods/conversations.archive
    ///
    pub async fn conversations_archive(
        &self,
        req: &SlackApiConversationsArchiveRequest,
    ) -> ClientResult<SlackApiConversationsArchiveResponse> {
        self.http_api.http_post("conversations.archive", req).await
    }

    ///
    /// https://api.slack.com/methods/conversations.close
    ///
    pub async fn conversations_close(
        &self,
        req: &SlackApiConversationsCloseRequest,
    ) -> ClientResult<SlackApiConversationsCloseResponse> {
        self.http_api.http_post("conversations.close", req).await
    }

    ///
    /// https://api.slack.com/methods/conversations.create
    ///
    pub async fn conversations_create(
        &self,
        req: &SlackApiConversationsCreateRequest,
    ) -> ClientResult<SlackApiConversationsCreateResponse> {
        self.http_api.http_post("conversations.create", req).await
    }

    ///
    /// https://api.slack.com/methods/conversations.history
    ///
    pub async fn conversations_history(
        &self,
        req: &SlackApiConversationsHistoryRequest,
    ) -> ClientResult<SlackApiConversationsHistoryResponse> {
        self.http_api
            .http_get(
                "conversations.history",
                &vec![
                    ("channel", req.channel.as_ref().map(|x| x.value())),
                    ("cursor", req.cursor.as_ref().map(|x| x.value())),
                    ("limit", req.limit.map(|v| v.to_string()).as_ref()),
                    ("inclusive",req.inclusive.map(|v| v.to_string()).as_ref()),
                    ("latest", req.latest.as_ref().map(|x| x.value())),
                    ("oldest", req.oldest.as_ref().map(|x| x.value())),
                ],
            )
            .await
    }

    ///
    /// https://api.slack.com/methods/conversations.info
    ///
    pub async fn conversations_info(
        &self,
        req: &SlackApiConversationsInfoRequest,
    ) -> ClientResult<SlackApiConversationsInfoResponse> {
        self.http_api
            .http_get(
                "conversations.info",
                &vec![
                    ("channel", Some(req.channel.value())),
                    ("include_num_members", req.include_num_members.map(|v| v.to_string()).as_ref()),
                    ("include_locale",req.include_locale.map(|v| v.to_string()).as_ref())
                ],
            )
            .await
    }

}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiConversationsArchiveRequest {
    pub channel: SlackChannelId,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiConversationsArchiveResponse {}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiConversationsCloseRequest {
    pub channel: SlackChannelId,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiConversationsCloseResponse {
    pub no_op: Option<bool>,
    pub already_closed: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiConversationsCreateRequest {
    pub name: String,
    pub is_private: Option<bool>,
    pub user_ds: Option<HashSet<SlackUserId>>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiConversationsCreateResponse {
    pub channel: SlackChannelInfo,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiConversationsHistoryRequest {
    pub channel: Option<SlackChannelId>,
    pub cursor: Option<SlackCursorId>,
    pub latest: Option<SlackTs>,
    pub limit: Option<u16>,
    pub oldest: Option<SlackTs>,
    pub inclusive : Option<bool>
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiConversationsHistoryResponse {
    pub messages: Vec<SlackHistoryMessage>,
    pub response_metadata: Option<SlackResponseMetadata>,
    pub has_more: Option<bool>,
    pub pin_count: Option<u64>,
}

impl SlackApiScrollableRequest for SlackApiConversationsHistoryRequest {
    type ResponseType = SlackApiConversationsHistoryResponse;
    type CursorType = SlackCursorId;
    type ResponseItemType = SlackHistoryMessage;

    fn with_new_cursor(&self, new_cursor: Option<&Self::CursorType>) -> Self {
        self.clone().opt_cursor(new_cursor.cloned())
    }

    fn scroll<'a, 's>(
        &'a self,
        session: &'a SlackClientSession<'s>,
    ) -> BoxFuture<'a, ClientResult<Self::ResponseType>> {
        async move { session.conversations_history(&self).await }.boxed()
    }
}

impl SlackApiScrollableResponse for SlackApiConversationsHistoryResponse {
    type CursorType = SlackCursorId;
    type ResponseItemType = SlackHistoryMessage;

    fn next_cursor(&self) -> Option<&Self::CursorType> {
        self.response_metadata
            .as_ref()
            .map(|rm| rm.next_cursor.as_ref())
            .flatten()
    }

    fn scrollable_items<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Self::ResponseItemType> + 'a> {
        Box::new(self.messages.iter())
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiConversationsInfoRequest{
    pub channel : SlackChannelId,
    pub include_locale: Option<bool>,
    pub include_num_members: Option<bool>
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiConversationsInfoResponse{
    pub channel: SlackChannelInfo
}