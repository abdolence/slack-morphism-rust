use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::ClientResult;
use crate::SlackClientSession;
use slack_morphism_models::common::*;

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
                (
                    "cursor".into(),
                    req.cursor.as_ref().map(|c| c.value().into()),
                ),
                ("limit".into(), req.limit.map(|v| v.to_string()).clone()),
                (
                    "include_locale".into(),
                    req.include_locale.map(|v| v.to_string()).clone(),
                ),
            ],
        )
        .await
    }
}
