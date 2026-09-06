use crate::models::socket_mode::*;
use crate::socket_mode::clients_manager::*;
use crate::*;
use async_trait::async_trait;
use std::sync::{Arc, Weak};

use crate::errors::*;
use crate::events::SlackInteractionResponse;
use crate::listener::SlackClientEventsListenerEnvironment;
use crate::socket_mode::wss_client_id::SlackSocketModeWssClientId;
use rvstruct::ValueStruct;
use serde::Serialize;
use tracing::*;

#[async_trait]
pub trait SlackSocketModeClientListener {
    async fn on_message(
        &self,
        client_id: &SlackSocketModeWssClientId,
        message_body: String,
    ) -> Option<String>;

    async fn on_error(&self, error: BoxError);

    async fn on_disconnect(&self, client_id: &SlackSocketModeWssClientId);
}

pub(crate) struct SlackSocketModeClientsManagerListener<SCHC>
where
    SCHC: SlackClientHttpConnector + SlackSocketModeClientsManagerFactory<SCHC> + Send + Sync,
{
    clients_manager: Weak<dyn SlackSocketModeClientsManager + Send + Sync>,
    listener_environment: Arc<SlackClientEventsListenerEnvironment<SCHC>>,
    callbacks: Arc<SlackSocketModeListenerCallbacks<SCHC>>,
}

impl<SCHC> SlackSocketModeClientsManagerListener<SCHC>
where
    SCHC: SlackClientHttpConnector + SlackSocketModeClientsManagerFactory<SCHC> + Send + Sync,
{
    pub(crate) fn new(
        manager: Weak<dyn SlackSocketModeClientsManager + Send + Sync>,
        listener_environment: Arc<SlackClientEventsListenerEnvironment<SCHC>>,
        callbacks: SlackSocketModeListenerCallbacks<SCHC>,
    ) -> Self {
        Self {
            clients_manager: manager,
            listener_environment,
            callbacks: Arc::new(callbacks),
        }
    }

    /// Serialises a Socket Mode acknowledgement frame.
    ///
    /// A failure is reported to the configured error handler and yields `None`, so no
    /// acknowledgement is sent and Slack redelivers the envelope instead of the client
    /// panicking.
    fn ack_frame<A: Serialize>(&self, ack: &A) -> Option<String> {
        match serde_json::to_string(ack) {
            Ok(frame) => Some(frame),
            Err(err) => {
                self.listener_environment.error_handler.clone()(
                    SlackClientProtocolError::new(err).into(),
                    self.listener_environment.client.clone(),
                    self.listener_environment.user_state.clone(),
                );
                None
            }
        }
    }
}

