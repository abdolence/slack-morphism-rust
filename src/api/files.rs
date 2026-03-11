//!
//! Support for Slack Files API methods
//!

use crate::api::{
    SlackApiUsersConversationsRequest, SlackApiUsersConversationsResponse,
    SlackApiUsersProfileSetRequest, SlackApiUsersProfileSetResponse,
};
use crate::models::*;
use crate::multipart_form::FileMultipartData;
use crate::ratectl::*;
use crate::{ClientResult, SlackClientHttpConnector};
use crate::{SlackApiScrollableRequest, SlackApiScrollableResponse, SlackClientSession};
use futures_util::future::BoxFuture;
use futures_util::FutureExt;
use rsb_derive::Builder;
use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize, Serializer};
use serde_with::skip_serializing_none;
use tokio_stream::StreamExt;

impl<'a, SCHC> SlackClientSession<'a, SCHC>
where
    SCHC: SlackClientHttpConnector + Send,
{
    ///
    /// https://api.slack.com/methods/files.info
    ///
    pub async fn files_info(
        &self,
        req: &SlackApiFilesInfoRequest,
    ) -> ClientResult<SlackApiFilesInfoResponse> {
        self.http_session_api
            .http_get(
                "files.info",
                &vec![("file", Some(req.file.value()))],
                Some(&SLACK_TIER4_METHOD_CONFIG),
            )
            .await
    }

    ///
    /// https://api.slack.com/methods/files.list
    ///
    pub async fn files_list(
        &self,
        req: &SlackApiFilesListRequest,
    ) -> ClientResult<SlackApiFilesListResponse> {
        self.http_session_api
            .http_get(
                "files.list",
                &vec![
                    ("channel", req.channel.as_ref().map(|x| x.value())),
                    ("user", req.user.as_ref().map(|x| x.value())),
                    ("types", req.types.as_ref()),
                    ("count", req.count.map(|x| x.to_string()).as_ref()),
                    ("page", req.page.map(|x| x.to_string()).as_ref()),
                    ("ts_from", req.ts_from.map(|x| x.to_string()).as_ref()),
                    ("ts_to", req.ts_to.map(|x| x.to_string()).as_ref()),
                    (
                        "show_files_hidden_by_limit",
                        req.show_files_hidden_by_limit
                            .map(|x| x.to_string())
                            .as_ref(),
                    ),
                ],
                Some(&SLACK_TIER3_METHOD_CONFIG),
            )
            .await
    }

    ///
    /// https://api.slack.com/methods/files.upload
    ///
    #[deprecated(
        note = "Deprecated by Slack. Use `getUploadURLExternal/files_upload_via_url/completeUploadExternal` instead."
    )]
    pub async fn files_upload(
        &self,
        req: &SlackApiFilesUploadRequest,
    ) -> ClientResult<SlackApiFilesUploadResponse> {
        let maybe_file = req.binary_content.as_ref().map(|file_data| {
            let filename = req.filename.clone().unwrap_or("file".to_string());
            let file_content_type = req.file_content_type.clone().unwrap_or_else(|| {
                let file_mime = mime_guess::MimeGuess::from_path(&filename).first_or_octet_stream();
                file_mime.to_string()
            });
            FileMultipartData {
                name: filename,
                content_type: file_content_type,
                data: file_data.as_slice(),
            }
        });
        self.http_session_api
            .http_post_multipart_form(
                "files.upload",
                maybe_file,
                &vec![
                    (
                        "channels",
                        req.channels
                            .as_ref()
                            .map(|xs| {
                                xs.iter()
                                    .map(|x| x.to_string())
                                    .collect::<Vec<String>>()
                                    .join(",")
                            })
                            .as_ref(),
                    ),
                    ("content", req.content.as_ref()),
                    ("filename", req.filename.as_ref()),
                    ("filetype", req.filetype.as_ref().map(|x| x.value())),
                    ("initial_comment", req.initial_comment.as_ref()),
                    ("thread_ts", req.thread_ts.as_ref().map(|x| x.value())),
                    ("title", req.title.as_ref()),
                ],
                Some(&SLACK_TIER2_METHOD_CONFIG),
            )
            .await
    }

    ///
    /// https://api.slack.com/methods/files.getUploadURLExternal
    ///
    pub async fn get_upload_url_external(
        &self,
        req: &SlackApiFilesGetUploadUrlExternalRequest,
    ) -> ClientResult<SlackApiFilesGetUploadUrlExternalResponse> {
        self.http_session_api
            .http_get(
                "files.getUploadURLExternal",
                &vec![
                    ("filename", Some(&req.filename)),
                    ("length", Some(&req.length.to_string())),
                    ("alt_txt", req.alt_txt.as_ref()),
                    ("snippet_type", req.snippet_type.as_ref().map(|v| v.value())),
                ],
                Some(&SLACK_TIER4_METHOD_CONFIG),
            )
            .await
    }

    pub async fn files_upload_via_url(
        &self,
        req: &SlackApiFilesUploadViaUrlRequest,
    ) -> ClientResult<SlackApiFilesUploadViaUrlResponse> {
        self.http_session_api
            .http_post_uri_binary(
                req.upload_url.value().clone(),
                req.content_type.clone(),
                &req.content,
                Some(&SLACK_TIER4_METHOD_CONFIG),
            )
            .await
    }

    ///
    /// https://api.slack.com/methods/files.completeUploadExternal
    ///
    pub async fn files_complete_upload_external(
        &self,
        req: &SlackApiFilesCompleteUploadExternalRequest,
    ) -> ClientResult<SlackApiFilesCompleteUploadExternalResponse> {
        self.http_session_api
            .http_post(
                "files.completeUploadExternal",
                req,
                Some(&SLACK_TIER4_METHOD_CONFIG),
            )
            .await
    }

    ///
    /// https://api.slack.com/methods/files.delete
    ///
    pub async fn files_delete(
        &self,
        req: &SlackApiFilesDeleteRequest,
    ) -> ClientResult<SlackApiFilesDeleteResponse> {
        self.http_session_api
            .http_post("files.delete", req, Some(&SLACK_TIER3_METHOD_CONFIG))
            .await
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiFilesInfoRequest {
    pub file: SlackFileId,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiFilesInfoResponse {
    pub file: SlackFile,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiFilesListRequest {
    pub channel: Option<SlackChannelId>,
    pub user: Option<SlackUserId>,
    pub types: Option<String>,
    pub count: Option<u32>,
    pub page: Option<u32>,
    pub ts_from: Option<i64>,
    pub ts_to: Option<i64>,
    pub show_files_hidden_by_limit: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiFilesListResponse {
    pub files: Vec<SlackFile>,
    pub paging: Option<SlackApiFilesListPaging>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiFilesListPaging {
    pub count: Option<u32>,
    pub total: Option<u32>,
    pub page: Option<u32>,
    pub pages: Option<u32>,
}

impl<SCHC> SlackApiScrollableRequest<SCHC> for SlackApiFilesListRequest
where
    SCHC: SlackClientHttpConnector + Send + Sync + Clone + 'static,
{
    type ResponseType = SlackApiFilesListResponse;
    type CursorType = u32;
    type ResponseItemType = SlackFile;

    fn with_new_cursor(&self, new_cursor: Option<&Self::CursorType>) -> Self {
        self.clone().opt_page(new_cursor.cloned())
    }

    fn scroll<'a, 's>(
        &'a self,
        session: &'a SlackClientSession<'s, SCHC>,
    ) -> BoxFuture<'a, ClientResult<Self::ResponseType>> {
        async move { session.files_list(self).await }.boxed()
    }
}

impl SlackApiScrollableResponse for SlackApiFilesListResponse {
    type CursorType = u32;
    type ResponseItemType = SlackFile;

    fn next_cursor(&self) -> Option<Self::CursorType> {
        self.paging
            .as_ref()
            .into_iter()
            .filter_map(|paging| match (paging.page, paging.pages) {
                (Some(page), Some(pages)) if page < pages => Some(page + 1),
                _ => None,
            })
            .next()
    }

    fn scrollable_items<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Self::ResponseItemType> + 'a> {
        Box::new(self.files.iter())
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiFilesUploadRequest {
    #[serde(serialize_with = "to_csv")]
    pub channels: Option<Vec<SlackChannelId>>,
    pub content: Option<String>,
    pub binary_content: Option<Vec<u8>>,
    pub filename: Option<String>,
    pub filetype: Option<SlackFileType>,
    pub initial_comment: Option<String>,
    pub thread_ts: Option<SlackTs>,
    pub title: Option<String>,
    pub file_content_type: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiFilesUploadResponse {
    pub file: SlackFile,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiFilesGetUploadUrlExternalRequest {
    pub filename: String,
    pub length: usize,
    pub alt_txt: Option<String>,
    pub snippet_type: Option<SlackFileSnippetType>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiFilesGetUploadUrlExternalResponse {
    pub upload_url: SlackFileUploadUrl,
    pub file_id: SlackFileId,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiFilesUploadViaUrlRequest {
    pub upload_url: SlackFileUploadUrl,
    pub content: Vec<u8>,
    pub content_type: String,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiFilesUploadViaUrlResponse {}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiFilesCompleteUploadExternalRequest {
    pub files: Vec<SlackApiFilesComplete>,
    pub channel_id: Option<SlackChannelId>,
    pub initial_comment: Option<String>,
    pub thread_ts: Option<SlackTs>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiFilesCompleteUploadExternalResponse {
    pub files: Vec<SlackFile>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiFilesComplete {
    pub id: SlackFileId,
    pub title: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiFilesDeleteRequest {
    pub file: SlackFileId,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiFilesDeleteResponse {}

fn to_csv<S: Serializer>(x: &Option<Vec<SlackChannelId>>, s: S) -> Result<S::Ok, S::Error> {
    match x {
        None => s.serialize_none(),
        Some(ids) => {
            let y: Vec<String> = ids.iter().map(|v| v.0.clone()).collect();
            y.join(",").serialize(s)
        }
    }
}
