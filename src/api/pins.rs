use rsb_derive::Builder;
use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    ratectl::SLACK_TIER2_METHOD_CONFIG, ClientResult, SlackChannelId, SlackClientHttpConnector,
    SlackClientSession, SlackMessage, SlackPin, SlackTs,
};

impl<'a, SCHC> SlackClientSession<'a, SCHC>
where
    SCHC: SlackClientHttpConnector + Send,
{
    ///
    /// https://api.slack.com/methods/pins.add
    ///
    pub async fn pins_add(
        &self,
        req: &SlackApiPinsAddRequest,
    ) -> ClientResult<SlackApiPinsAddResponse> {
        self.http_session_api
            .http_post("pins.add", req, Some(&SLACK_TIER2_METHOD_CONFIG))
            .await
    }

    ///
    /// https://api.slack.com/methods/pins.list
    ///
    pub async fn pins_list(
        &self,
        req: &SlackApiPinsListRequest,
    ) -> ClientResult<SlackApiPinsListResponse> {
        self.http_session_api
            .http_get(
                "pins.list",
                &[("channel", Some(&req.channel.value()))],
                Some(&SLACK_TIER2_METHOD_CONFIG),
            )
            .await
    }

    ///
    /// https://api.slack.com/methods/pins.remove
    ///
    pub async fn pins_remove(
        &self,
        req: &SlackApiPinsRemoveRequest,
    ) -> ClientResult<SlackApiPinsRemoveResponse> {
        self.http_session_api
            .http_post("pins.remove", req, Some(&SLACK_TIER2_METHOD_CONFIG))
            .await
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiPinsAddRequest {
    pub channel: SlackChannelId,
    pub timestamp: SlackTs,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiPinsAddResponse {}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiPinsListRequest {
    pub channel: SlackChannelId,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiPinsListResponse {
    items: Vec<SlackPin>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiPinsRemoveRequest {
    pub channel: SlackChannelId,
    pub timestamp: SlackTs,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiPinsRemoveResponse {}
