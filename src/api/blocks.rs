//!
//! Support for Slack blocks.validate method
//!

use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none};

use crate::errors::{map_serde_error, SlackClientError};
use crate::models::blocks::*;
use crate::models::*;
use crate::SlackClientSession;
use crate::{ClientResult, SlackClientHttpConnector};

const BLOCKS_VALIDATE_FAILURE_CODES: [&str; 3] =
    ["invalid_blocks", "invalid_message", "invalid_view"];

impl<'a, SCHC> SlackClientSession<'a, SCHC>
where
    SCHC: SlackClientHttpConnector + Send,
{
    ///
    /// https://docs.slack.dev/reference/methods/blocks.validate
    ///
    /// A payload that fails validation comes back as `ok: false` with `error`
    /// set to `invalid_blocks` / `invalid_message` / `invalid_view` and the
    /// details in `errors`. The generic envelope turns every `ok: false` into
    /// `SlackClientError::ApiError`, so this method intercepts those three
    /// codes and re-parses the recorded response body into
    /// [`SlackApiBlocksValidateResponse`]; the caller inspects `ok` and
    /// `errors`. Any other error (`invalid_arguments`, auth, ...) is still
    /// returned as `Err`.
    ///
    /// The docs state only that "special rate limits apply" with no figure,
    /// so no client-side throttle is configured for this method.
    ///
    pub async fn blocks_validate(
        &self,
        req: &SlackApiBlocksValidateRequest,
    ) -> ClientResult<SlackApiBlocksValidateResponse> {
        match self
            .http_session_api
            .http_post("blocks.validate", req, None)
            .await
        {
            Err(SlackClientError::ApiError(api_error))
                if BLOCKS_VALIDATE_FAILURE_CODES.contains(&api_error.code.as_str()) =>
            {
                match api_error.http_response_body.as_deref() {
                    Some(body) => {
                        serde_json::from_str(body).map_err(|e| map_serde_error(e, Some(body)))
                    }
                    None => Err(SlackClientError::ApiError(api_error)),
                }
            }
            other => other,
        }
    }
}

/// Exactly one of `blocks`, `message` or `view` must be set. Slack wants each
/// as a JSON-encoded string inside the request body, which
/// `serde_with::json::JsonString` produces.
#[serde_as]
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiBlocksValidateRequest {
    #[serde_as(as = "Option<serde_with::json::JsonString>")]
    pub blocks: Option<Vec<SlackBlock>>,
    #[serde_as(as = "Option<serde_with::json::JsonString>")]
    pub message: Option<SlackMessageContent>,
    #[serde_as(as = "Option<serde_with::json::JsonString>")]
    pub view: Option<SlackView>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiBlocksValidateResponse {
    pub ok: bool,
    /// Set when `ok` is false.
    pub error: Option<SlackApiBlocksValidateErrorKind>,
    pub errors: Option<Vec<SlackApiBlocksValidateError>>,
}

/// Which payload failed validation.
/// https://docs.slack.dev/reference/methods/blocks.validate#errors
#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SlackApiBlocksValidateErrorKind {
    InvalidBlocks,
    InvalidMessage,
    InvalidView,
    /// A value this crate does not model yet, carried verbatim.
    #[serde(untagged)]
    Other(String),
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiBlocksValidateError {
    /// JSON pointer to the invalid element, e.g. `/blocks/0/child_blocks/0/type`.
    pub pointer: String,
    /// e.g. `failed_constraint`.
    pub code: String,
    pub message: String,
    /// Shape depends on `code`, e.g. `{"type":"enum","expected":[...],"got":"..."}`.
    pub constraint: Option<serde_json::Value>,
    pub field: Option<String>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_slack_api_blocks_validate_request_encodes_payload_as_json_string() {
        let req =
            SlackApiBlocksValidateRequest::new().with_blocks(vec![SlackDividerBlock::new().into()]);
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(
            json,
            serde_json::json!({ "blocks": "[{\"type\":\"divider\"}]" })
        );
        let decoded: SlackApiBlocksValidateRequest = serde_json::from_value(json).unwrap();
        assert_eq!(decoded, req);
    }

    #[test]
    fn test_slack_api_blocks_validate_error_response_round_trip() {
        let payload = r#"{"ok":false,"error":"invalid_message","errors":[{"code":"failed_constraint","constraint":{"type":"enum","expected":["divider","image","section"],"got":"markdown"},"message":"unsupported type: markdown","pointer":"/blocks/0/child_blocks/0/type"}]}"#;

        let envelope: crate::SlackEnvelopeMessage = serde_json::from_str(payload).unwrap();
        assert_eq!(envelope.error.as_deref(), Some("invalid_message"));
        assert!(envelope.errors.unwrap()[0].contains("failed_constraint"));

        let response: SlackApiBlocksValidateResponse = serde_json::from_str(payload).unwrap();
        assert!(!response.ok);
        assert_eq!(
            response.error,
            Some(SlackApiBlocksValidateErrorKind::InvalidMessage)
        );
        let errors = response.errors.as_ref().unwrap();
        assert_eq!(errors[0].pointer, "/blocks/0/child_blocks/0/type");
        assert_eq!(errors[0].constraint.as_ref().unwrap()["got"], "markdown");
        assert_eq!(errors[0].field, None);
        assert_eq!(
            serde_json::to_value(&response).unwrap(),
            serde_json::from_str::<serde_json::Value>(payload).unwrap()
        );

        let ok: SlackApiBlocksValidateResponse = serde_json::from_str(r#"{"ok":true}"#).unwrap();
        assert!(ok.ok && ok.errors.is_none());

        let other: SlackApiBlocksValidateErrorKind =
            serde_json::from_str(r#""something_new""#).unwrap();
        assert_eq!(
            other,
            SlackApiBlocksValidateErrorKind::Other("something_new".into())
        );
        assert_eq!(serde_json::to_string(&other).unwrap(), r#""something_new""#);
    }
}
