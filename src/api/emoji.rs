//!
//! Support for Slack Team API methods
//!

use std::collections::HashMap;

use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::*;
use crate::ratectl::*;
use crate::SlackClientSession;
use crate::{ClientResult, SlackClientHttpConnector};

impl<'a, SCHC> SlackClientSession<'a, SCHC>
where
    SCHC: SlackClientHttpConnector + Send,
{
    ///
    /// https://api.slack.com/methods/emoji.list
    ///
    pub async fn emoji_list(&self) -> ClientResult<SlackApiEmojiListResponse> {
        self.http_session_api
            .http_get(
                "emoji.list",
                &crate::client::SLACK_HTTP_EMPTY_GET_PARAMS.clone(),
                Some(&SLACK_TIER2_METHOD_CONFIG),
            )
            .await
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiEmojiListResponse {
    pub emoji: HashMap<SlackEmojiName, SlackEmojiRef>,
}
