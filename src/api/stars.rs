//!
//! Support for Slack Stars API methods
//!

use rsb_derive::Builder;
use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::*;
use crate::ratectl::*;
use crate::*;
use futures::future::{BoxFuture, FutureExt};
use std::collections::HashSet;

impl<'a, SCHC> SlackClientSession<'a, SCHC>
where
    SCHC: SlackClientHttpConnector + Send,
{
    ///
    /// https://api.slack.com/methods/stars.add
    ///
    pub async fn stars_add(
        &self,
        req: &SlackApiStarsAddRequest,
    ) -> ClientResult<SlackApiStarsAddResponse> {
        self.http_session_api
            .http_post("stars.add", req, Some(&SLACK_TIER2_METHOD_CONFIG))
            .await
    }

    ///
    /// https://api.slack.com/methods/stars.remove
    ///
    pub async fn stars_remove(
        &self,
        req: &SlackApiStarsRemoveRequest,
    ) -> ClientResult<SlackApiStarsRemoveResponse> {
        self.http_session_api
            .http_post("stars.remove", req, Some(&SLACK_TIER2_METHOD_CONFIG))
            .await
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiStarsAddRequest {
    pub channel: Option<SlackChannelId>,
    pub file: Option<SlackFileId>,
    pub file_comment: Option<SlackFileCommentId>,
    pub timestamp: Option<SlackTs>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiStarsAddResponse {}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiStarsRemoveRequest {
    pub channel: Option<SlackChannelId>,
    pub file: Option<SlackFileId>,
    pub file_comment: Option<SlackFileCommentId>,
    pub timestamp: Option<SlackTs>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiStarsRemoveResponse {}
