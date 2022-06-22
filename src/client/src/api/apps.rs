//!
//! Support for Slack Apps API methods
//!

use rsb_derive::Builder;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
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

    ///
    /// https://api.slack.com/methods/apps.manifest.update
    ///
    pub async fn apps_manifest_update(
        &self,
        req: &SlackApiAppsManifestUpdateRequest,
    ) -> ClientResult<SlackApiAppsManifestUpdateResponse> {
        self.http_session_api
            .http_post(
                "apps.manifest.update",
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

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiAppsManifestUpdateRequest {
    pub app_id: SlackAppId,
    #[serde(
        serialize_with = "as_json_string",
        deserialize_with = "from_json_string"
    )]
    pub manifest: SlackAppManifest,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiAppsManifestUpdateResponse {
    pub app_id: SlackAppId,
    pub permission_updated: bool,
}

fn as_json_string<T, S>(x: &T, s: S) -> Result<S::Ok, S::Error>
where
    T: Serialize,
    S: Serializer,
{
    s.serialize_str(&serde_json::to_string(x).map_err(serde::ser::Error::custom)?)
}

fn from_json_string<'de, T, D>(d: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + 'static,
    D: Deserializer<'de>,
{
    serde_json::from_str(<&str>::deserialize(d)?).map_err(serde::de::Error::custom)
}
