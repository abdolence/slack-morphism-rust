use crate::api::*;
use crate::listener::SlackClientEventsListenerEnvironment;
use crate::socket_mode::clients::*;
use crate::*;
use async_trait::async_trait;
use std::sync::{Arc, RwLock, Weak};

use crate::errors::*;
use log::*;
use slack_morphism_models::socket_mode::SlackSocketModeEvent;

pub(crate) struct SlackSocketModeClientsManager<SCHC, SCWSS>
where
    SCHC:
        SlackClientHttpConnector + SlackSocketModeWssClientsFactory<SCWSS> + Send + Sync + 'static,
    SCWSS: SlackSocketModeWssClient + Send + Sync + 'static,
{
    listener_environment: Arc<SlackClientEventsListenerEnvironment<SCHC>>,
    active_clients: Arc<RwLock<Vec<SCWSS>>>,
    callbacks: Arc<SlackSocketModeListenerCallbacks<SCHC>>,
}

impl<SCHC, SCWSS> SlackSocketModeClientsManager<SCHC, SCWSS>
where
    SCHC:
        SlackClientHttpConnector + SlackSocketModeWssClientsFactory<SCWSS> + Send + Sync + 'static,
    SCWSS: SlackSocketModeWssClient + Send + Sync + 'static,
{
    pub fn new(
        listener_environment: Arc<SlackClientEventsListenerEnvironment<SCHC>>,
        callbacks: SlackSocketModeListenerCallbacks<SCHC>,
    ) -> Self {
        Self {
            listener_environment,
            active_clients: Arc::new(RwLock::new(vec![])),
            callbacks: Arc::new(callbacks),
        }
    }

    pub async fn shutdown(&self) {
        let mut drained_clients: Vec<SCWSS> = {
            let mut clients_write = self.active_clients.write().unwrap();
            let existing_vec = clients_write.drain(..).collect();
            existing_vec
        };

        for client in drained_clients.iter_mut() {
            client.destroy().await;
        }
    }

    pub async fn create_all_clients(
        &self,
        connections_count: u8,
        token: SlackApiToken,
        client_listener: Arc<dyn SlackSocketModeWssClientListener + Sync + Send>,
    ) -> ClientResult<()> {
        let mut clients_write = self.active_clients.write().unwrap();

        for client_id_value in 0..connections_count {
            let wss_client_result = self
                .create_new_wss_client(
                    client_id_value.into(),
                    token.clone(),
                    client_listener.clone(),
                )
                .await?;
            clients_write.push(wss_client_result)
        }

        Ok(())
    }

    async fn create_new_wss_client(
        &self,
        client_id: SlackSocketModeWssClientId,
        token: SlackApiToken,
        client_listener: Arc<dyn SlackSocketModeWssClientListener + Sync + Send + 'static>,
    ) -> ClientResult<SCWSS> {
        let session = self.listener_environment.client.open_session(&token);

        let open_connection_res = session
            .apps_connections_open(&SlackApiAppsConnectionOpenRequest::new())
            .await?;

        self.listener_environment
            .client
            .http_api
            .connector
            .create_wss_client(&open_connection_res.url, client_id, token, client_listener)
            .await
    }

    async fn remove_client(&self, client_id: &SlackSocketModeWssClientId) {
        let mut removed_clients = {
            let mut clients_write = self.active_clients.write().unwrap();

            match clients_write
                .iter()
                .enumerate()
                .find(|(_, client)| *client.id() == *client_id)
            {
                Some((index, _)) => clients_write.drain(index..index).collect::<Vec<SCWSS>>(),
                None => vec![],
            }
        };

        if !removed_clients.is_empty() {
            let removed_client = &mut removed_clients[0];
            removed_client.destroy().await;

            // Reconnect
            match self
                .create_new_wss_client(
                    removed_client.id().clone(),
                    removed_client.token().clone(),
                    removed_client.listener(),
                )
                .await
            {
                Ok(wss_client) => {
                    let mut clients_write = self.active_clients.write().unwrap();
                    clients_write.push(wss_client);
                }
                Err(err) => {
                    error!("Unable to recreate WSS client: {}", err);
                }
            }
        }
    }
}

pub(crate) struct SlackSocketModeClientsManagerListener<SCHC, SCWSS>
where
    SCHC:
        SlackClientHttpConnector + SlackSocketModeWssClientsFactory<SCWSS> + Send + Sync + 'static,
    SCWSS: SlackSocketModeWssClient + Send + Sync + 'static,
{
    clients_manager: Weak<SlackSocketModeClientsManager<SCHC, SCWSS>>,
}

impl<SCHC, SCWSS> SlackSocketModeClientsManagerListener<SCHC, SCWSS>
where
    SCHC:
        SlackClientHttpConnector + SlackSocketModeWssClientsFactory<SCWSS> + Send + Sync + 'static,
    SCWSS: SlackSocketModeWssClient + Send + Sync + 'static,
{
    pub(crate) fn new(manager: Weak<SlackSocketModeClientsManager<SCHC, SCWSS>>) -> Self {
        Self {
            clients_manager: manager,
        }
    }
}

#[async_trait]
impl<SCHC, SCWSS> SlackSocketModeWssClientListener
    for SlackSocketModeClientsManagerListener<SCHC, SCWSS>
where
    SCHC:
        SlackClientHttpConnector + SlackSocketModeWssClientsFactory<SCWSS> + Send + Sync + 'static,
    SCWSS: SlackSocketModeWssClient + Send + Sync + 'static,
{
    async fn on_message(&self, _client_id: &SlackSocketModeWssClientId, message_body: String) {
        if let Some(clients_manager) = self.clients_manager.upgrade() {
            match serde_json::from_str::<SlackSocketModeEvent>(message_body.as_str()).map_err(|e| {
                SlackClientProtocolError {
                    json_error: e,
                    http_response_body: message_body.clone(),
                }
                .into()
            }) {
                Ok(sm_event) => match sm_event {
                    SlackSocketModeEvent::Hello(event) => {
                        clients_manager
                            .callbacks
                            .hello_callback
                            .call(
                                event,
                                clients_manager.listener_environment.client.clone(),
                                clients_manager
                                    .listener_environment
                                    .user_state_storage
                                    .clone(),
                            )
                            .await;
                    }
                },
                Err(err) => {
                    clients_manager.listener_environment.error_handler.clone()(
                        err,
                        clients_manager.listener_environment.client.clone(),
                        clients_manager
                            .listener_environment
                            .user_state_storage
                            .clone(),
                    );
                }
            }
        }
    }

    async fn on_disconnect(&self, client_id: &SlackSocketModeWssClientId) {
        if let Some(clients_manager) = self.clients_manager.upgrade() {
            clients_manager.remove_client(client_id).await
        }
    }
}
