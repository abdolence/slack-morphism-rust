use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{SlackChannelId, SlackDateTime, SlackFile, SlackMessage, SlackUserId};

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackPin {
    pub channel: SlackChannelId,
    pub created: SlackDateTime,
    pub created_by: SlackUserId,
    pub message: Option<SlackMessage>,
}
