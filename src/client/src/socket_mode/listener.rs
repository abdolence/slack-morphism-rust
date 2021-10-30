use crate::listener::SlackClientEventsListenerEnvironment;
use crate::socket_mode::clients::*;
use crate::socket_mode::clients_manager::{
    SlackSocketModeClientsManager, SlackSocketModeClientsManagerListener,
};
use crate::*;
use std::sync::mpsc::channel;
use std::sync::Arc;

pub struct SlackClientSocketModeListener<SCHC, SCWSS>
where
    SCHC:
        SlackClientHttpConnector + SlackSocketModeWssClientsFactory<SCWSS> + Send + Sync + 'static,
    SCWSS: SlackSocketModeWssClient + Send + Sync + 'static,
{
    config: SlackClientSocketModeConfig,
    clients_manager: Arc<SlackSocketModeClientsManager<SCHC, SCWSS>>,
    clients_manager_listener: Arc<SlackSocketModeClientsManagerListener<SCHC, SCWSS>>,
}

impl<SCHC, SCWSS> SlackClientSocketModeListener<SCHC, SCWSS>
where
    SCHC:
        SlackClientHttpConnector + SlackSocketModeWssClientsFactory<SCWSS> + Send + Sync + 'static,
    SCWSS: SlackSocketModeWssClient + Send + Sync + 'static,
{
    pub fn new(
        config: &SlackClientSocketModeConfig,
        listener_environment: Arc<SlackClientEventsListenerEnvironment<SCHC>>,
        callbacks: SlackSocketModeListenerCallbacks<SCHC>,
    ) -> Self {
        let clients_manager = Arc::new(SlackSocketModeClientsManager::new(
            listener_environment,
            callbacks,
        ));
        let clients_manager_listener = Arc::new(SlackSocketModeClientsManagerListener::new(
            Arc::downgrade(&clients_manager),
        ));

        SlackClientSocketModeListener {
            config: config.clone(),
            clients_manager,
            clients_manager_listener,
        }
    }

    pub async fn start_for(&self, token: &SlackApiToken) -> ClientResult<()> {
        self.clients_manager
            .create_all_clients(
                self.config.clone(),
                token.clone(),
                self.clients_manager_listener.clone(),
            )
            .await?;

        Ok(())
    }

    pub async fn shutdown(&self) {
        self.clients_manager.shutdown().await;
    }

    pub async fn serve(&self) -> i32 {
        let (p, c) = channel();

        ctrlc::set_handler(move || p.send(1).unwrap()).expect("Error setting Ctrl-C handler");

        let result = async move { c.recv().expect("Could not receive exit from channel.") }.await;
        self.shutdown().await;

        result
    }
}
