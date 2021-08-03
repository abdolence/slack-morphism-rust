//!
//! Support for Slack Conversations API methods
//!

use rsb_derive::Builder;
use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::*;
use futures::future::{BoxFuture, FutureExt};
use slack_morphism_models::*;
use std::collections::HashSet;

impl<'a, SCHC> SlackClientSession<'a, SCHC>
where
    SCHC: SlackClientHttpConnector + Send,
{
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
                    ("inclusive", req.inclusive.map(|v| v.to_string()).as_ref()),
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
                    (
                        "include_num_members",
                        req.include_num_members.map(|v| v.to_string()).as_ref(),
                    ),
                    (
                        "include_locale",
                        req.include_locale.map(|v| v.to_string()).as_ref(),
                    ),
                ],
            )
            .await
    }

    ///
    /// https://api.slack.com/methods/conversations.invite
    ///
    pub async fn conversations_invite(
        &self,
        req: &SlackApiConversationsInviteRequest,
    ) -> ClientResult<SlackApiConversationsInviteResponse> {
        self.http_api.http_post("conversations.invite", req).await
    }

    ///
    /// https://api.slack.com/methods/conversations.join
    ///
    pub async fn conversations_join(
        &self,
        req: &SlackApiConversationsJoinRequest,
    ) -> ClientResult<SlackApiConversationsJoinResponse> {
        self.http_api.http_post("conversations.join", req).await
    }

    ///
    /// https://api.slack.com/methods/conversations.kick
    ///
    pub async fn conversations_kick(
        &self,
        req: &SlackApiConversationsKickRequest,
    ) -> ClientResult<SlackApiConversationsKickResponse> {
        self.http_api.http_post("conversations.kick", req).await
    }

    ///
    /// https://api.slack.com/methods/conversations.leave
    ///
    pub async fn conversations_leave(
        &self,
        req: &SlackApiConversationsLeaveRequest,
    ) -> ClientResult<SlackApiConversationsLeaveResponse> {
        self.http_api.http_post("conversations.leave", req).await
    }

    ///
    /// https://api.slack.com/methods/conversations.list
    ///
    pub async fn conversations_list(
        &self,
        req: &SlackApiConversationsListRequest,
    ) -> ClientResult<SlackApiConversationsListResponse> {
        self.http_api
            .http_get(
                "conversations.list",
                &vec![
                    ("cursor", req.cursor.as_ref().map(|x| x.value())),
                    ("limit", req.limit.map(|v| v.to_string()).as_ref()),
                    (
                        "exclude_archived",
                        req.exclude_archived.map(|v| v.to_string()).as_ref(),
                    ),
                    (
                        "types",
                        req.types
                            .as_ref()
                            .map(|xs| {
                                xs.iter()
                                    .map(|x| x.to_string())
                                    .collect::<Vec<String>>()
                                    .join(",")
                            })
                            .as_ref(),
                    ),
                ],
            )
            .await
    }

    ///
    /// https://api.slack.com/methods/conversations.members
    ///
    pub async fn conversations_members(
        &self,
        req: &SlackApiConversationsMembersRequest,
    ) -> ClientResult<SlackApiConversationsMembersResponse> {
        self.http_api
            .http_get(
                "conversations.members",
                &vec![
                    ("channel", req.channel.as_ref().map(|x| x.value())),
                    ("cursor", req.cursor.as_ref().map(|x| x.value())),
                    ("limit", req.limit.map(|v| v.to_string()).as_ref()),
                ],
            )
            .await
    }

    ///
    /// https://api.slack.com/methods/conversations.open
    /// return_im is set to None
    ///
    pub async fn conversations_open(
        &self,
        req: &SlackApiConversationsOpenRequest,
    ) -> ClientResult<SlackApiConversationsOpenResponse<SlackBasicChannelInfo>> {
        self.http_api
            .http_post("conversations.open", &req.clone().without_return_im())
            .await
    }

    ///
    /// https://api.slack.com/methods/conversations.open
    /// return_im is set to Some(true)
    ///
    pub async fn conversations_open_full(
        &self,
        req: &SlackApiConversationsOpenRequest,
    ) -> ClientResult<SlackApiConversationsOpenResponse<SlackChannelInfo>> {
        self.http_api
            .http_post("conversations.open", &req.clone().with_return_im(true))
            .await
    }

    ///
    /// https://api.slack.com/methods/conversations.rename
    ///
    pub async fn conversations_rename(
        &self,
        req: &SlackApiConversationsRenameRequest,
    ) -> ClientResult<SlackApiConversationsRenameResponse> {
        self.http_api.http_post("conversations.rename", req).await
    }

    ///
    /// https://api.slack.com/methods/conversations.replies
    ///
    pub async fn conversations_replies(
        &self,
        req: &SlackApiConversationsRepliesRequest,
    ) -> ClientResult<SlackApiConversationsRepliesResponse> {
        self.http_api
            .http_get(
                "conversations.replies",
                &vec![
                    ("channel", Some(req.channel.value())),
                    ("ts", Some(req.channel.value())),
                    ("cursor", req.cursor.as_ref().map(|x| x.value())),
                    ("limit", req.limit.map(|v| v.to_string()).as_ref()),
                    ("inclusive", req.inclusive.map(|v| v.to_string()).as_ref()),
                    ("latest", req.latest.as_ref().map(|x| x.value())),
                    ("oldest", req.oldest.as_ref().map(|x| x.value())),
                ],
            )
            .await
    }

    ///
    /// https://api.slack.com/methods/conversations.setPurpose
    ///
    pub async fn conversations_set_purpose(
        &self,
        req: &SlackApiConversationsSetPurposeRequest,
    ) -> ClientResult<SlackApiConversationsSetPurposeResponse> {
        self.http_api
            .http_post("conversations.setPurpose", req)
            .await
    }

    ///
    /// https://api.slack.com/methods/conversations.setTopic
    ///
    pub async fn conversations_set_topic(
        &self,
        req: &SlackApiConversationsSetTopicRequest,
    ) -> ClientResult<SlackApiConversationsSetTopicResponse> {
        self.http_api.http_post("conversations.setTopic", req).await
    }

    ///
    /// https://api.slack.com/methods/conversations.unarchive
    ///
    pub async fn conversations_unarchive(
        &self,
        req: &SlackApiConversationsUnarchiveRequest,
    ) -> ClientResult<SlackApiConversationsUnarchiveResponse> {
        self.http_api.http_post("conversations.setTopic", req).await
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
    pub inclusive: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiConversationsHistoryResponse {
    pub messages: Vec<SlackHistoryMessage>,
    pub response_metadata: Option<SlackResponseMetadata>,
    pub has_more: Option<bool>,
    pub pin_count: Option<u64>,
}

impl<SCHC> SlackApiScrollableRequest<SCHC> for SlackApiConversationsHistoryRequest
where
    SCHC: SlackClientHttpConnector + Send + Sync + Clone + 'static,
{
    type ResponseType = SlackApiConversationsHistoryResponse;
    type CursorType = SlackCursorId;
    type ResponseItemType = SlackHistoryMessage;

    fn with_new_cursor(&self, new_cursor: Option<&Self::CursorType>) -> Self {
        self.clone().opt_cursor(new_cursor.cloned())
    }

    fn scroll<'a, 's>(
        &'a self,
        session: &'a SlackClientSession<'s, SCHC>,
    ) -> BoxFuture<'a, ClientResult<Self::ResponseType>> {
        async move { session.conversations_history(self).await }.boxed()
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
pub struct SlackApiConversationsInfoRequest {
    pub channel: SlackChannelId,
    pub include_locale: Option<bool>,
    pub include_num_members: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiConversationsInfoResponse {
    pub channel: SlackChannelInfo,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiConversationsInviteRequest {
    pub channel: SlackChannelId,
    pub users: Vec<SlackUserId>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiConversationsInviteResponse {
    pub channel: SlackChannelInfo,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiConversationsJoinRequest {
    pub channel: SlackChannelId,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiConversationsJoinResponse {
    pub channel: SlackChannelInfo,
    pub response_metadata: Option<SlackResponseMetadata>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiConversationsKickRequest {
    pub channel: SlackChannelId,
    pub user: SlackUserId,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiConversationsKickResponse {}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiConversationsLeaveRequest {
    pub channel: SlackChannelId,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiConversationsLeaveResponse {
    pub not_in_channel: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiConversationsListRequest {
    pub cursor: Option<SlackCursorId>,
    pub limit: Option<u16>,
    pub exclude_archived: Option<bool>,
    pub types: Option<Vec<SlackConversationType>>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiConversationsListResponse {
    pub channels: Vec<SlackChannelInfo>,
    pub response_metadata: Option<SlackResponseMetadata>,
}

impl<SCHC> SlackApiScrollableRequest<SCHC> for SlackApiConversationsListRequest
where
    SCHC: SlackClientHttpConnector + Send + Sync + Clone + 'static,
{
    type ResponseType = SlackApiConversationsListResponse;
    type CursorType = SlackCursorId;
    type ResponseItemType = SlackChannelInfo;

    fn with_new_cursor(&self, new_cursor: Option<&Self::CursorType>) -> Self {
        self.clone().opt_cursor(new_cursor.cloned())
    }

    fn scroll<'a, 's>(
        &'a self,
        session: &'a SlackClientSession<'s, SCHC>,
    ) -> BoxFuture<'a, ClientResult<Self::ResponseType>> {
        async move { session.conversations_list(self).await }.boxed()
    }
}

impl SlackApiScrollableResponse for SlackApiConversationsListResponse {
    type CursorType = SlackCursorId;
    type ResponseItemType = SlackChannelInfo;

    fn next_cursor(&self) -> Option<&Self::CursorType> {
        self.response_metadata
            .as_ref()
            .map(|rm| rm.next_cursor.as_ref())
            .flatten()
    }

    fn scrollable_items<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Self::ResponseItemType> + 'a> {
        Box::new(self.channels.iter())
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiConversationsMembersRequest {
    pub channel: Option<SlackChannelId>,
    pub cursor: Option<SlackCursorId>,
    pub limit: Option<u16>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiConversationsMembersResponse {
    pub members: Vec<SlackUserId>,
    pub response_metadata: Option<SlackResponseMetadata>,
}

impl<SCHC> SlackApiScrollableRequest<SCHC> for SlackApiConversationsMembersRequest
where
    SCHC: SlackClientHttpConnector + Send + Sync + Clone + 'static,
{
    type ResponseType = SlackApiConversationsMembersResponse;
    type CursorType = SlackCursorId;
    type ResponseItemType = SlackUserId;

    fn with_new_cursor(&self, new_cursor: Option<&Self::CursorType>) -> Self {
        self.clone().opt_cursor(new_cursor.cloned())
    }

    fn scroll<'a, 's>(
        &'a self,
        session: &'a SlackClientSession<'s, SCHC>,
    ) -> BoxFuture<'a, ClientResult<Self::ResponseType>> {
        async move { session.conversations_members(self).await }.boxed()
    }
}

impl SlackApiScrollableResponse for SlackApiConversationsMembersResponse {
    type CursorType = SlackCursorId;
    type ResponseItemType = SlackUserId;

    fn next_cursor(&self) -> Option<&Self::CursorType> {
        self.response_metadata
            .as_ref()
            .map(|rm| rm.next_cursor.as_ref())
            .flatten()
    }

    fn scrollable_items<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Self::ResponseItemType> + 'a> {
        Box::new(self.members.iter())
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiConversationsOpenRequest {
    pub channel: Option<SlackChannelId>,
    pub return_im: Option<bool>,
    pub users: Option<Vec<SlackUserId>>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiConversationsOpenResponse<T>
where
    T: HasChannelInfo,
{
    pub channel: T,
    pub already_open: Option<bool>,
    pub no_op: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiConversationsRenameRequest {
    pub channel: SlackChannelId,
    pub name: String,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiConversationsRenameResponse {
    pub channel: SlackChannelInfo,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiConversationsRepliesRequest {
    pub channel: SlackChannelId,
    pub ts: SlackTs,
    pub cursor: Option<SlackCursorId>,
    pub latest: Option<SlackTs>,
    pub limit: Option<u16>,
    pub oldest: Option<SlackTs>,
    pub inclusive: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiConversationsRepliesResponse {
    pub messages: Vec<SlackHistoryMessage>,
    pub response_metadata: Option<SlackResponseMetadata>,
    pub has_more: Option<bool>,
}

impl<SCHC> SlackApiScrollableRequest<SCHC> for SlackApiConversationsRepliesRequest
where
    SCHC: SlackClientHttpConnector + Send + Sync + Clone + 'static,
{
    type ResponseType = SlackApiConversationsRepliesResponse;
    type CursorType = SlackCursorId;
    type ResponseItemType = SlackHistoryMessage;

    fn with_new_cursor(&self, new_cursor: Option<&Self::CursorType>) -> Self {
        self.clone().opt_cursor(new_cursor.cloned())
    }

    fn scroll<'a, 's>(
        &'a self,
        session: &'a SlackClientSession<'s, SCHC>,
    ) -> BoxFuture<'a, ClientResult<Self::ResponseType>> {
        async move { session.conversations_replies(self).await }.boxed()
    }
}

impl SlackApiScrollableResponse for SlackApiConversationsRepliesResponse {
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
pub struct SlackApiConversationsSetPurposeRequest {
    pub channel: SlackChannelId,
    pub purpose: String,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiConversationsSetPurposeResponse {
    pub purpose: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiConversationsSetTopicRequest {
    pub channel: SlackChannelId,
    pub topic: String,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiConversationsSetTopicResponse {
    pub topic: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiConversationsUnarchiveRequest {
    pub channel: SlackChannelId,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiConversationsUnarchiveResponse {}
