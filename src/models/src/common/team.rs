use crate::common::*;

use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackTeamInfo {
    pub id: SlackTeamId,
    pub name: Option<String>,
    pub domain: Option<String>,
    pub email_domain: Option<String>,
    pub icon: Option<SlackIcon>,
}
