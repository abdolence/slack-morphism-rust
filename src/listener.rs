use crate::models::*;
use crate::{ClientResult, SlackClient, SlackClientHttpConnector};
use futures::executor::block_on;
use futures::FutureExt;
use rsb_derive::Builder;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;
use tracing::*;
use url::Url;

type UserStatesMap = HashMap<TypeId, Box<dyn Any + Send + Sync + 'static>>;

pub struct SlackClientEventsListenerEnvironment<SCHC>
where
    SCHC: SlackClientHttpConnector + Send + Sync,
{
    pub client: Arc<SlackClient<SCHC>>,
    pub error_handler: BoxedErrorHandler<SCHC>,
    pub user_state: SlackClientEventsUserState,
}

pub type SlackClientEventsUserState = futures_locks::RwLock<SlackClientEventsUserStateStorage>;

impl<SCHC> SlackClientEventsListenerEnvironment<SCHC>
where
    SCHC: SlackClientHttpConnector + Send + Sync,
{
    pub fn new(client: Arc<SlackClient<SCHC>>) -> Self {
        Self {
            client,
            error_handler: Box::new(Self::empty_error_handler),
            user_state: SlackClientEventsUserState::new(SlackClientEventsUserStateStorage::new()),
        }
    }

    pub fn with_error_handler(self, error_handler: ErrorHandler<SCHC>) -> Self {
        Self {
            error_handler: Box::new(error_handler),
            ..self
        }
    }

    fn empty_error_handler(
        err: Box<dyn std::error::Error + Send + Sync>,
        _client: Arc<SlackClient<SCHC>>,
        _user_state_storage: SlackClientEventsUserState,
    ) -> http::StatusCode {
        error!("Slack listener error occurred: {:?}", err);
        http::StatusCode::BAD_REQUEST
    }

    pub fn with_user_state<T: Send + Sync + 'static>(self, state: T) -> Self {
        let future_init_state = self
            .user_state
            .write()
            .map(|mut guard| guard.set_user_state(state));
        block_on(future_init_state);
        self
    }
}

pub struct SlackClientEventsUserStateStorage {
    user_state_map: UserStatesMap,
}

impl SlackClientEventsUserStateStorage {
    pub fn new() -> Self {
        SlackClientEventsUserStateStorage {
            user_state_map: HashMap::new(),
        }
    }

    pub fn get_user_state<T: Send + Sync + 'static>(&self) -> Option<&T> {
        self.user_state_map
            .get(&TypeId::of::<T>())
            .and_then(|boxed| (&**boxed as &(dyn Any + 'static)).downcast_ref())
    }

    pub fn set_user_state<T: Send + Sync + 'static>(&mut self, state: T) {
        self.user_state_map
            .insert(TypeId::of::<T>(), Box::new(state));
    }

    pub fn len(&self) -> usize {
        self.user_state_map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.user_state_map.is_empty()
    }
}

pub type BoxedErrorHandler<SCHC> = Box<ErrorHandler<SCHC>>;

pub type ErrorHandler<SCHC> = fn(
    Box<dyn std::error::Error + Send + Sync + 'static>,
    Arc<SlackClient<SCHC>>,
    SlackClientEventsUserState,
) -> http::StatusCode;

#[derive(Debug, PartialEq, Clone, Builder)]
pub struct SlackCommandEventsListenerConfig {
    pub events_signing_secret: SlackSigningSecret,
    #[default = "SlackCommandEventsListenerConfig::DEFAULT_EVENTS_URL_VALUE.into()"]
    pub events_path: String,
}

impl SlackCommandEventsListenerConfig {
    pub const DEFAULT_EVENTS_URL_VALUE: &'static str = "/command";
}

#[derive(Debug, PartialEq, Clone, Builder)]
pub struct SlackPushEventsListenerConfig {
    pub events_signing_secret: SlackSigningSecret,
    #[default = "SlackPushEventsListenerConfig::DEFAULT_EVENTS_URL_VALUE.into()"]
    pub events_path: String,
}

impl SlackPushEventsListenerConfig {
    const DEFAULT_EVENTS_URL_VALUE: &'static str = "/push";
}

#[derive(Debug, PartialEq, Clone, Builder)]
pub struct SlackInteractionEventsListenerConfig {
    pub events_signing_secret: SlackSigningSecret,
    #[default = "SlackInteractionEventsListenerConfig::DEFAULT_EVENTS_URL_VALUE.into()"]
    pub events_path: String,
}

impl SlackInteractionEventsListenerConfig {
    pub const DEFAULT_EVENTS_URL_VALUE: &'static str = "/interaction";
}

#[derive(Debug, PartialEq, Clone, Builder)]
pub struct SlackOAuthListenerConfig {
    pub client_id: SlackClientId,
    pub client_secret: SlackClientSecret,
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

    pub fn to_redirect_url(&self) -> ClientResult<Url> {
        Url::parse(
            format!(
                "{}{}",
                &self.redirect_callback_host, &self.redirect_callback_path
            )
            .as_str(),
        )
        .map_err(|e| e.into())
    }
}

pub type UserCallbackFunction<E, IF, SCHC> =
    fn(E, Arc<SlackClient<SCHC>>, SlackClientEventsUserState) -> IF;
