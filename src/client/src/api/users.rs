use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::scroller::*;
use crate::ClientResult;
use crate::SlackClientSession;
use slack_morphism_models::common::*;
use futures::future::{FutureExt, BoxFuture};

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiUsersListRequest {
    cursor: Option<SlackCursorId>,
    include_locale: Option<bool>,
    limit: Option<u16>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiUsersListResponse {
    members: Vec<SlackUser>,
    response_metadata: Option<SlackResponseMetadata>,
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
                ("limit", req.limit.map(|v| v.to_string()).clone()),
                (
                    "include_locale",
                    req.include_locale.map(|v| v.to_string()).clone(),
                ),
            ],
        )
        .await
    }

}

impl<'a> SlackApiResponseScrollerFactory for SlackApiUsersListRequest {
    type ResponseType = SlackApiUsersListResponse;
    type CursorType = SlackCursorId;

    fn scroller(
        &self,
    ) -> Box<
        dyn SlackApiResponseScroller<
            ResponseType = Self::ResponseType,
            CursorType = Self::CursorType,
        >,
    > {
        Box::new(SlackApiResponseScrollerState {
            request: self.clone(),
            last_cursor: None,
            last_response: None,
        })
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

    fn scroll<'a,'s>(
        &'a self,
        session: &'a SlackClientSession<'s>,
    ) -> BoxFuture<'a,ClientResult<Self::ResponseType>> {
        let async_res = async move {
            session.users_list(&self).await
        };
        async_res.boxed()
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
