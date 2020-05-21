use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::common::*;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SlackPushEvent {
    #[serde(rename = "url_verification")]
    UrlVerification(SlackUrlVerificationEvent),
    AppRateLimited(SlackAppRateLimitedEvent)
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackUrlVerificationEvent {
    challenge: String
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackAppRateLimitedEvent {
    team_id: String,
    minute_rate_limited: SlackDateTime,
    api_app_id: String
}
