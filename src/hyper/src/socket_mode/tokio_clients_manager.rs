use crate::*;
use async_trait::async_trait;
use hyper::client::connect::Connect;
use std::ops::Range;
use std::sync::Arc;

use crate::socket_mode::tungstenite_wss_client::SlackTungsteniteWssClient;
use crate::socket_mode::SlackSocketModeWssClientId;
use futures::future;
use log::*;
use slack_morphism::clients_manager::SlackSocketModeClientsManager;
use slack_morphism::listener::SlackClientEventsListenerEnvironment;
use slack_morphism::{
    ClientResult, SlackApiToken, SlackClientHttpConnector, SlackClientSocketModeConfig,
    SlackSocketModeClientListener,
};
use tokio::sync::RwLock;

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

    async fn get_next_clients_range_indices(
        &self,
        config: &SlackClientSocketModeConfig,
    ) -> Range<u32> {
        let clients_read = self.active_clients.read().await;
        let last_client_id_value = clients_read.len();
        last_client_id_value as u32
            ..(last_client_id_value as u32 + config.max_connections_count) as u32
    }

    async fn create_new_wss_client(
        &self,
        client_id: SlackSocketModeWssClientId,
        token: SlackApiToken,
        client_listener: Arc<dyn SlackSocketModeClientListener + Sync + Send + 'static>,
        config: SlackClientSocketModeConfig,
    ) -> ClientResult<SlackTungsteniteWssClient<SCHC>> {
        Ok(SlackTungsteniteWssClient::new(
            client_id.clone(),
            client_listener.clone(),
            &token,
            &config,
            self.listener_environment.clone(),
        ))
    }
}

#[async_trait]
impl<H: Send + Sync + Clone + Connect + 'static> SlackSocketModeClientsManager
    for SlackSocketModeTokioClientsManager<SlackClientHyperConnector<H>>
{
    async fn create_all_clients(
        &self,
        config: &SlackClientSocketModeConfig,
        token: SlackApiToken,
        client_listener: Arc<dyn SlackSocketModeClientListener + Sync + Send>,
    ) -> ClientResult<()> {
        let new_clients_range = self.get_next_clients_range_indices(config).await;
        {
            let mut clients_write = self.active_clients.write().await;

            for client_id_value in new_clients_range {
                let wss_client_result = self
                    .create_new_wss_client(
                        SlackSocketModeWssClientId::new(client_id_value, 0),
                        token.clone(),
                        client_listener.clone(),
                        config.clone(),
                    )
                    .await?;

                clients_write.push(wss_client_result);
            }
        }

        Ok(())
    }

    async fn start_clients(&self, config: &SlackClientSocketModeConfig) {
        let clients_read = self.active_clients.read().await;
        let mut clients_to_await = vec![];
        for client_id_value in 0..clients_read.len() {
            clients_to_await.push(clients_read[client_id_value].start(
                client_id_value as u64 * config.initial_backoff_in_seconds,
                config.reconnect_timeout_in_seconds,
                config.ping_interval_in_seconds,
                config.ping_failure_threshold_times,
                config.debug_connections,
            ));
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

    async fn restart_client(&self, client_id: &SlackSocketModeWssClientId) {
        debug!("[{}] Removing client", client_id.to_string());

        let mut removed_clients = {
            let mut clients_write = self.active_clients.write().await;

            match clients_write
                .iter()
                .enumerate()
                .find(|(_, client)| client.id == *client_id)
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
            trace!("[{}] Reconnecting...", client_id.to_string());
            match self
                .create_new_wss_client(
                    removed_client.id.new_reconnected_id(),
                    removed_client.token.clone(),
                    removed_client.client_listener.clone(),
                    removed_client.config.clone(),
                )
                .await
            {
                Ok(client) => {
                    client
                        .start(
                            0,
                            removed_client.config.reconnect_timeout_in_seconds,
                            removed_client.config.ping_interval_in_seconds,
                            removed_client.config.ping_failure_threshold_times,
                            removed_client.config.debug_connections,
                        )
                        .await;
                    let mut clients_write = self.active_clients.write().await;
                    clients_write.push(client);
                }
                Err(err) => {
                    error!(
                        "[{}] Unable to recreate WSS client: {}",
                        client_id.to_string(),
                        err
                    );
                }
            }
        } else {
            trace!(
                "[{}] No need to reconnect for client",
                client_id.to_string()
            )
        }
    }
}
