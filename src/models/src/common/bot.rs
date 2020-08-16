use crate::common::*;

use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBotInfo {
    pub id: Option<SlackBotId>,
    pub name: String,
    pub updated: Option<SlackDateTime>,
    pub app_id: String,
    pub user_id: String,
    #[serde(flatten)]
    pub icons: Option<SlackIconImages>
}