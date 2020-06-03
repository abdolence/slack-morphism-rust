use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::scroller::*;
use crate::ClientResult;
use crate::SlackClientSession;
use futures::future::{BoxFuture, FutureExt};
use slack_morphism_models::common::*;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiUsersListRequest {
    pub cursor: Option<SlackCursorId>,
    pub include_locale: Option<bool>,
    pub limit: Option<u16>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiUsersListResponse {
    pub members: Vec<SlackUser>,
    pub response_metadata: Option<SlackResponseMetadata>,
}

impl<'a> SlackClientSession<'a> {

    pub async fn users_list(
        &self,
        req: &SlackApiUsersListRequest,
    ) -> ClientResult<SlackApiUsersListResponse> {
        self.http_get(
            "users.list",
            vec![
                ("cursor", req.cursor.as_ref().map(|c| c.value().into())),
                ("limit", req.limit.map(|v| v.to_string())),
                (
                    "include_locale",
                    req.include_locale.map(|v| v.to_string()),
                ),
            ],
        )
        .await
    }

}

impl SlackApiScrollableRequest for SlackApiUsersListRequest {
    type ResponseType = SlackApiUsersListResponse;
    type CursorType = SlackCursorId;

    fn with_new_cursor(&self, new_cursor: Option<&Self::CursorType>) -> Self {
        Self {
            cursor: new_cursor.cloned(),
            ..*self
        }
    }

    fn scroll<'a, 's>(
        &'a self,
        session: &'a SlackClientSession<'s>,
    ) -> BoxFuture<'a, ClientResult<Self::ResponseType>> {
        async move { session.users_list(&self).await }.boxed()
    }
}

impl SlackApiScrollableResponse for SlackApiUsersListResponse {
    type CursorType = SlackCursorId;

    fn next_cursor(&self) -> Option<&Self::CursorType> {
        self.response_metadata
            .as_ref()
            .map(|rm| rm.next_cursor.as_ref())
            .flatten()
    }
}