#[async_trait]
impl<SCHC> SlackSocketModeClientListener for SlackSocketModeClientsManagerListener<SCHC>
where
    SCHC: SlackClientHttpConnector
        + SlackSocketModeClientsManagerFactory<SCHC>
        + Send
        + Sync
        + 'static,
{
    async fn on_message(
        &self,
        client_id: &SlackSocketModeWssClientId,
        message_body: String,
    ) -> Option<String> {
        if let Some(clients_manager) = self.clients_manager.upgrade() {
            match serde_json::from_str::<SlackSocketModeEvent>(message_body.as_str()).map_err(|e| {
                SlackClientProtocolError::new(e)
                    .with_json_body(message_body)
                    .into()
            }) {
                Ok(sm_event) => match sm_event {
                    SlackSocketModeEvent::Hello(event) => {
                        self.callbacks
                            .hello_callback
                            .call(
                                event,
                                self.listener_environment.client.clone(),
                                self.listener_environment.user_state.clone(),
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
                        clients_manager.restart_client(client_id).await;
                        None
                    }
                    SlackSocketModeEvent::Interactive(event) => {
                        let envelope_id = event.envelope_params.envelope_id.clone();
                        let accepts_response_payload =
                            event.envelope_params.accepts_response_payload;
                        let bare_ack = || {
                            self.ack_frame(&SlackSocketModeEventCommonAcknowledge::new(
                                envelope_id.clone(),
                            ))
                        };

                        match self
                            .callbacks
                            .interaction_callback
                            .call(
                                event.payload.clone(),
                                self.listener_environment.client.clone(),
                                self.listener_environment.user_state.clone(),
                            )
                            .await
                        {
                            Ok(SlackInteractionResponse::Empty) => bare_ack(),
                            Ok(payload) if accepts_response_payload => self.ack_frame(
                                &SlackSocketModeInteractiveEventAck::new(
                                    SlackSocketModeEventCommonAcknowledge::new(envelope_id.clone()),
                                )
                                .with_payload(payload),
                            ),
                            Ok(payload) => {
                                let response_kind = match payload {
                                    SlackInteractionResponse::Empty => "empty",
                                    SlackInteractionResponse::ViewSubmission(_) => {
                                        "view_submission"
                                    }
                                    SlackInteractionResponse::BlockSuggestion(_) => {
                                        "block_suggestion"
                                    }
                                };
                                warn!(
                                    "[{}] Dropping a '{}' interaction response for the envelope '{}': Slack didn't accept a response payload for it",
                                    client_id.to_string(),
                                    response_kind,
                                    envelope_id.value()
                                );
                                bare_ack()
                            }
                            Err(err) => {
                                if self.listener_environment.error_handler.clone()(
                                    err,
                                    self.listener_environment.client.clone(),
                                    self.listener_environment.user_state.clone(),
                                )
                                .is_success()
                                {
                                    bare_ack()
                                } else {
                                    None
                                }
                            }
                        }
                    }
                    SlackSocketModeEvent::EventsApi(event) => {
                        let bare_ack = || {
                            self.ack_frame(&SlackSocketModeEventCommonAcknowledge::new(
                                event.envelope_params.envelope_id.clone(),
                            ))
                        };

                        match self
                            .callbacks
                            .push_events_callback
                            .call(
                                event.payload.clone(),
                                self.listener_environment.client.clone(),
                                self.listener_environment.user_state.clone(),
                            )
                            .await
                        {
                            Ok(_) => bare_ack(),
                            Err(err) => {
                                if self.listener_environment.error_handler.clone()(
                                    err,
                                    self.listener_environment.client.clone(),
                                    self.listener_environment.user_state.clone(),
                                )
                                .is_success()
                                {
                                    bare_ack()
                                } else {
                                    None
                                }
                            }
                        }
                    }

                    SlackSocketModeEvent::SlashCommands(event) => {
                        match self
                            .callbacks
                            .command_callback
                            .call(
                                event.payload.clone(),
                                self.listener_environment.client.clone(),
                                self.listener_environment.user_state.clone(),
                            )
                            .await
                        {
                            Ok(reply) => self.ack_frame(
                                &SlackSocketModeCommandEventAck::new(
                                    SlackSocketModeEventCommonAcknowledge::new(
                                        event.envelope_params.envelope_id,
                                    ),
                                )
                                .with_payload(reply),
                            ),
                            Err(err) => {
                                if self.listener_environment.error_handler.clone()(
                                    err,
                                    self.listener_environment.client.clone(),
                                    self.listener_environment.user_state.clone(),
                                )
                                .is_success()
                                {
                                    self.ack_frame(&SlackSocketModeCommandEventAck::new(
                                        SlackSocketModeEventCommonAcknowledge::new(
                                            event.envelope_params.envelope_id,
                                        ),
                                    ))
                                } else {
                                    None
                                }
                            }
                        }
                    }
                },
                Err(err) => {
                    self.listener_environment.error_handler.clone()(
                        err,
                        self.listener_environment.client.clone(),
                        self.listener_environment.user_state.clone(),
                    );
                    None
                }
            }
        } else {
            None
        }
    }

    async fn on_error(&self, error: BoxError) {
        self.listener_environment.error_handler.clone()(
            error,
            self.listener_environment.client.clone(),
            self.listener_environment.user_state.clone(),
        );
    }

    async fn on_disconnect(&self, client_id: &SlackSocketModeWssClientId) {
        if let Some(clients_manager) = self.clients_manager.upgrade() {
            clients_manager.restart_client(client_id).await
        }
    }
}
