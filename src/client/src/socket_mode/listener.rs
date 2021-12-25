use crate::listener::SlackClientEventsListenerEnvironment;
use crate::socket_mode::clients::*;
use crate::socket_mode::clients_manager_listener::SlackSocketModeClientsManagerListener;
use crate::*;
use futures::channel::mpsc::channel;
use futures::StreamExt;
use std::sync::Arc;

pub struct SlackClientSocketModeListener<SCHC>
where
    SCHC: SlackClientHttpConnector
        + SlackSocketModeClientsManagerFactory<SCHC>
        + Send
        + Sync
        + 'static,
{
    config: SlackClientSocketModeConfig,
    clients_manager: Arc<dyn SlackSocketModeClientsManager + Send + Sync>,
    clients_manager_listener: Arc<SlackSocketModeClientsManagerListener<SCHC>>,
}

impl<SCHC> SlackClientSocketModeListener<SCHC>
where
    SCHC: SlackClientHttpConnector
        + SlackSocketModeClientsManagerFactory<SCHC>
        + Send
        + Sync
        + 'static,
{
    pub fn new(
        config: &SlackClientSocketModeConfig,
        listener_environment: Arc<SlackClientEventsListenerEnvironment<SCHC>>,
        callbacks: SlackSocketModeListenerCallbacks<SCHC>,
    ) -> Self {
        let clients_manager: Arc<dyn SlackSocketModeClientsManager + Send + Sync> =
            listener_environment
                .client
                .http_api
                .connector
                .new_clients_manager(listener_environment.clone());

        let clients_manager_listener = Arc::new(SlackSocketModeClientsManagerListener::new(
            Arc::downgrade(&clients_manager),
            listener_environment.clone(),
            callbacks,
        ));

        SlackClientSocketModeListener {
            config: config.clone(),
            clients_manager,
            clients_manager_listener,
        }
    }

    pub async fn listen_for(&self, token: &SlackApiToken) -> ClientResult<()> {
        self.clients_manager
            .create_all_clients(
                &self.config,
                token.clone(),
                self.clients_manager_listener.clone(),
            )
            .await
    }

    pub async fn start(&self) {
        self.clients_manager.start_clients(&self.config).await;
    }

    pub async fn shutdown(&self) {
        self.clients_manager.shutdown().await;
    }

    pub async fn serve(&self) -> i32 {
        let (mut sender, mut receiver) = channel(1);

        self.start().await;

        ctrlc::set_handler(move || sender.try_send(1).unwrap())
            .expect("Error setting Ctrl-C handler");

        let result = receiver
            .next()
            .await
            .expect("Could not receive exit from channel.");

        self.shutdown().await;

        result
    }
}
