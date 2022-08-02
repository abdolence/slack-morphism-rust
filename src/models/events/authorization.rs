use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::*;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackEventAuthorization {
    pub team_id: SlackTeamId,
    pub user_id: SlackUserId,
    pub is_bot: Option<bool>,
}
