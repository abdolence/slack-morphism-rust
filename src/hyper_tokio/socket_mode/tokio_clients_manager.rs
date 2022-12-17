use crate::*;
use async_trait::async_trait;
use hyper::client::connect::Connect;
use std::sync::Arc;

use crate::hyper_tokio::socket_mode::tungstenite_wss_client::SlackTungsteniteWssClient;
use crate::socket_mode::SlackSocketModeWssClientId;
use futures::future;
use futures::stream::StreamExt;
use signal_hook::consts::TERM_SIGNALS;
use signal_hook::iterator::exfiltrator::WithOrigin;
use signal_hook_tokio::SignalsInfo;

use crate::clients_manager::SlackSocketModeClientsManager;
use crate::hyper_tokio::SlackClientHyperConnector;
use crate::listener::SlackClientEventsListenerEnvironment;
use tokio::sync::RwLock;
use tracing::*;

pub struct SlackSocketModeTokioClientsManager<SCHC>
where
    SCHC: SlackClientHttpConnector + Send + Sync,
{
    listener_environment: Arc<SlackClientEventsListenerEnvironment<SCHC>>,
    active_clients: Arc<RwLock<Vec<SlackTungsteniteWssClient<SCHC>>>>,
}

impl<SCHC> SlackSocketModeTokioClientsManager<SCHC>
where
    SCHC: SlackClientHttpConnector + Send + Sync,
{
    pub fn new(listener_environment: Arc<SlackClientEventsListenerEnvironment<SCHC>>) -> Self {
        Self {
            listener_environment,
            active_clients: Arc::new(RwLock::new(vec![])),
        }
    }
}

#[async_trait]
impl<H: Send + Sync + Clone + Connect + 'static> SlackSocketModeClientsManager
    for SlackSocketModeTokioClientsManager<SlackClientHyperConnector<H>>
{
    async fn register_new_token(
        &self,
        config: &SlackClientSocketModeConfig,
        token: SlackApiToken,
        client_listener: Arc<dyn SlackSocketModeClientListener + Sync + Send>,
    ) -> ClientResult<()> {
        {
            let mut clients_write = self.active_clients.write().await;

            let last_client_id_value = clients_write.len();
            let new_clients_range = last_client_id_value as u32
                ..(last_client_id_value as u32 + config.max_connections_count);

            for client_id_value in new_clients_range {
                let wss_client_result = SlackTungsteniteWssClient::new(
                    SlackSocketModeWssClientId::new(
                        client_id_value,
                        client_id_value - last_client_id_value as u32,
                        0,
                    ),
                    client_listener.clone(),
                    &token,
                    config,
                    self.listener_environment.clone(),
                );
                clients_write.push(wss_client_result);
            }
        }

        Ok(())
    }

    async fn restart_client(&self, client_id: &SlackSocketModeWssClientId) {
        debug!(
            slack_wss_client_id = client_id.to_string().as_str(),
            "[{}] Removing client",
            client_id.to_string()
        );

        let mut removed_clients = {
            let mut clients_write = self.active_clients.write().await;

            match clients_write
                .iter()
                .enumerate()
                .find(|(_, client)| client.identity.id == *client_id)
            {
                Some((index, _)) => clients_write
                    .drain(index..=index)
                    .collect::<Vec<SlackTungsteniteWssClient<SlackClientHyperConnector<H>>>>(),
                None => vec![],
            }
        };

        if !removed_clients.is_empty() {
            let removed_client = &mut removed_clients[0];
            removed_client.shutdown_channel().await;

            // Reconnect
            trace!(
                slack_wss_client_id = client_id.to_string().as_str(),
                "[{}] Reconnecting...",
                client_id.to_string()
            );
            let client = SlackTungsteniteWssClient::new(
                removed_client.identity.id.new_reconnected_id(),
                removed_client.identity.client_listener.clone(),
                &removed_client.identity.token,
                &removed_client.identity.config,
                self.listener_environment.clone(),
            );

            client.start(0).await;
            let mut clients_write = self.active_clients.write().await;
            clients_write.push(client);
        } else {
            trace!(
                "[{}] No need to reconnect for client",
                client_id.to_string()
            )
        }
    }

    async fn start(&self) {
        let clients_read = self.active_clients.read().await;
        let mut clients_to_await = vec![];
        for client_id_value in 0..clients_read.len() {
            let client = &clients_read[client_id_value];
            clients_to_await.push(
                client.start(
                    client_id_value as u64 * client.identity.config.initial_backoff_in_seconds,
                ),
            );
        }

        future::join_all(clients_to_await).await;
    }

    async fn shutdown(&self) {
        let mut drained_clients: Vec<SlackTungsteniteWssClient<SlackClientHyperConnector<H>>> = {
            let mut clients_write = self.active_clients.write().await;
            let existing_vec = clients_write.drain(..).collect();
            existing_vec
        };

        for client in drained_clients.iter_mut() {
            client.shutdown_channel().await;
        }
    }

    async fn await_term_signals(&self) {
        let mut signals = SignalsInfo::<WithOrigin>::new(TERM_SIGNALS).unwrap();

        if let Some(info) = signals.next().await {
            debug!("Received a signal: {:?}. Terminating...", info);
        }
    }
}
