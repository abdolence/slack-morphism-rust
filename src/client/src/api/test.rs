//!
//! Support for Slack test API methods
//!

use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

use crate::{ClientResult, SlackClientHttpApiUri, SlackClientHttpConnector, SlackClientSession};

impl<'a, SCHC> SlackClientSession<'a, SCHC>
where
    SCHC: SlackClientHttpConnector + Send,
{
    ///
    /// https://api.slack.com/methods/api.test
    ///
    pub async fn api_test(&self, req: &SlackApiTestRequest) -> ClientResult<SlackApiTestResponse> {
        let full_uri = SlackClientHttpApiUri::create_url_with_params(
            &SlackClientHttpApiUri::create_method_uri_path("api.test"),
            &vec![("foo", req.foo.as_ref()), ("error", req.error.as_ref())],
        );
        self.http_session_api.http_post_uri(full_uri, &req).await
    }
}

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
