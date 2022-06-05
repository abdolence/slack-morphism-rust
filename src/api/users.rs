//!
//! Support for Slack Users API methods
//!

use rsb_derive::Builder;
use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::*;
use crate::ratectl::*;
use crate::scroller::*;
use crate::SlackClientSession;
use crate::{ClientResult, SlackClientHttpConnector};
use futures::future::{BoxFuture, FutureExt};

impl<'a, SCHC> SlackClientSession<'a, SCHC>
where
    SCHC: SlackClientHttpConnector + Send,
{
    ///
    /// https://api.slack.com/methods/users.conversations
    ///
    pub async fn users_conversations(
        &self,
        req: &SlackApiUsersConversationsRequest,
    ) -> ClientResult<SlackApiUsersConversationsResponse> {
        self.http_session_api
            .http_get(
                "users.conversations",
                &vec![
                    ("user", req.user.as_ref().map(|x| x.value())),
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
                Some(&SLACK_TIER3_METHOD_CONFIG),
            )
            .await
    }

    ///
    /// https://api.slack.com/methods/users.getPresence
    ///
    pub async fn users_get_presence(
        &self,
        req: &SlackApiUsersGetPresenceRequest,
    ) -> ClientResult<SlackApiUsersGetPresenceResponse> {
        self.http_session_api
            .http_get(
                "users.getPresence",
                &vec![("user", Some(req.user.value()))],
                Some(&SLACK_TIER3_METHOD_CONFIG),
            )
            .await
    }

    ///
    /// https://api.slack.com/methods/users.identity
    ///
    pub async fn users_identity(&self) -> ClientResult<SlackApiUsersGetPresenceResponse> {
        self.http_session_api
            .http_get(
                "users.identity",
                &crate::client::SLACK_HTTP_EMPTY_GET_PARAMS.clone(),
                Some(&SLACK_TIER4_METHOD_CONFIG),
            )
            .await
    }

    ///
    /// https://api.slack.com/methods/users.info
    ///
    pub async fn users_info(
        &self,
        req: &SlackApiUsersInfoRequest,
    ) -> ClientResult<SlackApiUsersInfoResponse> {
        self.http_session_api
            .http_get(
                "users.info",
                &vec![
                    ("user", Some(req.user.value())),
                    (
                        "include_locale",
                        req.include_locale.map(|v| v.to_string()).as_ref(),
                    ),
                ],
                Some(&SLACK_TIER4_METHOD_CONFIG),
            )
            .await
    }

    ///
    /// https://api.slack.com/methods/users.list
    ///
    pub async fn users_list(
        &self,
        req: &SlackApiUsersListRequest,
    ) -> ClientResult<SlackApiUsersListResponse> {
        self.http_session_api
            .http_get(
                "users.list",
                &vec![
                    ("cursor", req.cursor.as_ref().map(|v| v.value())),
                    ("team_id", req.team_id.as_ref().map(|v| v.value())),
                    ("limit", req.limit.map(|v| v.to_string()).as_ref()),
                    (
                        "include_locale",
                        req.include_locale.map(|v| v.to_string()).as_ref(),
                    ),
                ],
                Some(&SLACK_TIER2_METHOD_CONFIG),
            )
            .await
    }

    ///
    /// https://api.slack.com/methods/users.lookupByEmail
    ///
    pub async fn users_lookup_by_email(
        &self,
        req: &SlackApiUsersLookupByEmailRequest,
    ) -> ClientResult<SlackApiUsersLookupByEmailResponse> {
        self.http_session_api
            .http_get(
                "users.lookupByEmail",
                &vec![("email", Some(req.email.value()))],
                Some(&SLACK_TIER3_METHOD_CONFIG),
            )
            .await
    }

    ///
    /// https://api.slack.com/methods/users.setPresence
    ///
    pub async fn users_set_presence(
        &self,
        req: &SlackApiUsersSetPresenceRequest,
    ) -> ClientResult<SlackApiUsersSetPresenceResponse> {
        self.http_session_api
            .http_post("users.setPresence", req, Some(&SLACK_TIER2_METHOD_CONFIG))
            .await
    }

    ///
    /// https://api.slack.com/methods/users.profile.get
    ///
    pub async fn users_profile_get(
        &self,
        req: &SlackApiUsersProfileGetRequest,
    ) -> ClientResult<SlackApiUsersProfileGetResponse> {
        self.http_session_api
            .http_get(
                "users.profile.get",
                &vec![
                    ("user", req.user.as_ref().map(|v| v.value())),
                    (
                        "include_locale",
                        req.include_locale.map(|v| v.to_string()).as_ref(),
                    ),
                ],
                Some(&SLACK_TIER4_METHOD_CONFIG),
            )
            .await
    }

    ///
    /// https://api.slack.com/methods/users.profile.set
    ///
    pub async fn users_profile_set(
        &self,
        req: &SlackApiUsersProfileSetRequest,
    ) -> ClientResult<SlackApiUsersProfileSetResponse> {
        self.http_session_api
            .http_post("users.profile.set", req, Some(&SLACK_TIER3_METHOD_CONFIG))
            .await
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiUsersConversationsRequest {
    pub cursor: Option<SlackCursorId>,
    pub limit: Option<u16>,
    pub exclude_archived: Option<bool>,
    pub types: Option<Vec<SlackConversationType>>,
    pub user: Option<SlackUserId>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiUsersConversationsResponse {
    pub channels: Vec<SlackChannelInfo>,
    pub response_metadata: Option<SlackResponseMetadata>,
}

impl<SCHC> SlackApiScrollableRequest<SCHC> for SlackApiUsersConversationsRequest
where
    SCHC: SlackClientHttpConnector + Send + Sync + Clone + 'static,
{
    type ResponseType = SlackApiUsersConversationsResponse;
    type CursorType = SlackCursorId;
    type ResponseItemType = SlackChannelInfo;

    fn with_new_cursor(&self, new_cursor: Option<&Self::CursorType>) -> Self {
        self.clone().opt_cursor(new_cursor.cloned())
    }

    fn scroll<'a, 's>(
        &'a self,
        session: &'a SlackClientSession<'s, SCHC>,
    ) -> BoxFuture<'a, ClientResult<Self::ResponseType>> {
        async move { session.users_conversations(self).await }.boxed()
    }
}

impl SlackApiScrollableResponse for SlackApiUsersConversationsResponse {
    type CursorType = SlackCursorId;
    type ResponseItemType = SlackChannelInfo;

    fn next_cursor(&self) -> Option<&Self::CursorType> {
        self.response_metadata
            .as_ref()
            .and_then(|rm| rm.next_cursor.as_ref())
    }

    fn scrollable_items<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Self::ResponseItemType> + 'a> {
        Box::new(self.channels.iter())
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiUsersListRequest {
    pub cursor: Option<SlackCursorId>,
    pub include_locale: Option<bool>,
    pub limit: Option<u16>,
    pub team_id: Option<SlackTeamId>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiUsersListResponse {
    pub members: Vec<SlackUser>,
    pub response_metadata: Option<SlackResponseMetadata>,
}

impl<SCHC> SlackApiScrollableRequest<SCHC> for SlackApiUsersListRequest
where
    SCHC: SlackClientHttpConnector + Send + Sync + Clone + 'static,
{
    type ResponseType = SlackApiUsersListResponse;
    type CursorType = SlackCursorId;
    type ResponseItemType = SlackUser;

    fn with_new_cursor(&self, new_cursor: Option<&Self::CursorType>) -> Self {
        self.clone().opt_cursor(new_cursor.cloned())
    }

    fn scroll<'a, 's>(
        &'a self,
        session: &'a SlackClientSession<'s, SCHC>,
    ) -> BoxFuture<'a, ClientResult<Self::ResponseType>> {
        async move { session.users_list(self).await }.boxed()
    }
}

impl SlackApiScrollableResponse for SlackApiUsersListResponse {
    type CursorType = SlackCursorId;
    type ResponseItemType = SlackUser;

    fn next_cursor(&self) -> Option<&Self::CursorType> {
        self.response_metadata
            .as_ref()
            .and_then(|rm| rm.next_cursor.as_ref())
    }

    fn scrollable_items<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Self::ResponseItemType> + 'a> {
        Box::new(self.members.iter())
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiUsersGetPresenceRequest {
    pub user: SlackUserId,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiUsersGetPresenceResponse {
    pub presence: String,
    pub online: Option<bool>,
    pub auto_away: Option<bool>,
    pub manual_away: Option<bool>,
    pub connection_count: Option<u64>,
    pub last_activity: Option<SlackDateTime>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiUsersIdentityResponse {
    pub user: SlackUserProfile,
    pub team: SlackBasicTeamInfo,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiUsersInfoRequest {
    pub user: SlackUserId,
    pub include_locale: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiUsersInfoResponse {
    pub user: SlackUser,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiUsersLookupByEmailRequest {
    pub email: EmailAddress,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiUsersLookupByEmailResponse {
    pub user: SlackUser,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiUsersSetPresenceRequest {
    pub presence: String,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiUsersSetPresenceResponse {}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiUsersProfileGetRequest {
    pub user: Option<SlackUserId>,
    pub include_locale: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiUsersProfileGetResponse {
    pub profile: SlackUserProfile,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiUsersProfileSetRequest {
    pub profile: SlackUserProfile,
    pub user: Option<SlackUserId>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiUsersProfileSetResponse {
    pub profile: SlackUserProfile,
}
