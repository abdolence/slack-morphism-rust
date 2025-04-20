use crate::ratectl::{
    SlackApiMethodRateControlConfig, SlackApiRateControlLimit, SlackApiRateControlSpecialLimit,
    SLACK_TIER4_METHOD_CONFIG,
};
use crate::{
    ClientResult, SlackAssistantPrompt, SlackChannelId, SlackClientHttpConnector,
    SlackClientSession, SlackTs,
};
use lazy_static::lazy_static;
use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

impl<'a, SCHC> SlackClientSession<'a, SCHC>
where
    SCHC: SlackClientHttpConnector + Send,
{
    /// https://api.slack.com/methods/assistant.threads.setStatus
    pub async fn assistant_threads_set_status(
        &self,
        req: &SlackApiAssistantThreadsSetStatusRequest,
    ) -> ClientResult<SlackApiAssistantThreadsSetStatusResponse> {
        self.http_session_api
            .http_post(
                "assistant.threads.setStatus",
                req,
                Some(&ASSISTANT_THREAD_SET_STATUS_SPECIAL_LIMIT_RATE_CTL),
            )
            .await
    }

    /// https://api.slack.com/methods/assistant.threads.setSuggestedPrompts
    pub async fn assistant_threads_set_suggested_prompts(
        &self,
        req: &SlackApiAssistantThreadsSetSuggestedPromptsRequest,
    ) -> ClientResult<SlackApiAssistantThreadsSetSuggestedPromptsResponse> {
        self.http_session_api
            .http_post(
                "assistant.threads.setSuggestedPrompts",
                req,
                Some(&SLACK_TIER4_METHOD_CONFIG),
            )
            .await
    }

    /// https://api.slack.com/methods/assistant.threads.setTitle
    pub async fn assistant_threads_set_title(
        &self,
        req: &SlackApiAssistantThreadSetTitleRequest,
    ) -> ClientResult<SlackApiAssistantThreadSetTitleResponse> {
        self.http_session_api
            .http_post(
                "assistant.threads.setTitle",
                req,
                Some(&SLACK_TIER4_METHOD_CONFIG),
            )
            .await
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiAssistantThreadsSetStatusRequest {
    pub channel_id: SlackChannelId,
    pub status: String,
    pub thread_ts: SlackTs,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiAssistantThreadsSetStatusResponse {}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiAssistantThreadsSetSuggestedPromptsRequest {
    pub channel_id: SlackChannelId,
    pub thread_ts: SlackTs,
    pub prompts: Vec<SlackAssistantPrompt>,
    pub title: Option<String>,
}
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiAssistantThreadsSetSuggestedPromptsResponse {}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiAssistantThreadSetTitleRequest {
    pub channel_id: SlackChannelId,
    pub thread_ts: SlackTs,
    pub title: String,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiAssistantThreadSetTitleResponse {}

lazy_static! {
    pub static ref ASSISTANT_THREAD_SET_STATUS_SPECIAL_LIMIT_RATE_CTL: SlackApiMethodRateControlConfig =
        SlackApiMethodRateControlConfig::new().with_special_rate_limit(
            SlackApiRateControlSpecialLimit::new(
                "assistant.threads.setStatus".into(),
                SlackApiRateControlLimit::new(1, std::time::Duration::from_secs(1))
            )
        );
}
