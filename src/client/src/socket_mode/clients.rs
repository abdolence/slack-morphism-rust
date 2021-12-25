use async_trait::async_trait;
use std::sync::Arc;

use slack_morphism_models::*;

use crate::listener::SlackClientEventsListenerEnvironment;
use crate::socket_mode::wss_client_id::SlackSocketModeWssClientId;
use crate::*;

pub trait SlackSocketModeClientsManagerFactory<SCHC>
where
    SCHC: SlackClientHttpConnector + Send + Sync + 'static,
{
    fn new_clients_manager(
        &self,
        listener_environment: Arc<SlackClientEventsListenerEnvironment<SCHC>>,
    ) -> Arc<dyn SlackSocketModeClientsManagerT + Send + Sync>;
}

#[async_trait]
pub trait SlackSocketModeClientsManagerT {
    async fn create_all_clients(
        &self,
        config: &SlackClientSocketModeConfig,
        token: SlackApiToken,
        client_listener: Arc<dyn SlackSocketModeWssClientListener + Sync + Send>,
    ) -> ClientResult<()>;

    async fn start_clients(&self, config: &SlackClientSocketModeConfig);

    async fn shutdown(&self);
    async fn restart_client(&self, client_id: &SlackSocketModeWssClientId);
}

pub trait SlackSocketModeWssClientsFactory<SCWSS>
where
    SCWSS: SlackSocketModeWssClient + Send + Sync,
{
    fn create_wss_client<'a>(
        &'a self,
        wss_url: &'a SlackWebSocketsUrl,
        client_id: SlackSocketModeWssClientId,
        client_listener: Arc<dyn SlackSocketModeWssClientListener + Sync + Send + 'static>,
    ) -> ClientResult<SCWSS>;
}

#[async_trait]
pub trait SlackSocketModeWssClient {
    async fn message(&self, message_body: String) -> ClientResult<()>;

    async fn start(
        &self,
        initial_wait_timeout: u64,
        reconnect_timeout: u64,
        ping_interval: u64,
        ping_failure_threshold: u64,
    );
    async fn destroy(&mut self);
}

#[async_trait]
pub trait SlackSocketModeWssClientListener {
    async fn on_message(
        &self,
        client_id: &SlackSocketModeWssClientId,
        message_body: String,
    ) -> Option<String>;

    async fn on_error(&self, error: Box<dyn std::error::Error + Send + Sync>);

    async fn on_disconnect(&self, client_id: &SlackSocketModeWssClientId);
}
