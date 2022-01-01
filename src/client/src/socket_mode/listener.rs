use crate::listener::SlackClientEventsListenerEnvironment;
use crate::socket_mode::clients_manager::*;
use crate::socket_mode::clients_manager_listener::SlackSocketModeClientsManagerListener;
use crate::*;
use log::debug;
use signal_hook::consts::TERM_SIGNALS;
use signal_hook::iterator::exfiltrator::WithOrigin;
use signal_hook::iterator::SignalsInfo;
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
            .register_new_token(
                &self.config,
                token.clone(),
                self.clients_manager_listener.clone(),
            )
            .await
    }

    pub async fn start(&self) {
        debug!("Starting all WSS clients");
        self.clients_manager.start().await;
    }

    pub async fn shutdown(&self) {
        debug!("Shutting down all WSS clients");
        self.clients_manager.shutdown().await;
    }

    pub async fn serve(&self) -> i32 {
        self.start().await;

        let mut signals = SignalsInfo::<WithOrigin>::new(TERM_SIGNALS).unwrap();

        for info in &mut signals {
            debug!("Received a signal: {:?}. Terminating...", info);
            break;
        }

        self.shutdown().await;
        0
    }
}
