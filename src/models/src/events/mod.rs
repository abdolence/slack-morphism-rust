use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::common::*;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SlackPushEvent {
    #[serde(rename = "url_verification")]
    UrlVerification(SlackUrlVerificationEvent),
    #[serde(rename = "event_callback")]
    EventCallback(SlackEventCallback),
    #[serde(rename = "app_rate_limited")]
    AppRateLimited(SlackAppRateLimitedEvent)
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackUrlVerificationEvent {
    pub challenge: String
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackAppRateLimitedEvent {
    pub team_id: String,
    pub minute_rate_limited: SlackDateTime,
    pub api_app_id: String
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct  SlackEventCallback {
    pub team_id: SlackTeamId,
    pub api_app_id: SlackAppId,
    pub event: SlackEventCallbackBody,
    pub event_id: SlackEventId,
    pub event_time: SlackDateTime,
    pub authed_users: Option<Vec<SlackUserId>>
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SlackEventCallbackBody {

}