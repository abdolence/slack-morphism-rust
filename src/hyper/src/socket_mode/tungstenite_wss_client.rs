use futures::{SinkExt, StreamExt};
use rvstruct::*;
use slack_morphism::api::SlackApiAppsConnectionOpenRequest;
use slack_morphism::errors::*;
use slack_morphism::listener::SlackClientEventsListenerEnvironment;
use slack_morphism::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::SystemTime;
use tokio::net::TcpStream;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::RwLock;
use tokio_tungstenite::*;
use tracing::*;
use url::Url;

#[derive(Clone)]
pub struct SlackTungsteniteWssClientIdentity {
    pub id: SlackSocketModeWssClientId,
    pub token: SlackApiToken,
    pub client_listener: Arc<dyn SlackSocketModeClientListener + Sync + Send>,
    pub config: SlackClientSocketModeConfig,
}

#[derive(Clone)]
pub struct SlackTungsteniteWssClient<SCHC>
where
    SCHC: SlackClientHttpConnector + Send + Sync,
{
    pub identity: SlackTungsteniteWssClientIdentity,
    command_writer: Arc<RwLock<UnboundedSender<SlackTungsteniteWssClientCommand>>>,
    command_reader: Arc<RwLock<Option<UnboundedReceiver<SlackTungsteniteWssClientCommand>>>>,
    destroyed: Arc<AtomicBool>,
    listener_environment: Arc<SlackClientEventsListenerEnvironment<SCHC>>,
}

#[derive(Clone, Debug)]
enum SlackTungsteniteWssClientCommand {
    Message(String),
    Ping,
    Pong(Vec<u8>),
    Exit,
}

