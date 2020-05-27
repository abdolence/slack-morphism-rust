use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

use crate::ClientResult;
use crate::SlackClient;
use crate::SlackClientSession;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiTestRequest {
    pub error: Option<String>,
    pub foo: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiTestResponse {
    args: Option<HashMap<String, String>>,
}

impl<'a> SlackClientSession<'a> {
    pub async fn api_test(&self, req: &SlackApiTestRequest) -> ClientResult<SlackApiTestResponse> {
        let full_uri = SlackClient::create_url_with_params(
            &SlackClient::create_method_uri_path("api.test"),
            vec![("foo", req.foo.clone()), ("error", req.error.clone())],
        );
        self.http_post_uri(full_uri, &req).await
    }
}
