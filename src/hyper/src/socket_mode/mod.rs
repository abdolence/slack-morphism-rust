use crate::SlackClientHyperConnector;
use async_trait::async_trait;
use futures::{SinkExt, StreamExt};
use log::*;
use rvstruct::*;
use slack_morphism::clients::{
    SlackSocketModeWssClient, SlackSocketModeWssClientId, SlackSocketModeWssClientListener,
    SlackSocketModeWssClientsFactory,
};
use slack_morphism::errors::*;
use slack_morphism::*;
use slack_morphism_models::SlackWebSocketsUrl;
use hyper::client::connect::Connect;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use std::time::SystemTime;
use tokio::net::TcpStream;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio_tungstenite::*;
use url::Url;

#[derive(Clone)]
pub struct SlackTungsteniteWssClient {
    id: SlackSocketModeWssClientId,
    token: SlackApiToken,
    client_listener: Arc<dyn SlackSocketModeWssClientListener + Sync + Send>,
    url: Url,
    command_writer: Arc<RwLock<Option<UnboundedSender<SlackTungsteniteWssClientCommand>>>>,
    destroyed: Arc<AtomicBool>,
}

#[derive(Clone, Debug)]
enum SlackTungsteniteWssClientCommand {
    Message(String),
    Ping,
    Pong(Vec<u8>),
    Exit,
}

impl SlackTungsteniteWssClient {
    pub fn new(
        wss_url: &SlackWebSocketsUrl,
        id: SlackSocketModeWssClientId,
        token: SlackApiToken,
        client_listener: Arc<dyn SlackSocketModeWssClientListener + Sync + Send>,
    ) -> Self {
        let url_to_connect = Url::parse(wss_url.value()).unwrap();

        SlackTungsteniteWssClient {
            id,
            token,
            client_listener,
            url: url_to_connect,
            command_writer: Arc::new(RwLock::new(None)),
            destroyed: Arc::new(AtomicBool::new(false)),
        }
    }

    async fn try_to_connect(&self) -> Option<WebSocketStream<MaybeTlsStream<TcpStream>>> {
        match connect_async(&self.url).await {
            Ok((_, response))
                if !response.status().is_success() && !response.status().is_informational() =>
            {
                error!(
                    "[{}] Unable to connect {}: {}",
                    self.id.to_string(),
                    self.url,
                    response.status()
                );

                None
            }
            Err(err) => {
                error!(
                    "[{}] Unable to connect {}: {:?}",
                    self.id.to_string(),
                    self.url,
                    err
                );

                None
            }
            Ok((wss_stream, _)) => Some(wss_stream),
        }
    }

    async fn connect_with_reconnections(
        &self,
        reconnect_timeout: u64,
    ) -> ClientResult<WebSocketStream<MaybeTlsStream<TcpStream>>> {
        let mut maybe_stream = self.try_to_connect().await;
        loop {
            if let Some(wss_stream) = maybe_stream {
                return Ok(wss_stream);
            } else if !self.destroyed.load(Ordering::Relaxed) {
                trace!(
                    "[{}] Reconnecting after {} seconds...",
                    self.id.to_string(),
                    reconnect_timeout
                );
                let mut interval =
                    tokio::time::interval(std::time::Duration::from_secs(reconnect_timeout));

                interval.tick().await;
                interval.tick().await;

                maybe_stream = self.try_to_connect().await;
            } else {
                return Err(SlackClientError::EndOfStream(
                    SlackClientEndOfStreamError::new(),
                ));
            }
        }
    }