impl<SCHC> SlackTungsteniteWssClient<SCHC>
where
    SCHC: SlackClientHttpConnector + Send + Sync + 'static,
{
    pub fn new(
        id: SlackSocketModeWssClientId,
        client_listener: Arc<dyn SlackSocketModeClientListener + Sync + Send>,
        token: &SlackApiToken,
        config: &SlackClientSocketModeConfig,
        listener_environment: Arc<SlackClientEventsListenerEnvironment<SCHC>>,
    ) -> Self {
        let identity = SlackTungsteniteWssClientIdentity {
            id,
            client_listener,
            token: token.clone(),
            config: config.clone(),
        };

        let (tx, rx): (
            UnboundedSender<SlackTungsteniteWssClientCommand>,
            UnboundedReceiver<SlackTungsteniteWssClientCommand>,
        ) = tokio::sync::mpsc::unbounded_channel();

        SlackTungsteniteWssClient {
            identity,
            command_writer: Arc::new(RwLock::new(tx)),
            command_reader: Arc::new(RwLock::new(Some(rx))),
            destroyed: Arc::new(AtomicBool::new(false)),
            listener_environment,
        }
    }

    async fn try_to_connect(
        identity: &SlackTungsteniteWssClientIdentity,
        listener_environment: Arc<SlackClientEventsListenerEnvironment<SCHC>>,
    ) -> Option<WebSocketStream<MaybeTlsStream<TcpStream>>> {
        let session = listener_environment.client.open_session(&identity.token);

        trace!(
            "[{}] Receiving WSS URL to connect through Slack app.connections.open()",
            identity.id.to_string()
        );

        match session
            .apps_connections_open(&SlackApiAppsConnectionOpenRequest::new())
            .await
        {
            Ok(open_connection_res) => {
                let open_connection_res_url = if identity.config.debug_connections {
                    format!("{}&debug_reconnects=true", open_connection_res.url.value()).into()
                } else {
                    open_connection_res.url
                };

                trace!(
                    "[{}] Connecting to: {}",
                    identity.id.to_string(),
                    open_connection_res_url.value()
                );

                let url_to_connect = Url::parse(open_connection_res_url.value()).unwrap();

                match connect_async(&url_to_connect).await {
                    Ok((_, response))
                        if !response.status().is_success()
                            && !response.status().is_informational() =>
                    {
                        error!(
                            slack_wss_client_id = identity.id.to_string().as_str(),
                            "[{}] Unable to connect {}: {}",
                            identity.id.to_string(),
                            url_to_connect,
                            response.status()
                        );

                        None
                    }
                    Err(err) => {
                        error!(
                            slack_wss_client_id = identity.id.to_string().as_str(),
                            "[{}] Unable to connect {}: {:?}",
                            identity.id.to_string(),
                            url_to_connect,
                            err
                        );

                        None
                    }
                    Ok((wss_stream, _)) => {
                        debug!(
                            slack_wss_client_id = identity.id.to_string().as_str(),
                            "[{}] Connected to {}",
                            identity.id.to_string(),
                            url_to_connect
                        );
                        Some(wss_stream)
                    }
                }
            }
            Err(err) => {
                error!(
                    "[{}] Unable to create WSS url: {}",
                    identity.id.to_string(),
                    err
                );
                None
            }
        }
    }

    async fn connect_with_reconnections(
        identity: &SlackTungsteniteWssClientIdentity,
        listener_environment: Arc<SlackClientEventsListenerEnvironment<SCHC>>,
        destroyed: Arc<AtomicBool>,
    ) -> ClientResult<WebSocketStream<MaybeTlsStream<TcpStream>>> {
        let mut maybe_stream = Self::try_to_connect(identity, listener_environment.clone()).await;
        loop {
            if let Some(wss_stream) = maybe_stream {
                return Ok(wss_stream);
            } else if !destroyed.load(Ordering::Relaxed) {
                trace!(
                    slack_wss_client_id = identity.id.to_string().as_str(),
                    "[{}] Reconnecting after {} seconds...",
                    identity.id.to_string(),
                    identity.config.reconnect_timeout_in_seconds
                );
                let mut interval = tokio::time::interval(std::time::Duration::from_secs(
                    identity.config.reconnect_timeout_in_seconds,
                ));

                interval.tick().await;
                interval.tick().await;

                maybe_stream = Self::try_to_connect(identity, listener_environment.clone()).await;
            } else {
                return Err(SlackClientError::EndOfStream(
                    SlackClientEndOfStreamError::new(),
                ));
            }
        }
    }

    async fn connect_spawn(
        mut rx: UnboundedReceiver<SlackTungsteniteWssClientCommand>,
        tx: UnboundedSender<SlackTungsteniteWssClientCommand>,
        identity: SlackTungsteniteWssClientIdentity,
        listener_environment: Arc<SlackClientEventsListenerEnvironment<SCHC>>,
        destroyed: Arc<AtomicBool>,
        initial_wait_timeout: u64,
    ) -> ClientResult<()> {
        if initial_wait_timeout > 0 {
            trace!(
                slack_wss_client_id = identity.id.to_string().as_str(),
                "[{}] Postponed connection for {} seconds (backoff timeout for multiple connections)",
                identity.id.to_string(),
                initial_wait_timeout
            );
            let mut interval =
                tokio::time::interval(std::time::Duration::from_secs(initial_wait_timeout));

            interval.tick().await;
            interval.tick().await;
        }

        let wss_stream =
            Self::connect_with_reconnections(&identity, listener_environment, destroyed).await?;

        let (mut writer, mut reader) = wss_stream.split();

        struct PongState {
            time: SystemTime,
        }
        let last_time_pong_received: Arc<RwLock<PongState>> = Arc::new(RwLock::new(PongState {
            time: SystemTime::now(),
        }));

        {
            let thread_identity = identity.clone();
            let thread_last_time_pong_received = last_time_pong_received.clone();

            tokio::spawn(async move {
                while let Some(message) = rx.recv().await {
                    match message {
                        SlackTungsteniteWssClientCommand::Message(body) => {
                            if writer
                                .send(tokio_tungstenite::tungstenite::Message::Text(body))
                                .await
                                .is_err()
                            {
                                rx.close()
                            }
                        }
                        SlackTungsteniteWssClientCommand::Pong(body) => {
                            trace!(
                                slack_wss_client_id = thread_identity.id.to_string().as_str(),
                                "[{}] Pong to Slack: {:?}",
                                thread_identity.id.to_string(),
                                body
                            );
                            if writer
                                .send(tokio_tungstenite::tungstenite::Message::Pong(body))
                                .await
                                .is_err()
                            {
                                rx.close()
                            }
                        }
                        SlackTungsteniteWssClientCommand::Ping => {
                            let body: [u8; 5] = rand::random();
                            trace!(
                                slack_wss_client_id = thread_identity.id.to_string().as_str(),
                                "[{}] Ping to Slack: {:?}",
                                thread_identity.id.to_string(),
                                body
                            );

                            let seen_pong_time_in_secs = {
                                let last_pong = thread_last_time_pong_received.read().await;

                                SystemTime::now()
                                    .duration_since(last_pong.time)
                                    .unwrap()
                                    .as_secs()
                            };

                            if seen_pong_time_in_secs
                                > thread_identity.config.ping_interval_in_seconds
                                    * thread_identity.config.ping_failure_threshold_times
                            {
                                warn!(
                                    slack_wss_client_id = thread_identity.id.to_string().as_str(),
                                    "[{}] Haven't seen any pong from Slack since {} seconds ago",
                                    thread_identity.id.to_string(),
                                    seen_pong_time_in_secs
                                );
                                rx.close()
                            } else if let Err(err) = writer
                                .send(tokio_tungstenite::tungstenite::Message::Ping(body.to_vec()))
                                .await
                            {
                                warn!(
                                    slack_wss_client_id = thread_identity.id.to_string().as_str(),
                                    "[{}] Ping slack failed with: {:?}",
                                    thread_identity.id.to_string(),
                                    err
                                );
                                rx.close()
                            }
                        }
                        SlackTungsteniteWssClientCommand::Exit => {
                            writer.close().await.unwrap_or(());
                            rx.close();
                            trace!(
                                slack_wss_client_id = thread_identity.id.to_string().as_str(),
                                "[{}] WSS client command channel has been closed",
                                thread_identity.id.to_string()
                            );
                        }
                    }
                }
            });
        }

        {
            let thread_identity = identity.clone();
            let ping_tx = tx.clone();
            tokio::spawn(async move {
                let mut interval = tokio::time::interval(std::time::Duration::from_secs(
                    thread_identity.config.ping_interval_in_seconds,
                ));

                loop {
                    interval.tick().await;
                    if !ping_tx.is_closed() {
                        if ping_tx
                            .send(SlackTungsteniteWssClientCommand::Ping)
                            .is_err()
                        {
                            break;
                        }
                    } else {
                        thread_identity
                            .client_listener
                            .on_disconnect(&thread_identity.id)
                            .await;
                        break;
                    }
                }
            });
        }

        {
            let thread_identity = identity.clone();
            let thread_last_time_pong_received = last_time_pong_received;

            tokio::spawn(async move {
                while let Some(message) = reader.next().await {
                    match message {
                        Ok(tokio_tungstenite::tungstenite::Message::Text(body)) => {
                            trace!(
                                slack_wss_client_id = thread_identity.id.to_string().as_str(),
                                "[{}] Received from Slack: {:?}",
                                thread_identity.id.to_string(),
                                body
                            );
                            if let Some(reply) = thread_identity
                                .client_listener
                                .on_message(&thread_identity.id, body)
                                .await
                            {
                                trace!(
                                    slack_wss_client_id = thread_identity.id.to_string().as_str(),
                                    "[{}] Sending reply to Slack: {:?}",
                                    thread_identity.id.to_string(),
                                    reply
                                );
                                tx.send(SlackTungsteniteWssClientCommand::Message(reply))
                                    .unwrap_or(());
                            }
                        }
                        Ok(tokio_tungstenite::tungstenite::Message::Ping(body)) => {
                            trace!(
                                slack_wss_client_id = thread_identity.id.to_string().as_str(),
                                "[{}] Ping from Slack: {:?}",
                                thread_identity.id.to_string(),
                                body
                            );
                            tx.send(SlackTungsteniteWssClientCommand::Pong(body))
                                .unwrap_or(());
                        }
                        Ok(tokio_tungstenite::tungstenite::Message::Pong(body)) => {
                            trace!(
                                slack_wss_client_id = thread_identity.id.to_string().as_str(),
                                "[{}] Pong from Slack: {:?}",
                                thread_identity.id.to_string(),
                                body
                            );
                            let mut last_pong = thread_last_time_pong_received.write().await;
                            last_pong.time = SystemTime::now();
                        }
                        Ok(tokio_tungstenite::tungstenite::Message::Binary(body)) => {
                            warn!(
                                slack_wss_client_id = thread_identity.id.to_string().as_str(),
                                "[{}] Unexpected binary received from Slack: {:?}",
                                thread_identity.id.to_string(),
                                body
                            );
                            thread_identity
                                .client_listener
                                .on_error(Box::new(SlackClientError::SocketModeProtocolError(
                                    SlackClientSocketModeProtocolError::new(format!(
                                        "Unexpected binary received from Slack: {:?}",
                                        body
                                    )),
                                )))
                                .await;
                        }
                        Ok(tokio_tungstenite::tungstenite::Message::Close(body)) => {
                            debug!(
                                slack_wss_client_id = thread_identity.id.to_string().as_str(),
                                "[{}] Shutting down WSS channel: {:?}",
                                thread_identity.id.to_string(),
                                body
                            );
                            thread_identity
                                .client_listener
                                .on_disconnect(&thread_identity.id)
                                .await
                        }
                        Err(err) => {
                            error!(
                                slack_wss_client_id = thread_identity.id.to_string().as_str(),
                                "[{}] Slack WSS error: {:?}",
                                thread_identity.id.to_string(),
                                err
                            );
                            thread_identity
                                .client_listener
                                .on_error(Box::new(SlackClientError::SocketModeProtocolError(
                                    SlackClientSocketModeProtocolError::new(format!(
                                        "Unexpected binary received from Slack: {:?}",
                                        err
                                    )),
                                )))
                                .await;
                            thread_identity
                                .client_listener
                                .on_disconnect(&thread_identity.id)
                                .await
                        }
                    }
                }
            });
        }

        Ok(())
    }

    pub async fn connect(&self, initial_wait_timeout: u64) -> ClientResult<()> {
        let maybe_rx = {
            let mut rx_writer = self.command_reader.write().await;
            rx_writer.take()
        };

        match maybe_rx {
            Some(rx) => {
                let tx = {
                    let tx_writer = self.command_writer.write().await;
                    (*tx_writer).clone()
                };

                tokio::spawn(Self::connect_spawn(
                    rx,
                    tx,
                    self.identity.clone(),
                    self.listener_environment.clone(),
                    self.destroyed.clone(),
                    initial_wait_timeout,
                ));
                Ok(())
            }
            None => Err(SlackClientError::EndOfStream(
                SlackClientEndOfStreamError::new(),
            )),
        }
    }

    pub async fn shutdown_channel(&mut self) {
        debug!(
            slack_wss_client_id = self.identity.id.to_string().as_str(),
            "[{}] Destroying WSS client",
            self.identity.id.to_string()
        );
        let sender = {
            let commands_writer = self.command_writer.write().await;
            (*commands_writer).clone()
        };

        sender
            .send(SlackTungsteniteWssClientCommand::Exit)
            .unwrap_or(());

        self.destroyed.store(true, Ordering::Relaxed);
    }

    pub async fn start(&self, initial_wait_timeout: u64) {
        self.connect(initial_wait_timeout).await.unwrap_or(());
    }
}
