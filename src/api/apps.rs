//!
//! Support for Slack Apps API methods
//!

use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none};
use url::Url;

use crate::*;

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
    /// https://api.slack.com/methods/apps.manifest.create
    ///
    pub async fn apps_manifest_create(
        &self,
        req: &SlackApiAppsManifestCreateRequest,
    ) -> ClientResult<SlackApiAppsManifestCreateResponse> {
        self.http_session_api
            .http_post(
                "apps.manifest.create",
                req,
                Some(&SLACK_TIER1_METHOD_CONFIG),
            )
            .await
    }

    ///
    /// https://api.slack.com/methods/apps.manifest.delete
    ///
    pub async fn apps_manifest_delete(
        &self,
        req: &SlackApiAppsManifestDeleteRequest,
    ) -> ClientResult<()> {
        self.http_session_api
            .http_post(
                "apps.manifest.delete",
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
                Some(&SLACK_TIER1_METHOD_CONFIG),
            )
            .await
    }

    ///
    /// https://api.slack.com/methods/apps.manifest.validate
    ///
    pub async fn apps_manifest_validate(
        &self,
        req: &SlackApiAppsManifestValidateRequest,
    ) -> ClientResult<()> {
        self.http_session_api
            .http_post(
                "apps.manifest.validate",
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

#[serde_as]
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiAppsManifestCreateRequest {
    pub app_id: SlackAppId,

    // This API requires a "json-encoded" string in a JSON object.
    #[serde_as(as = "serde_with::json::JsonString")]
    pub manifest: SlackAppManifest,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiAppsManifestCreateResponse {
    pub app_id: SlackAppId,
    pub credentials: SlackAppCredentials,
    pub oauth_authorize_url: Url,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiAppsManifestDeleteRequest {
    pub app_id: SlackAppId,
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

#[serde_as]
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiAppsManifestUpdateRequest {
    pub app_id: SlackAppId,

    // This API requires a "json-encoded" string in a JSON object.
    #[serde_as(as = "serde_with::json::JsonString")]
    pub manifest: SlackAppManifest,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiAppsManifestUpdateResponse {
    pub app_id: SlackAppId,
    pub permissions_updated: bool,
}

#[serde_as]
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiAppsManifestValidateRequest {
    // This API requires a "json-encoded" string in a JSON object.
    #[serde_as(as = "serde_with::json::JsonString")]
    pub manifest: SlackAppManifest,

    pub app_id: Option<SlackAppId>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_slack_api_apps_manifest_create_request() {
        let payload = include_str!("./fixtures/slack_api_apps_manifest_create_request.json");
        let model: SlackApiAppsManifestCreateRequest = serde_json::from_str(payload).unwrap();
        assert_eq!(model.app_id, SlackAppId::from("test-app-id"));
        assert_eq!(model.manifest.display_information.name, "My App");
    }
}
