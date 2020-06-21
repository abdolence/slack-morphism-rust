//!
//! Support for Slack Team API methods
//!

use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::ClientResult;
use crate::SlackClientSession;
use slack_morphism_models::*;

impl<'a> SlackClientSession<'a> {
    ///
    /// https://api.slack.com/methods/team.info
    ///
    pub async fn team_info(
        &self,
        req: &SlackApiTeamInfoRequest,
    ) -> ClientResult<SlackApiTeamInfoResponse> {
        self.http_api
            .http_get("team.info", &vec![("team", req.team.as_ref())])
            .await
    }

    ///
    /// https://api.slack.com/methods/team.profile.get
    ///
    pub async fn team_profile_get(
        &self,
        req: &SlackApiTeamProfileGetRequest,
    ) -> ClientResult<SlackApiTeamProfileGetResponse> {
        self.http_api
            .http_get(
                "team.profile.get",
                &vec![("visibility", req.visibility.as_ref())],
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
