//!
//! Support for Slack Views API methods
//!

use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::ratectl::*;
use crate::SlackClientSession;
use crate::{ClientResult, SlackClientHttpConnector};
use slack_morphism_models::blocks::*;
use slack_morphism_models::*;

impl<'a, SCHC> SlackClientSession<'a, SCHC>
where
    SCHC: SlackClientHttpConnector + Send,
{
    ///
    /// https://api.slack.com/methods/views.open
    ///
    pub async fn views_open(
        &self,
        req: &SlackApiViewsOpenRequest,
    ) -> ClientResult<SlackApiViewsOpenResponse> {
        self.http_session_api
            .http_post("views.open", req, Some(&SLACK_TIER4_METHOD_CONFIG))
            .await
    }

    ///
    /// https://api.slack.com/methods/views.publish
    ///
    pub async fn views_publish(
        &self,
        req: &SlackApiViewsPublishRequest,
    ) -> ClientResult<SlackApiViewsPublishResponse> {
        self.http_session_api
            .http_post("views.publish", req, Some(&SLACK_TIER4_METHOD_CONFIG))
            .await
    }

    ///
    /// https://api.slack.com/methods/views.push
    ///
    pub async fn views_push(
        &self,
        req: &SlackApiViewsPushRequest,
    ) -> ClientResult<SlackApiViewsPushResponse> {
        self.http_session_api
            .http_post("views.push", req, Some(&SLACK_TIER4_METHOD_CONFIG))
            .await
    }

    ///
    /// https://api.slack.com/methods/views.update
    ///
    pub async fn views_update(
        &self,
        req: &SlackApiViewsUpdateRequest,
    ) -> ClientResult<SlackApiViewsUpdateResponse> {
        self.http_session_api
            .http_post("views.update", req, Some(&SLACK_TIER4_METHOD_CONFIG))
            .await
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiViewsOpenRequest {
    pub trigger_id: SlackTriggerId,
    pub view: SlackView,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiViewsOpenResponse {
    pub view: SlackStatefulView,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiViewsPublishRequest {
    pub user_id: SlackUserId,
    pub view: SlackView,
    pub hash: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiViewsPublishResponse {
    pub view: SlackStatefulView,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiViewsPushRequest {
    pub trigger_id: SlackTriggerId,
    pub view: SlackView,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiViewsPushResponse {
    pub view: SlackStatefulView,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiViewsUpdateRequest {
    pub view: SlackView,
    pub external_id: Option<String>,
    pub hash: Option<String>,
    pub view_id: Option<SlackViewId>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiViewsUpdateResponse {
    pub view: SlackStatefulView,
}
