use crate::socket_mode::clients::*;
use crate::*;
use async_trait::async_trait;
use slack_morphism_models::socket_mode::*;
use std::sync::Weak;

use crate::errors::*;
use crate::socket_mode::clients_manager::SlackSocketModeClientsManager;
use log::*;
use slack_morphism_models::socket_mode::SlackSocketModeEvent;

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
                SlackClientProtocolError::new(e)
                    .with_json_body(message_body.to_string())
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
                                clients_manager.listener_environment.user_state.clone(),
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
                        let reply =
                            serde_json::to_string(&SlackSocketModeEventCommonAcknowledge::new(
                                event.envelope_params.envelope_id,
                            ))
                            .unwrap();

                        match clients_manager
                            .callbacks
                            .interaction_callback
                            .call(
                                event.payload.clone(),
                                clients_manager.listener_environment.client.clone(),
                                clients_manager.listener_environment.user_state.clone(),
                            )
                            .await
                        {
                            Ok(_) => Some(reply),
                            Err(err) => {
                                if clients_manager.listener_environment.error_handler.clone()(
                                    err,
                                    clients_manager.listener_environment.client.clone(),
                                    clients_manager.listener_environment.user_state.clone(),
                                )
                                .is_success()
                                {
                                    Some(reply)
                                } else {
                                    None
                                }
                            }
                        }
                    }
                    SlackSocketModeEvent::EventsApi(event) => {
                        let reply =
                            serde_json::to_string(&SlackSocketModeEventCommonAcknowledge::new(
                                event.envelope_params.envelope_id,
                            ))
                            .unwrap();

                        match clients_manager
                            .callbacks
                            .push_events_callback
                            .call(
                                event.payload.clone(),
                                clients_manager.listener_environment.client.clone(),
                                clients_manager.listener_environment.user_state.clone(),
                            )
                            .await
                        {
                            Ok(_) => Some(reply),
                            Err(err) => {
                                if clients_manager.listener_environment.error_handler.clone()(
                                    err,
                                    clients_manager.listener_environment.client.clone(),
                                    clients_manager.listener_environment.user_state.clone(),
                                )
                                .is_success()
                                {
                                    Some(reply)
                                } else {
                                    None
                                }
                            }
                        }
                    }

                    SlackSocketModeEvent::SlashCommands(event) => {
                        match clients_manager
                            .callbacks
                            .command_callback
                            .call(
                                event.payload.clone(),
                                clients_manager.listener_environment.client.clone(),
                                clients_manager.listener_environment.user_state.clone(),
                            )
                            .await
                        {
                            Ok(reply) => Some(
                                serde_json::to_string(
                                    &SlackSocketModeCommandEventAck::new(
                                        SlackSocketModeEventCommonAcknowledge::new(
                                            event.envelope_params.envelope_id,
                                        ),
                                    )
                                    .with_payload(reply),
                                )
                                .unwrap(),
                            ),
                            Err(err) => {
                                if clients_manager.listener_environment.error_handler.clone()(
                                    err,
                                    clients_manager.listener_environment.client.clone(),
                                    clients_manager.listener_environment.user_state.clone(),
                                )
                                .is_success()
                                {
                                    Some(
                                        serde_json::to_string(
                                            &SlackSocketModeCommandEventAck::new(
                                                SlackSocketModeEventCommonAcknowledge::new(
                                                    event.envelope_params.envelope_id,
                                                ),
                                            ),
                                        )
                                        .unwrap(),
                                    )
                                } else {
                                    None
                                }
                            }
                        }
                    }
                },
                Err(err) => {
                    clients_manager.listener_environment.error_handler.clone()(
                        err,
                        clients_manager.listener_environment.client.clone(),
                        clients_manager.listener_environment.user_state.clone(),
                    );
                    None
                }
            }
        } else {
            None
        }
    }

    async fn on_error(&self, error: Box<dyn std::error::Error + Send + Sync>) {
        if let Some(clients_manager) = self.clients_manager.upgrade() {
            clients_manager.listener_environment.error_handler.clone()(
                error,
                clients_manager.listener_environment.client.clone(),
                clients_manager.listener_environment.user_state.clone(),
            );
        }
    }

    async fn on_disconnect(&self, client_id: &SlackSocketModeWssClientId) {
        if let Some(clients_manager) = self.clients_manager.upgrade() {
            clients_manager.remove_client(client_id).await
        }
    }
}
