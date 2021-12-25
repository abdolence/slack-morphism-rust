use async_trait::async_trait;
use std::sync::Arc;

use crate::listener::SlackClientEventsListenerEnvironment;
use crate::socket_mode::clients_manager_listener::SlackSocketModeWssClientListener;
use crate::socket_mode::wss_client_id::SlackSocketModeWssClientId;
use crate::*;

pub trait SlackSocketModeClientsManagerFactory<SCHC>
where
    SCHC: SlackClientHttpConnector + Send + Sync,
{
    fn new_clients_manager(
        &self,
        listener_environment: Arc<SlackClientEventsListenerEnvironment<SCHC>>,
    ) -> Arc<dyn SlackSocketModeClientsManager + Send + Sync>;
}

#[async_trait]
pub trait SlackSocketModeClientsManager {
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