    pub async fn connect(
        &self,
        initial_wait_timeout: u64,
        reconnect_timeout: u64,
        ping_interval: u64,
        ping_failure_threshold: u64,
    ) -> ClientResult<()> {
        debug!("[{}] Connecting to {}", self.id.to_string(), self.url);
        if initial_wait_timeout > 0 {
            debug!(
                "[{}] Delayed connection for {} seconds (backoff timeout for multiple connections)",
                self.id.to_string(),
                initial_wait_timeout
            );
            let mut interval =
                tokio::time::interval(std::time::Duration::from_secs(initial_wait_timeout));

            interval.tick().await;
            interval.tick().await;
        }

        let wss_stream = self.connect_with_reconnections(reconnect_timeout).await?;
        debug!("[{}] Connected to {}", self.id.to_string(), self.url);

        let (mut writer, mut reader) = wss_stream.split();

        let (tx, mut rx): (
            UnboundedSender<SlackTungsteniteWssClientCommand>,
            UnboundedReceiver<SlackTungsteniteWssClientCommand>,
        ) = tokio::sync::mpsc::unbounded_channel();

        {
            let mut self_command_writer = self.command_writer.write().unwrap();
            self_command_writer.replace(tx.clone());
        };

        struct PongState {
            time: SystemTime,
        }
        let last_time_pong_received: Arc<RwLock<PongState>> = Arc::new(RwLock::new(PongState {
            time: SystemTime::now(),
        }));

        {
            let thread_last_time_pong_received = last_time_pong_received.clone();
            let thread_client_id = self.id.clone();

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
                                "[{}] Pong to Slack: {:?}",
                                thread_client_id.to_string(),
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
                                "[{}] Ping to Slack: {:?}",
                                thread_client_id.to_string(),
                                body
                            );

                            let seen_pong_time_in_secs = {
                                let last_pong = thread_last_time_pong_received.read().unwrap();

                                SystemTime::now()
                                    .duration_since(last_pong.time)
                                    .unwrap()
                                    .as_secs()
                            };

                            if seen_pong_time_in_secs > ping_interval * ping_failure_threshold {
                                warn!(
                                    "[{}] Haven't seen any pong from Slack since {} seconds ago",
                                    thread_client_id.to_string(),
                                    seen_pong_time_in_secs
                                );
                                rx.close()
                            } else if let Err(err) = writer
                                .send(tokio_tungstenite::tungstenite::Message::Ping(body.to_vec()))
                                .await
                            {
                                warn!(
                                    "[{}] Ping slack failed with: {:?}",
                                    thread_client_id.to_string(),
                                    err
                                );
                                rx.close()
                            }
                        }
                        SlackTungsteniteWssClientCommand::Exit => {
                            writer.close().await.unwrap_or(());
                            rx.close();
                        }
                    }
                }
            });
        }

        {
            let thread_listener = self.client_listener.clone();
            let thread_client_id = self.id.clone();
            let ping_tx = tx.clone();
            tokio::spawn(async move {
                let mut interval =
                    tokio::time::interval(std::time::Duration::from_secs(ping_interval));

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
                        thread_listener.on_disconnect(&thread_client_id).await;
                        break;
                    }
                }
            });
        }

        {
            let thread_listener = self.client_listener.clone();
            let thread_client_id = self.id.clone();
            let thread_last_time_pong_received = last_time_pong_received;

            tokio::spawn(async move {
                while let Some(message) = reader.next().await {
                    match message {
                        Ok(tokio_tungstenite::tungstenite::Message::Text(body)) => {
                            trace!(
                                "[{}] Received from Slack: {:?}",
                                thread_client_id.to_string(),
                                body
                            );
                            if let Some(reply) =
                                thread_listener.on_message(&thread_client_id, body).await
                            {
                                trace!(
                                    "[{}] Sending reply to Slack: {:?}",
                                    thread_client_id.to_string(),
                                    reply
                                );
                                tx.send(SlackTungsteniteWssClientCommand::Message(reply))
                                    .unwrap_or(());
                            }
                        }
                        Ok(tokio_tungstenite::tungstenite::Message::Ping(body)) => {
                            trace!(
                                "[{}] Ping from Slack: {:?}",
                                thread_client_id.to_string(),
                                body
                            );
                            tx.send(SlackTungsteniteWssClientCommand::Pong(body))
                                .unwrap_or(());
                        }
                        Ok(tokio_tungstenite::tungstenite::Message::Pong(body)) => {
                            trace!(
                                "[{}] Pong from Slack: {:?}",
                                thread_client_id.to_string(),
                                body
                            );
                            let mut last_pong = thread_last_time_pong_received.write().unwrap();
                            last_pong.time = SystemTime::now();
                        }
                        Ok(tokio_tungstenite::tungstenite::Message::Binary(body)) => {
                            warn!(
                                "[{}] Unexpected binary received from Slack: {:?}",
                                thread_client_id.to_string(),
                                body
                            );
                            thread_listener
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
                                "[{}] Shutting down WSS channel: {:?}",
                                thread_client_id.to_string(),
                                body
                            );
                            thread_listener.on_disconnect(&thread_client_id).await
                        }
                        Err(err) => {
                            error!(
                                "[{}] Slack WSS error: {:?}",
                                thread_client_id.to_string(),
                                err
                            );
                            thread_listener
                                .on_error(Box::new(SlackClientError::SocketModeProtocolError(
                                    SlackClientSocketModeProtocolError::new(format!(
                                        "Unexpected binary received from Slack: {:?}",
                                        err
                                    )),
                                )))
                                .await;
                            thread_listener.on_disconnect(&thread_client_id).await
                        }
                    }
                }
            });
        }

        Ok(())
    }

    async fn shutdown_channel(&mut self) {
        let maybe_sender = {
            let mut commands_writer = self.command_writer.write().unwrap().clone();
            commands_writer.take()
        };

        if let Some(sender) = maybe_sender {
            sender
                .send(SlackTungsteniteWssClientCommand::Exit)
                .unwrap_or(());
        }

        self.destroyed.store(true, Ordering::Relaxed);
    }
}

#[async_trait]
impl SlackSocketModeWssClient for SlackTungsteniteWssClient {
    async fn message(&self, message_body: String) -> ClientResult<()> {
        let maybe_sender = {
            let commands_writer = self.command_writer.read().unwrap().clone();
            commands_writer
        };

        if let Some(sender) = maybe_sender {
            if !sender.is_closed() {
                tokio::spawn(async move {
                    sender.send(SlackTungsteniteWssClientCommand::Message(message_body))
                });

                Ok(())
            } else {
                Err(SlackClientError::EndOfStream(
                    SlackClientEndOfStreamError::new(),
                ))
            }
        } else {
            Err(SlackClientError::EndOfStream(
                SlackClientEndOfStreamError::new(),
            ))
        }
    }

    async fn start(
        &self,
        initial_wait_timeout: u64,
        reconnect_timeout: u64,
        ping_interval: u64,
        ping_failure_threshold: u64,
    ) {
        self.connect(
            initial_wait_timeout,
            reconnect_timeout,
            ping_interval,
            ping_failure_threshold,
        )
        .await
        .unwrap_or(());
    }

    async fn destroy(&mut self) {
        debug!("[{}] Destroying {}", self.id.to_string(), self.url);
        self.shutdown_channel().await;
    }
}

impl<H: Send + Sync + Clone + Connect> SlackSocketModeWssClientsFactory<SlackTungsteniteWssClient>
    for SlackClientHyperConnector<H>
{
    fn create_wss_client(
        &self,
        wss_url: &SlackWebSocketsUrl,
        client_id: SlackSocketModeWssClientId,
        token: SlackApiToken,
        client_listener: Arc<dyn SlackSocketModeWssClientListener + Sync + Send + 'static>,
    ) -> ClientResult<SlackTungsteniteWssClient> {
        Ok(SlackTungsteniteWssClient::new(
            wss_url,
            client_id,
            token,
            client_listener,
        ))
    }
}
