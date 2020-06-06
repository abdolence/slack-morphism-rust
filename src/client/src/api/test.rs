use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

use crate::SlackClientSession;
use crate::{ClientResult, SlackClientHttpApi};

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
        let full_uri = SlackClientHttpApi::create_url_with_params(
            &SlackClientHttpApi::create_method_uri_path("api.test"),
            &vec![("foo", req.foo.as_ref()), ("error", req.error.as_ref())],
        );
        self.http_api.http_post_uri(full_uri, &req).await
    }
}
