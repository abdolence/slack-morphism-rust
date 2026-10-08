//!
//! Support for Slack canvases.* and conversations.canvases.create methods
//!

use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::*;
use crate::ratectl::*;
use crate::SlackClientSession;
use crate::{ClientResult, SlackClientHttpConnector};

impl<'a, SCHC> SlackClientSession<'a, SCHC>
where
    SCHC: SlackClientHttpConnector + Send,
{
    ///
    /// https://docs.slack.dev/reference/methods/canvases.create
    ///
    pub async fn canvases_create(
        &self,
        req: &SlackApiCanvasesCreateRequest,
    ) -> ClientResult<SlackApiCanvasesCreateResponse> {
        self.http_session_api
            .http_post("canvases.create", req, Some(&SLACK_TIER2_METHOD_CONFIG))
            .await
    }

    ///
    /// https://docs.slack.dev/reference/methods/canvases.edit
    ///
    pub async fn canvases_edit(
        &self,
        req: &SlackApiCanvasesEditRequest,
    ) -> ClientResult<SlackApiCanvasesEditResponse> {
        self.http_session_api
            .http_post("canvases.edit", req, Some(&SLACK_TIER3_METHOD_CONFIG))
            .await
    }

    ///
    /// https://docs.slack.dev/reference/methods/canvases.delete
    ///
    pub async fn canvases_delete(
        &self,
        req: &SlackApiCanvasesDeleteRequest,
    ) -> ClientResult<SlackApiCanvasesDeleteResponse> {
        self.http_session_api
            .http_post("canvases.delete", req, Some(&SLACK_TIER3_METHOD_CONFIG))
            .await
    }

    ///
    /// https://docs.slack.dev/reference/methods/canvases.sections.lookup
    ///
    pub async fn canvases_sections_lookup(
        &self,
        req: &SlackApiCanvasesSectionsLookupRequest,
    ) -> ClientResult<SlackApiCanvasesSectionsLookupResponse> {
        self.http_session_api
            .http_post(
                "canvases.sections.lookup",
                req,
                Some(&SLACK_TIER3_METHOD_CONFIG),
            )
            .await
    }

    ///
    /// https://docs.slack.dev/reference/methods/canvases.access.set
    ///
    pub async fn canvases_access_set(
        &self,
        req: &SlackApiCanvasesAccessSetRequest,
    ) -> ClientResult<SlackApiCanvasesAccessSetResponse> {
        self.http_session_api
            .http_post("canvases.access.set", req, Some(&SLACK_TIER3_METHOD_CONFIG))
            .await
    }

    ///
    /// https://docs.slack.dev/reference/methods/canvases.access.delete
    ///
    pub async fn canvases_access_delete(
        &self,
        req: &SlackApiCanvasesAccessDeleteRequest,
    ) -> ClientResult<SlackApiCanvasesAccessDeleteResponse> {
        self.http_session_api
            .http_post(
                "canvases.access.delete",
                req,
                Some(&SLACK_TIER3_METHOD_CONFIG),
            )
            .await
    }

    ///
    /// https://docs.slack.dev/reference/methods/conversations.canvases.create
    ///
    pub async fn conversations_canvases_create(
        &self,
        req: &SlackApiConversationsCanvasesCreateRequest,
    ) -> ClientResult<SlackApiConversationsCanvasesCreateResponse> {
        self.http_session_api
            .http_post(
                "conversations.canvases.create",
                req,
                Some(&SLACK_TIER2_METHOD_CONFIG),
            )
            .await
    }
}

/// Canvas content, serialised as `{"type":"markdown","markdown":"..."}`.
/// Markdown is limited to 1 MiB; see the method pages for the supported elements.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
#[serde(tag = "type", rename = "markdown")]
pub struct SlackCanvasDocumentContent {
    pub markdown: String,
}

