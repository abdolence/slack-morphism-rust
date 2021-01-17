use crate::{SlackClient, SlackClientHttpConnector};
use rsb_derive::Builder;
use std::sync::Arc;

#[derive(Clone)]
pub struct SlackClientEventsListenerEnvironment<SCHC>
where
    SCHC: SlackClientHttpConnector + Send + Clone + Sync,
{
    pub client: Arc<SlackClient<SCHC>>,
    pub error_handler: ErrorHandler<SCHC>,
}

impl<SCHC> SlackClientEventsListenerEnvironment<SCHC>
where
    SCHC: SlackClientHttpConnector + Send + Clone + Sync,
{
    pub fn new(client: Arc<SlackClient<SCHC>>) -> Self {
        Self {
            client,
            error_handler: Box::new(Self::empty_error_handler),
        }
    }

    pub fn with_error_handler(
        self,
        error_handler: fn(
            Box<dyn std::error::Error + Send + Sync + 'static>,
            Arc<SlackClient<SCHC>>,
        ),
    ) -> Self {
        Self {
            error_handler: Box::new(error_handler),
            ..self
        }
    }

    fn empty_error_handler(
        _err: Box<dyn std::error::Error + Send + Sync>,
        _client: Arc<SlackClient<SCHC>>,
    ) {
    }
}

pub type ErrorHandler<SCHC> =
    Box<fn(Box<dyn std::error::Error + Send + Sync + 'static>, Arc<SlackClient<SCHC>>)>;

#[derive(Debug, PartialEq, Clone, Builder)]
pub struct SlackCommandEventsListenerConfig {
    pub events_signing_secret: String,
    #[default = "SlackCommandEventsListenerConfig::DEFAULT_EVENTS_URL_VALUE.into()"]
    pub events_path: String,
}

impl SlackCommandEventsListenerConfig {
    pub const DEFAULT_EVENTS_URL_VALUE: &'static str = "/command";
}

#[derive(Debug, PartialEq, Clone, Builder)]
pub struct SlackPushEventsListenerConfig {
    pub events_signing_secret: String,
    #[default = "SlackPushEventsListenerConfig::DEFAULT_EVENTS_URL_VALUE.into()"]
    pub events_path: String,
}

impl SlackPushEventsListenerConfig {
    const DEFAULT_EVENTS_URL_VALUE: &'static str = "/push";
}

#[derive(Debug, PartialEq, Clone, Builder)]
pub struct SlackInteractionEventsListenerConfig {
    pub events_signing_secret: String,
    #[default = "SlackInteractionEventsListenerConfig::DEFAULT_EVENTS_URL_VALUE.into()"]
    pub events_path: String,
}

impl SlackInteractionEventsListenerConfig {
    pub const DEFAULT_EVENTS_URL_VALUE: &'static str = "/interaction";
}

#[derive(Debug, PartialEq, Clone, Builder)]
pub struct SlackOAuthListenerConfig {
    pub client_id: String,
    pub client_secret: String,
    pub bot_scope: String,
    pub redirect_callback_host: String,
    #[default = "SlackOAuthListenerConfig::DEFAULT_INSTALL_PATH_VALUE.into()"]
    pub install_path: String,
    #[default = "SlackOAuthListenerConfig::DEFAULT_CALLBACK_PATH_VALUE.into()"]
    pub redirect_callback_path: String,
    #[default = "SlackOAuthListenerConfig::DEFAULT_INSTALLED_URL_VALUE.into()"]
    pub redirect_installed_url: String,
    #[default = "SlackOAuthListenerConfig::DEFAULT_CANCELLED_URL_VALUE.into()"]
    pub redirect_cancelled_url: String,
    #[default = "SlackOAuthListenerConfig::DEFAULT_ERROR_URL_VALUE.into()"]
    pub redirect_error_redirect_url: String,
}

impl SlackOAuthListenerConfig {
    pub const DEFAULT_INSTALL_PATH_VALUE: &'static str = "/auth/install";
    pub const DEFAULT_CALLBACK_PATH_VALUE: &'static str = "/auth/callback";
    pub const DEFAULT_INSTALLED_URL_VALUE: &'static str = "/installed";
    pub const DEFAULT_CANCELLED_URL_VALUE: &'static str = "/cancelled";
    pub const DEFAULT_ERROR_URL_VALUE: &'static str = "/error";

    pub const OAUTH_AUTHORIZE_URL_VALUE: &'static str = "https://slack.com/oauth/v2/authorize";

    pub fn to_redirect_url(&self) -> String {
        format!(
            "{}{}",
            &self.redirect_callback_host, &self.redirect_callback_path
        )
    }
}
