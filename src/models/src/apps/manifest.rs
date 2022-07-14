use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use url::Url;

use crate::{SlackApiTokenScope, SlackCallbackId, SlackShortcutType};

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackAppManifestMetadata {
    pub major_version: Option<usize>,
    pub minor_version: Option<usize>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackAppManifestDisplayInformation {
    pub name: String,
    pub description: Option<String>,
    pub background_color: Option<String>,
    pub long_description: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackAppManifestSettingsEventSubscriptions {
    pub request_url: Option<Url>,
    pub bot_events: Option<Vec<String>>,
    pub user_events: Option<Vec<String>>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackAppManifestSettingsInteractivity {
    pub is_enabled: bool,
    pub request_url: Option<Url>,
    pub message_menu_options_url: Option<Url>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackAppManifestSettings {
    pub allowed_ip_address_ranges: Option<Vec<String>>,
    pub event_subscriptions: Option<SlackAppManifestSettingsEventSubscriptions>,
    pub interactivity: Option<SlackAppManifestSettingsInteractivity>,
    pub org_deploy_enabled: Option<bool>,
    pub socket_mode_enabled: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackAppManifestFeaturesAppHome {
    pub home_tab_enabled: Option<bool>,
    pub messages_tab_enabled: Option<bool>,
    pub messages_tab_read_only_enabled: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackAppManifestFeaturesBotUser {
    pub display_name: String,
    pub always_online: bool,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackAppManifestFeaturesShortcut {
    pub name: String,
    pub callback_id: SlackCallbackId,
    pub description: String,
    #[serde(rename = "type")]
    pub ty: SlackShortcutType,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackAppManifestFeaturesSlashCommand {
    pub command: String,
    pub description: String,
    pub should_escape: Option<bool>,
    pub url: Option<Url>,
    pub usage_hint: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackAppManifestFeaturesWorkflowStep {
    pub name: String,
    pub callback_id: SlackCallbackId,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackAppManifestFeatures {
    pub app_home: Option<SlackAppManifestFeaturesAppHome>,
    pub bot_user: Option<SlackAppManifestFeaturesBotUser>,
    pub shortcuts: Option<Vec<SlackAppManifestFeaturesShortcut>>,
    pub slash_commands: Option<Vec<SlackAppManifestFeaturesSlashCommand>>,
    pub unfurl_domains: Option<Vec<String>>,
    pub workflow_steps: Option<Vec<SlackAppManifestFeaturesWorkflowStep>>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackAppManifestOAuthConfigScopes {
    pub bot: Option<Vec<SlackApiTokenScope>>,
    pub user: Option<Vec<SlackApiTokenScope>>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackAppManifestOAuthConfig {
    pub redirect_urls: Option<Vec<Url>>,
    pub scopes: Option<SlackAppManifestOAuthConfigScopes>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackAppManifest {
    #[serde(rename = "_metadata")]
    pub metadata: Option<SlackAppManifestMetadata>,
    pub display_information: SlackAppManifestDisplayInformation,
    pub settings: Option<SlackAppManifestSettings>,
    pub features: Option<SlackAppManifestFeatures>,
    pub oauth_config: Option<SlackAppManifestOAuthConfig>,
}
