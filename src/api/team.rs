//!
//! Support for Slack Team API methods
//!

use rsb_derive::Builder;
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
    /// https://api.slack.com/methods/team.info
    ///
    pub async fn team_info(
        &self,
        req: &SlackApiTeamInfoRequest,
    ) -> ClientResult<SlackApiTeamInfoResponse> {
        self.http_session_api
            .http_get(
                "team.info",
                &vec![("team", req.team.as_ref())],
                Some(&SLACK_TIER3_METHOD_CONFIG),
            )
            .await
    }

    ///
    /// https://api.slack.com/methods/team.profile.get
    ///
    pub async fn team_profile_get(
        &self,
        req: &SlackApiTeamProfileGetRequest,
    ) -> ClientResult<SlackApiTeamProfileGetResponse> {
        self.http_session_api
            .http_get(
                "team.profile.get",
                &vec![("visibility", req.visibility.as_ref())],
                Some(&SLACK_TIER3_METHOD_CONFIG),
            )
            .await
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiTeamInfoRequest {
    pub team: Option<SlackTeamId>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiTeamInfoResponse {
    pub team: SlackTeamInfo,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiTeamProfileGetRequest {
    pub visibility: Option<SlackTeamId>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiTeamProfileGetResponse {
    pub profile: SlackTeamProfile,
}
