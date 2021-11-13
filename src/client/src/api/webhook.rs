//!
//! Support for Slack Webhooks methods
//!

use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::errors::*;
use crate::SlackClient;
use crate::{ClientResult, SlackClientHttpConnector};
use slack_morphism_models::*;

impl<SCHC> SlackClient<SCHC>
where
    SCHC: SlackClientHttpConnector + Send,
{
    ///
    /// Post a webhook message using webhook url
    ///
    pub async fn post_webhook_message(
        &self,
        hook_url: &str,
        req: &SlackApiPostWebhookMessageRequest,
    ) -> ClientResult<SlackApiPostWebhookMessageResponse> {
        self.http_api
            .connector
            .http_post_uri(
                hook_url.parse().map_err(|cause| {
                    SlackClientError::SystemError(
                        SlackClientSystemError::new().with_cause(Box::new(cause)),
                    )
                })?,
                req,
                None,
            )
            .await
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiPostWebhookMessageRequest {
    #[serde(flatten)]
    pub content: SlackMessageContent,
    pub thread_ts: Option<SlackTs>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiPostWebhookMessageResponse {}
