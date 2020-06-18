//!
//! Support for Slack Conversations API methods
//!

use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::ClientResult;
use crate::SlackClientSession;
use slack_morphism_models::SlackChannelId;

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
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiConversationsArchiveRequest {
    pub channel: SlackChannelId,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiConversationsArchiveResponse {}
