use crate::api::*;
use crate::listener::SlackClientEventsListenerEnvironment;
use crate::socket_mode::clients::*;
use crate::*;
use async_trait::async_trait;
use slack_morphism_models::socket_mode::*;
use std::ops::Range;
use std::sync::{Arc, RwLock, Weak};

use crate::errors::*;
use log::*;
use rvstruct::*;
use slack_morphism_models::socket_mode::SlackSocketModeEvent;

pub(crate) struct SlackSocketModeClientsManager<SCHC, SCWSS>
where
    SCHC:
        SlackClientHttpConnector + SlackSocketModeWssClientsFactory<SCWSS> + Send + Sync + 'static,
    SCWSS: SlackSocketModeWssClient + Send + Sync + 'static,
{
    listener_environment: Arc<SlackClientEventsListenerEnvironment<SCHC>>,
    active_clients: Arc<RwLock<Vec<SlackSocketModeClient<SCWSS>>>>,
    callbacks: Arc<SlackSocketModeListenerCallbacks<SCHC>>,
}

struct SlackSocketModeClient<SCWSS>
where
    SCWSS: SlackSocketModeWssClient + Send + Sync + 'static,
{
    id: SlackSocketModeWssClientId,
    wss_client: SCWSS,
    token: SlackApiToken,
    config: SlackClientSocketModeConfig,
    listener: Arc<dyn SlackSocketModeWssClientListener + Sync + Send>,
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
        let mut drained_clients: Vec<SlackSocketModeClient<SCWSS>> = {
            let mut clients_write = self.active_clients.write().unwrap();
            let existing_vec = clients_write.drain(..).collect();
            existing_vec
        };

        for client in drained_clients.iter_mut() {
            client.wss_client.destroy().await;
        }
    }

    fn get_next_clients_range_indices(&self, config: &SlackClientSocketModeConfig) -> Range<u32> {
        let clients_read = self.active_clients.read().unwrap();
        let last_client_id_value = clients_read.len();
        last_client_id_value as u32
            ..(last_client_id_value as u32 + config.max_connections_count) as u32
    }

    pub async fn create_all_clients(
        &self,
        config: SlackClientSocketModeConfig,
        token: SlackApiToken,
        client_listener: Arc<dyn SlackSocketModeWssClientListener + Sync + Send>,
    ) -> ClientResult<()> {
        let new_clients_range = self.get_next_clients_range_indices(&config);
        {
            let mut clients_write = self.active_clients.write().unwrap();

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

    pub async fn start_clients(&self, config: &SlackClientSocketModeConfig) {
        let clients_read = self.active_clients.read().unwrap();
        for client_id_value in 0..clients_read.len() {
            clients_read[client_id_value]
                .wss_client
                .start(
                    client_id_value as u64 * config.initial_backoff_in_seconds,
                    config.reconnect_timeout_in_seconds,
                )
                .await
        }
    }

    async fn create_new_wss_client(
        &self,
        client_id: SlackSocketModeWssClientId,
        token: SlackApiToken,
        client_listener: Arc<dyn SlackSocketModeWssClientListener + Sync + Send + 'static>,
        config: SlackClientSocketModeConfig,
    ) -> ClientResult<SlackSocketModeClient<SCWSS>> {
        let session = self.listener_environment.client.open_session(&token);

        let open_connection_res = session
            .apps_connections_open(&SlackApiAppsConnectionOpenRequest::new())
            .await?;

        let open_connection_res_url = if config.debug_connections {
            format!("{}&debug_reconnects=true", open_connection_res.url.value()).into()
        } else {
            open_connection_res.url
        };

        trace!(
            "Creating a new WSS client: {:?}. Url: {}",
            client_id,
            open_connection_res_url.value()
        );

        let wss_client = self
            .listener_environment
            .client
            .http_api
            .connector
            .create_wss_client(
                &open_connection_res_url,
                client_id.clone(),
                token.clone(),
                client_listener.clone(),
            )
            .await?;

        Ok(SlackSocketModeClient {
            id: client_id.clone(),
            wss_client,
            token: token.clone(),
            config: config.clone(),
            listener: client_listener.clone(),
        })
    }

    async fn remove_client(&self, client_id: &SlackSocketModeWssClientId) {
        debug!("[{}] Removing client", client_id.to_string());

        let mut removed_clients = {
            let mut clients_write = self.active_clients.write().unwrap();

            match clients_write
                .iter()
                .enumerate()
                .find(|(_, client)| client.id == *client_id)
            {
                Some((index, _)) => clients_write
                    .drain(index..=index)
                    .collect::<Vec<SlackSocketModeClient<SCWSS>>>(),
                None => vec![],
            }
        };

        if !removed_clients.is_empty() {
            let removed_client = &mut removed_clients[0];
            removed_client.wss_client.destroy().await;

            // Reconnect
            trace!("[{}] Reconnecting...", client_id.to_string());
            match self
                .create_new_wss_client(
                    removed_client.id.new_reconnected_id(),
                    removed_client.token.clone(),
                    removed_client.listener.clone(),
                    removed_client.config.clone(),
                )
                .await
            {
                Ok(client) => {
                    client
                        .wss_client
                        .start(0, removed_client.config.reconnect_timeout_in_seconds)
                        .await;
                    let mut clients_write = self.active_clients.write().unwrap();
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
    async fn on_message(
        &self,
        client_id: &SlackSocketModeWssClientId,
        message_body: String,
    ) -> Option<String> {
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
                        None
                    }
                    SlackSocketModeEvent::Disconnect(event) => {
                        trace!(
                            "[{}] Received socket mode disconnected event: {:?}",
                            client_id.to_string(),
                            event
                        );
                        clients_manager.remove_client(client_id).await;
                        None
                    }
                    SlackSocketModeEvent::Interactive(event) => {
                        clients_manager
                            .callbacks
                            .interaction_callback
                            .call(
                                event.payload.clone(),
                                clients_manager.listener_environment.client.clone(),
                                clients_manager
                                    .listener_environment
                                    .user_state_storage
                                    .clone(),
                            )
                            .await;
                        Some(
                            serde_json::to_string(&SlackSocketModeEventCommonAcknowledge::new(
                                event.envelope_params.envelope_id,
                            ))
                            .unwrap(),
                        )
                    }
                    SlackSocketModeEvent::EventsApi(event) => {
                        clients_manager
                            .callbacks
                            .push_events_callback
                            .call(
                                event.payload.clone(),
                                clients_manager.listener_environment.client.clone(),
                                clients_manager
                                    .listener_environment
                                    .user_state_storage
                                    .clone(),
                            )
                            .await;
                        Some(
                            serde_json::to_string(&SlackSocketModeEventCommonAcknowledge::new(
                                event.envelope_params.envelope_id,
                            ))
                            .unwrap(),
                        )
                    }

                    SlackSocketModeEvent::SlashCommands(event) => {
                        if let Ok(response) = clients_manager
                            .callbacks
                            .command_callback
                            .call(
                                event.payload.clone(),
                                clients_manager.listener_environment.client.clone(),
                                clients_manager
                                    .listener_environment
                                    .user_state_storage
                                    .clone(),
                            )
                            .await
                        {
                            Some(
                                serde_json::to_string(
                                    &SlackSocketModeCommandEventAck::new(
                                        SlackSocketModeEventCommonAcknowledge::new(
                                            event.envelope_params.envelope_id,
                                        ),
                                    )
                                    .with_payload(response),
                                )
                                .unwrap(),
                            )
                        } else {
                            None
                        }
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
                    None
                }
            }
        } else {
            None
        }
    }

    async fn on_disconnect(&self, client_id: &SlackSocketModeWssClientId) {
        if let Some(clients_manager) = self.clients_manager.upgrade() {
            clients_manager.remove_client(client_id).await
        }
    }
}
