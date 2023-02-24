//!
//! Support for Slack Files API methods
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
    /// https://api.slack.com/methods/files.upload
    ///
    pub async fn files_upload(
        &self,
        req: &SlackApiFilesUploadRequest,
    ) -> ClientResult<SlackApiFilesUploadResponse> {
        self.http_session_api
            .http_post_form_urlencoded("files.upload", req, Some(&SLACK_TIER2_METHOD_CONFIG))
            .await
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiFilesUploadRequest {
    pub channels: SlackChannelId,
    pub content: Option<String>,
    pub filename: Option<String>,
    pub filetype: Option<SlackFileType>,
    pub initial_comment: Option<String>,
    pub thread_ts: Option<SlackTs>,
    pub title: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiFilesUploadResponse {
    pub channel: SlackChannelId,
    pub ts: SlackTs,
    pub message: SlackMessage,
}
