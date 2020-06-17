use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use crate::blocks::kit::SlackBlock;
use crate::common::SlackCallbackId;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SlackView {
    #[serde(rename = "home")]
    Home(SlackHomeView)
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackHomeView{
    blocks: Vec<SlackBlock>,
    #[serde(with = "serde_with::rust::string_empty_as_none")]
    private_metadata: Option<String>,
    #[serde(with = "serde_with::rust::string_empty_as_none")]
    callback_id: Option<SlackCallbackId>,
    #[serde(with = "serde_with::rust::string_empty_as_none")]
    external_id: Option<String>
}