//!
//! Support for Slack Webhooks methods
//!

use lazy_static::lazy_static;
use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use url::Url;

use crate::models::*;
use crate::ratectl::*;
use crate::SlackClient;
use crate::{ClientResult, SlackClientHttpConnector};
use rvstruct::ValueStruct;
use tracing::*;

impl<SCHC> SlackClient<SCHC>
where
    SCHC: SlackClientHttpConnector + Send,
{
    ///
    /// Post a webhook message using webhook url
    ///
    pub async fn post_webhook_message(
        &self,
        incoming_webhook_url: &Url,
        req: &SlackApiPostWebhookMessageRequest,
    ) -> ClientResult<SlackApiPostWebhookMessageResponse> {
        let http_webhook_span = span!(Level::DEBUG, "Slack WebHook");

        let context = crate::SlackClientApiCallContext {
            rate_control_params: Some(&POST_WEBHOOK_SPECIAL_LIMIT_RATE_CTL),
            token: None,
            tracing_span: &http_webhook_span,
            is_sensitive_url: true,
        };

        self.http_api
            .connector
            .http_post_uri(incoming_webhook_url.clone(), req, context)
            .await
    }

    //
    // Respond to event using a Slack ResponseURL and providing message.
    //
    pub async fn respond_to_event(
        &self,
        response_url: &SlackResponseUrl,
        req: &SlackApiPostWebhookMessageRequest,
    ) -> ClientResult<SlackApiPostWebhookMessageResponse> {
        self.post_webhook_message(response_url.value(), req).await
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

lazy_static! {
    pub static ref POST_WEBHOOK_SPECIAL_LIMIT_RATE_CTL: SlackApiMethodRateControlConfig =
        SlackApiMethodRateControlConfig::new().with_special_rate_limit(
            SlackApiRateControlSpecialLimit::new(
                "post_webhook_message".into(),
                SlackApiRateControlLimit::new(1, std::time::Duration::from_secs(1))
            )
        );
}
