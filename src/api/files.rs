//!
//! Support for Slack Files API methods
//!

use rsb_derive::Builder;
use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize, Serializer};
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
    /// https://api.slack.com/methods/files.upload
    ///
    pub async fn files_upload(
        &self,
        req: &SlackApiFilesUploadRequest,
    ) -> ClientResult<SlackApiFilesUploadResponse> {
        if let Some(file) = &req.file {
            let filename = req.filename.clone().unwrap_or("file".to_string());
            let file_content_type = req.file_content_type.clone().unwrap_or_else(|| {
                let file_mime = mime_guess::MimeGuess::from_path(&filename).first_or_octet_stream();
                file_mime.to_string()
            });
            self.http_session_api
                .http_post_multipart_form(
                    "files.upload",
                    filename,
                    file_content_type,
                    file,
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
                        ("filetype", req.filetype.as_ref().map(|x| x.value())),
                        ("initial_comment", req.initial_comment.as_ref()),
                        ("thread_ts", req.thread_ts.as_ref().map(|x| x.value())),
                        ("title", req.title.as_ref()),
                    ],
                    Some(&SLACK_TIER2_METHOD_CONFIG),
                )
                .await
        } else {
            self.http_session_api
                .http_post_form_urlencoded("files.upload", req, Some(&SLACK_TIER2_METHOD_CONFIG))
                .await
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiFilesUploadRequest {
    #[serde(serialize_with = "to_csv")]
    pub channels: Option<Vec<SlackChannelId>>,
    pub content: Option<String>,
    pub filename: Option<String>,
    pub filetype: Option<SlackFileType>,
    pub initial_comment: Option<String>,
    pub thread_ts: Option<SlackTs>,
    pub title: Option<String>,
    pub file: Option<Vec<u8>>,
    pub file_content_type: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiFilesUploadResponse {
    pub file: SlackFile,
}

fn to_csv<S: Serializer>(x: &Option<Vec<SlackChannelId>>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match x {
        None => s.serialize_none(),
        Some(ids) => {
            let y: Vec<String> = ids.iter().map(|v| v.0.clone()).collect();
            y.join(",").serialize(s)
        }
    }
}
