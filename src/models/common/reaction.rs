use crate::*;

use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackReaction {
    pub name: String,
    pub count: usize,
    pub users: Vec<SlackUserId>,
}
