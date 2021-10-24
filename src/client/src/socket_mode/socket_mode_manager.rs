use crate::api::*;
use crate::listener::SlackClientEventsListenerEnvironment;
use crate::socket_mode::socket_mode_connector::*;
use crate::*;
use std::sync::mpsc::channel;
use std::sync::{Arc, RwLock};

pub struct SlackClientSocketModeManager<SCHC, SCWSS>
where
    SCHC: SlackClientHttpConnector + SlackClientSocketModeConnector<SCWSS> + Send + Clone + Sync,
    SCWSS: SlackSocketModeWssClient + Send + Sync,
{
    config: SlackClientSocketModeConfig,
    listener_environment: Arc<SlackClientEventsListenerEnvironment<SCHC>>,
    clients: SlackClientSocketModeClientsStorage<SCWSS>,
}

impl<SCHC, SCWSS> SlackClientSocketModeManager<SCHC, SCWSS>
where
    SCHC: SlackClientHttpConnector + SlackClientSocketModeConnector<SCWSS> + Send + Clone + Sync,
    SCWSS: SlackSocketModeWssClient + Send + Sync,
{
    pub fn new(
        config: &SlackClientSocketModeConfig,
        listener_environment: Arc<SlackClientEventsListenerEnvironment<SCHC>>,
    ) -> Self {
        SlackClientSocketModeManager {
            config: config.clone(),
            listener_environment,
            clients: Arc::new(RwLock::new(SlackClientSocketModeClients::new())),
        }
    }

    pub async fn start<'a>(&self, session: &'a SlackClientSession<'a, SCHC>) -> ClientResult<()> {
        self.create_all_clients(session).await?;

        Ok(())
    }

    pub async fn shutdown(&self) {
        let mut drained_clients = {
            let mut clients_write = self.clients.write().unwrap();
            clients_write.clear()
        };

        for client in drained_clients.iter_mut() {
            client.destroy().await;
        }
    }

    async fn create_all_clients<'a>(
        &self,
        session: &'a SlackClientSession<'a, SCHC>,
    ) -> ClientResult<()> {
        let mut clients_write = self.clients.write().unwrap();
        for _client_idx in 0..self.config.max_connections_count {
            let wss_client = self.create_new_wss_client(session).await?;
            clients_write.add_new_client(wss_client);
        }

        Ok(())
    }

    async fn create_new_wss_client<'a>(
        &self,
        session: &'a SlackClientSession<'a, SCHC>,
    ) -> ClientResult<SCWSS> {
        let open_connection_res = session
            .apps_connections_open(&SlackApiAppsConnectionOpenRequest::new())
            .await?;

        self.listener_environment
            .client
            .http_api
            .connector
            .create_wss_client(&open_connection_res.url)
            .await
    }

    pub async fn serve(&self) -> i32 {
        let (p, c) = channel();

        ctrlc::set_handler(move || p.send(1).unwrap()).expect("Error setting Ctrl-C handler");

        let result = async move { c.recv().expect("Could not receive exit from channel.") }.await;
        self.shutdown().await;

        result
    }
}
