use crate::blocks::kit::*;
use crate::common::*;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use rsb_derive::Builder;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackMessageOrigin {
    pub ts: SlackTs,
    pub channel: Option<SlackChannelId>,
    pub channel_type: Option<SlackChannelType>,
    pub thread_ts: Option<SlackTs>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackMessageContent {
    pub text: String,
    pub blocks: Option<Vec<SlackBlock>>,
}