/// `channel_id` tabs the canvas in that channel and is required for free teams.
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiCanvasesCreateRequest {
    pub title: Option<String>,
    pub document_content: Option<SlackCanvasDocumentContent>,
    pub channel_id: Option<SlackChannelId>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiCanvasesCreateResponse {
    pub canvas_id: SlackCanvasId,
}

/// `changes` currently takes exactly one change per call.
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiCanvasesEditRequest {
    pub canvas_id: SlackCanvasId,
    pub changes: Vec<SlackCanvasChange>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiCanvasesEditResponse {}

/// One edit. `insert_before` / `insert_after` / `delete` need `section_id`;
/// every insert and `replace` need `document_content` (`replace` without a
/// `section_id` replaces the whole canvas); `rename` takes `title_content`.
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackCanvasChange {
    pub operation: SlackCanvasOperation,
    pub section_id: Option<SlackCanvasSectionId>,
    pub document_content: Option<SlackCanvasDocumentContent>,
    pub title_content: Option<SlackCanvasDocumentContent>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SlackCanvasOperation {
    InsertAfter,
    InsertBefore,
    InsertAtStart,
    InsertAtEnd,
    Replace,
    Delete,
    Rename,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiCanvasesDeleteRequest {
    pub canvas_id: SlackCanvasId,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiCanvasesDeleteResponse {}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiCanvasesSectionsLookupRequest {
    pub canvas_id: SlackCanvasId,
    pub criteria: SlackCanvasSectionsLookupCriteria,
}

/// Criteria combine: `section_types` narrows by kind, `contains_text` by content.
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackCanvasSectionsLookupCriteria {
    pub section_types: Option<Vec<SlackCanvasSectionType>>,
    pub contains_text: Option<String>,
}

/// Section kinds that `canvases.sections.lookup` can filter on.
/// https://docs.slack.dev/reference/methods/canvases.sections.lookup#usage-info
#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SlackCanvasSectionType {
    AnyHeader,
    Blockquote,
    Callout,
    CanvasUnfurl,
    Chart,
    Citation,
    FileUnfurl,
    Flexbox,
    H1,
    H2,
    H3,
    HorizontalLine,
    List,
    MessageUnfurl,
    SfdcRecordMention,
    SfdcRecordUnfurl,
    Table,
    UserMention,
    UserUnfurl,
    /// A kind this crate does not model yet, sent verbatim.
    #[serde(untagged)]
    Other(String),
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiCanvasesSectionsLookupResponse {
    pub sections: Vec<SlackCanvasSection>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackCanvasSection {
    pub id: SlackCanvasSectionId,
}

/// Exactly one of `channel_ids` / `user_ids` (1–20 entries). Only regular
/// channels are accepted; use `user_ids` for DMs and MPDMs. `owner` is valid
/// for users only.
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiCanvasesAccessSetRequest {
    pub canvas_id: SlackCanvasId,
    pub access_level: SlackCanvasAccessLevel,
    pub channel_ids: Option<Vec<SlackChannelId>>,
    pub user_ids: Option<Vec<SlackUserId>>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SlackCanvasAccessLevel {
    Read,
    Write,
    Owner,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiCanvasesAccessSetResponse {}

/// Exactly one of `channel_ids` / `user_ids` (a single entry each).
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiCanvasesAccessDeleteRequest {
    pub canvas_id: SlackCanvasId,
    pub channel_ids: Option<Vec<SlackChannelId>>,
    pub user_ids: Option<Vec<SlackUserId>>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiCanvasesAccessDeleteResponse {}

/// A channel already holding a canvas answers `channel_canvas_already_exists`;
/// its id is in `channel.properties.canvas` from `conversations.info`.
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiConversationsCanvasesCreateRequest {
    pub channel_id: SlackChannelId,
    pub document_content: Option<SlackCanvasDocumentContent>,
    pub title: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiConversationsCanvasesCreateResponse {
    pub canvas_id: SlackCanvasId,
}

#[cfg(test)]
mod test {
    use super::*;

    /// Deserialises, re-serialises and compares; the envelope's `ok` is not
    /// part of any response model, so it is dropped from the expectation.
    fn round_trip<T>(payload: &str) -> T
    where
        T: Serialize + for<'de> Deserialize<'de>,
    {
        let mut expected: serde_json::Value = serde_json::from_str(payload).unwrap();
        expected.as_object_mut().unwrap().remove("ok");
        let model: T = serde_json::from_str(payload).unwrap();
        assert_eq!(serde_json::to_value(&model).unwrap(), expected);
        model
    }

    #[test]
    fn test_slack_api_canvases_create_request_matches_docs_example() {
        let req = SlackApiCanvasesCreateRequest::new()
            .with_title("Your Brilliant Title".into())
            .with_document_content(SlackCanvasDocumentContent::new(
                "> standalone canvas!".into(),
            ));
        assert_eq!(
            serde_json::to_value(&req).unwrap(),
            serde_json::json!({
                "title": "Your Brilliant Title",
                "document_content": { "type": "markdown", "markdown": "> standalone canvas!" }
            })
        );
        let resp: SlackApiCanvasesCreateResponse =
            round_trip(r#"{ "ok": true, "canvas_id": "F1234ABCD" }"#);
        assert_eq!(resp.canvas_id, "F1234ABCD".into());
    }

    #[test]
    fn test_slack_api_canvases_edit_request_fixture() {
        let req: SlackApiCanvasesEditRequest = round_trip(include_str!(
            "./fixtures/slack_api_canvases_edit_request.json"
        ));
        assert_eq!(req.canvas_id, "F0166DCSTS7".into());
        assert_eq!(req.changes.len(), 1);
        assert_eq!(req.changes[0].operation, SlackCanvasOperation::InsertAfter);
        assert_eq!(
            req.changes[0].section_id,
            Some("temp:C:VXX8e648e6984e441c6aa8c61173".into())
        );
        assert_eq!(
            req.changes[0].document_content.as_ref().unwrap().markdown,
            "- [ ] asparagus\n- [ ] coffee\n"
        );

        let rename = SlackCanvasChange::new(SlackCanvasOperation::Rename)
            .with_title_content(SlackCanvasDocumentContent::new("Project Status".into()));
        assert_eq!(
            serde_json::to_value(&rename).unwrap(),
            serde_json::json!({
                "operation": "rename",
                "title_content": { "type": "markdown", "markdown": "Project Status" }
            })
        );
    }

    #[test]
    fn test_slack_api_canvases_sections_lookup_fixtures() {
        let req: SlackApiCanvasesSectionsLookupRequest = round_trip(include_str!(
            "./fixtures/slack_api_canvases_sections_lookup_request.json"
        ));
        assert_eq!(
            req.criteria.section_types,
            Some(vec![SlackCanvasSectionType::H1, SlackCanvasSectionType::H2])
        );
        assert_eq!(req.criteria.contains_text.as_deref(), Some("Grocery List"));

        let future: SlackCanvasSectionType = serde_json::from_str(r#""hologram""#).unwrap();
        assert_eq!(future, SlackCanvasSectionType::Other("hologram".into()));

        let resp: SlackApiCanvasesSectionsLookupResponse = round_trip(include_str!(
            "./fixtures/slack_api_canvases_sections_lookup_response.json"
        ));
        assert_eq!(
            resp.sections[0].id,
            "temp:C:eBa219af721c664422cb90a52fac".into()
        );
    }

    #[test]
    fn test_slack_api_canvases_access_requests_serialize_only_given_targets() {
        let set = SlackApiCanvasesAccessSetRequest::new(
            "F1234ABCD".into(),
            SlackCanvasAccessLevel::Write,
        )
        .with_channel_ids(vec!["C1234ABCD".into()]);
        assert_eq!(
            serde_json::to_value(&set).unwrap(),
            serde_json::json!({
                "canvas_id": "F1234ABCD",
                "access_level": "write",
                "channel_ids": ["C1234ABCD"]
            })
        );

        let delete = SlackApiCanvasesAccessDeleteRequest::new("F1234ABCD".into())
            .with_user_ids(vec!["U1234ABCD".into()]);
        assert_eq!(
            serde_json::to_value(&delete).unwrap(),
            serde_json::json!({ "canvas_id": "F1234ABCD", "user_ids": ["U1234ABCD"] })
        );

        let empty: SlackApiCanvasesAccessSetResponse = round_trip(r#"{ "ok": true }"#);
        assert_eq!(empty, SlackApiCanvasesAccessSetResponse::new());
    }

    #[test]
    fn test_slack_api_conversations_canvases_create_round_trip() {
        let req = SlackApiConversationsCanvasesCreateRequest::new("C1234ABCD".into())
            .with_document_content(SlackCanvasDocumentContent::new("> channel canvas!".into()));
        assert_eq!(
            serde_json::to_value(&req).unwrap(),
            serde_json::json!({
                "channel_id": "C1234ABCD",
                "document_content": { "type": "markdown", "markdown": "> channel canvas!" }
            })
        );
        let resp: SlackApiConversationsCanvasesCreateResponse =
            round_trip(r#"{ "ok": true, "canvas_id": "F1234ABCD" }"#);
        assert_eq!(resp.canvas_id, "F1234ABCD".into());
    }
}
