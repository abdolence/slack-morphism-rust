//!
//! Support for Slack Apps API methods
//!

use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use slack_morphism_models::*;

use crate::ratectl::*;
use crate::SlackClientSession;
use crate::{ClientResult, SlackClientHttpConnector};

impl<'a, SCHC> SlackClientSession<'a, SCHC>
where
    SCHC: SlackClientHttpConnector + Send,
{
    ///
    /// https://api.slack.com/methods/apps.connections.open
    ///
    pub async fn apps_connections_open(
        &self,
        req: &SlackApiAppsConnectionOpenRequest,
    ) -> ClientResult<SlackApiAppsConnectionOpenResponse> {
        self.http_session_api
            .http_post(
                "apps.connections.open",
                req,
                Some(&SLACK_TIER1_METHOD_CONFIG),
            )
            .await
    }

    ///
    /// https://api.slack.com/methods/apps.manifest.export
    ///
    pub async fn apps_manifest_export(
        &self,
        req: &SlackApiAppsManifestExportRequest,
    ) -> ClientResult<SlackApiAppsManifestExportResponse> {
        self.http_session_api
            .http_post(
                "apps.manifest.export",
                req,
                Some(&SLACK_TIER3_METHOD_CONFIG),
            )
            .await
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiAppsConnectionOpenRequest {}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiAppsConnectionOpenResponse {
    pub url: SlackWebSocketsUrl,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiAppsManifestExportRequest {
    pub app_id: SlackAppId,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiAppsManifestExportResponse {
    pub manifest: SlackAppManifest,
}
