use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

use crate::SlackClientSession;
use crate::ClientResult;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiTestRequest {
    pub error: Option<String>,
    pub foo: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiTestResponse {
    args: Option<Vec<HashMap<String, String>>>,
}

impl <'a> SlackClientSession<'a> {

    pub async fn api_test(&self, req : &SlackApiTestRequest) -> ClientResult<SlackApiTestResponse> {
        self.post(
            "api.test",
            &req
        ).await
    }

}