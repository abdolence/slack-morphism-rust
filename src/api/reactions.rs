//!
//! Support for Slack Reactions API methods
//!

use rsb_derive::Builder;
use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use url::Url;

use crate::models::*;
use crate::ratectl::*;
use crate::SlackClientSession;
use crate::{ClientResult, SlackClientHttpConnector};

impl<'a, SCHC> SlackClientSession<'a, SCHC>
where
    SCHC: SlackClientHttpConnector + Send,
{
    ///
    /// https://api.slack.com/methods/reactions.get
    ///
    pub async fn reactions_get(
        &self,
        req: &SlackApiReactionsGetRequest,
    ) -> ClientResult<SlackApiReactionsGetResponse> {
        self.http_session_api
            .http_get(
                "reactions.get",
                &vec![
                    ("channel", req.channel.as_ref().map(|x| x.value())),
                    ("file", req.file.as_ref().map(|x| x.value())),
                    ("full", req.full.map(|v| v.to_string()).as_ref()),
                    ("timestamp", req.timestamp.as_ref().map(|x| x.value())),
                ],
                Some(&SLACK_TIER3_METHOD_CONFIG),
            )
            .await
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiReactionsGetRequest {
    pub channel: Option<SlackChannelId>,
    pub file: Option<SlackFileId>,
    pub full: Option<bool>,
    pub timestamp: Option<SlackTs>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiReactionsGetMessageResponse {
    #[serde(flatten)]
    pub message: SlackHistoryMessage,
    pub permalink: Url,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[allow(clippy::large_enum_variant)]
pub enum SlackApiReactionsGetResponse {
    Message(SlackApiReactionsGetMessageResponse),
    File(SlackFile),
}
