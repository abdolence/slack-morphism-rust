//!
//! Support for Slack User Groups API methods
//!

use rsb_derive::Builder;
use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::*;
use crate::ratectl::*;
use crate::SlackClientSession;
use crate::{ClientResult, SlackClientHttpConnector};

impl<'a, SCHC> SlackClientSession<'a, SCHC>
where
    SCHC: SlackClientHttpConnector + Send,
{
    ///
    /// https://api.slack.com/methods/usergroups.list
    ///
    pub async fn usergroups_list(
        &self,
        req: &SlackApiUserGroupsListRequest,
    ) -> ClientResult<SlackApiUserGroupsListResponse> {
        self.http_session_api
            .http_get(
                "usergroups.list",
                &vec![
                    (
                        "include_count",
                        req.include_count.map(|v| v.to_string()).as_ref(),
                    ),
                    (
                        "include_disabled",
                        req.include_disabled.map(|v| v.to_string()).as_ref(),
                    ),
                    (
                        "include_users",
                        req.include_users.map(|v| v.to_string()).as_ref(),
                    ),
                    ("team_id", req.team_id.as_ref().map(|x| x.value())),
                ],
                Some(&SLACK_TIER2_METHOD_CONFIG),
            )
            .await
    }

    ///
    /// https://api.slack.com/methods/usergroups.users.list
    ///
    pub async fn usergroups_users_list(
        &self,
        req: &SlackApiUserGroupsUsersListRequest,
    ) -> ClientResult<SlackApiUserGroupsUsersListResponse> {
        self.http_session_api
            .http_get(
                "usergroups.users.list",
                &vec![
                    (
                        "include_disabled",
                        req.include_disabled.map(|v| v.to_string()).as_ref(),
                    ),
                    ("team_id", req.team_id.as_ref().map(|x| x.value())),
                ],
                Some(&SLACK_TIER2_METHOD_CONFIG),
            )
            .await
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiUserGroupsListRequest {
    pub include_count: Option<bool>,
    pub include_disabled: Option<bool>,
    pub include_users: Option<bool>,
    pub team_id: Option<SlackTeamId>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiUserGroupsListResponse {
    pub usergroups: Vec<SlackUserGroup>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiUserGroupsUsersListRequest {
    pub usergroup: SlackUserGroupId,
    pub include_disabled: Option<bool>,
    pub team_id: Option<SlackTeamId>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiUserGroupsUsersListResponse {
    pub users: Vec<SlackUserId>,
}
