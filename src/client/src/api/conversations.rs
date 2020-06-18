//!
//! Support for Slack Conversations API methods
//!

use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::ClientResult;
use crate::SlackClientSession;
use slack_morphism_models::*;
use std::collections::HashSet;

impl<'a> SlackClientSession<'a> {
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
