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
use std::sync::Arc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio_tungstenite::*;
use url::Url;

#[derive(Clone)]
pub struct SlackTungsteniteWssClient {
    id: SlackSocketModeWssClientId,
    token: SlackApiToken,
    client_listener: Arc<dyn SlackSocketModeWssClientListener + Sync + Send>,
    url: Url,
    command_writer: Option<UnboundedSender<SlackTungsteniteWssClientCommand>>,
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
            command_writer: None,
        }
    }

    pub async fn connect(&mut self) -> ClientResult<()> {
        debug!("Connecting to {}. Id: {:?}", self.url, self.id);
        let (wss_stream, response) = connect_async(&self.url).await?;
        if !response.status().is_success() && !response.status().is_informational() {
            return Err(
                Box::new(SlackClientError::HttpError(SlackClientHttpError::new(
                    response.status(),
                )))
                .into(),
            );
        } else {
            debug!("Connected to {}. Id: {:?}", self.url, self.id);

            let (mut writer, mut reader) = wss_stream.split();

            let (tx, mut rx): (
                UnboundedSender<SlackTungsteniteWssClientCommand>,
                UnboundedReceiver<SlackTungsteniteWssClientCommand>,
            ) = tokio::sync::mpsc::unbounded_channel();

            self.command_writer = Some(tx.clone());

            {
                tokio::spawn(async move {
                    while let Some(message) = rx.recv().await {
                        match message {
                            SlackTungsteniteWssClientCommand::Message(body) => {
                                writer
                                    .send(tokio_tungstenite::tungstenite::Message::Text(body))
                                    .await
                                    .unwrap();
                            }
                            SlackTungsteniteWssClientCommand::Pong(body) => {
                                debug!("Pong to Slack: {:?}", body);
                                writer
                                    .send(tokio_tungstenite::tungstenite::Message::Pong(body))
                                    .await
                                    .unwrap();
                            }
                            SlackTungsteniteWssClientCommand::Ping => {
                                let body: [u8; 5] = rand::random();
                                debug!("Ping to Slack: {:?}", body);

                                writer
                                    .send(tokio_tungstenite::tungstenite::Message::Ping(
                                        body.to_vec(),
                                    ))
                                    .await
                                    .unwrap();
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
                let ping_tx = tx.clone();
                tokio::spawn(async move {
                    let mut interval = tokio::time::interval(std::time::Duration::from_secs(10));

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
                            break;
                        }
                    }
                });
            }

            let thread_listener = self.client_listener.clone();
            let thread_client_id = self.id.clone();

            tokio::spawn(async move {
                while let Some(message) = reader.next().await {
                    match message {
                        Ok(tokio_tungstenite::tungstenite::Message::Text(body)) => {
                            thread_listener.on_message(&thread_client_id, body).await
                        }
                        Ok(tokio_tungstenite::tungstenite::Message::Ping(body)) => {
                            debug!("Ping from Slack: {:?}", body);
                            tx.send(SlackTungsteniteWssClientCommand::Pong(body))
                                .unwrap_or(());
                        }
                        Ok(tokio_tungstenite::tungstenite::Message::Pong(body)) => {
                            debug!("Pong from Slack: {:?}", body);
                        }
                        Ok(tokio_tungstenite::tungstenite::Message::Binary(body)) => {
                            warn!("Unexpected binary received from Slack: {:?}", body);
                        }
                        Ok(tokio_tungstenite::tungstenite::Message::Close(body)) => {
                            debug!("Shutting down WSS channel: {:?}", body);
                            thread_listener.on_disconnect(&thread_client_id).await
                        }
                        Err(err) => {
                            error!("Slack WSS error: {:?}", err);
                            thread_listener.on_disconnect(&thread_client_id).await
                        }
                    }
                }
            });
        }
        Ok(())
    }

    async fn shutdown_channel(&mut self) {
        if let Some(sender) = self.command_writer.clone() {
            sender
                .send(SlackTungsteniteWssClientCommand::Exit)
                .unwrap_or(());
            self.command_writer = None;
        }
    }
}

#[async_trait]
impl SlackSocketModeWssClient for SlackTungsteniteWssClient {
    fn id(&self) -> &SlackSocketModeWssClientId {
        &self.id
    }

    fn token(&self) -> &SlackApiToken {
        &self.token
    }

    fn listener(&self) -> Arc<dyn SlackSocketModeWssClientListener + Sync + Send> {
        self.client_listener.clone()
    }

    async fn message(&mut self, message_body: String) -> ClientResult<()> {
        if let Some(sender) = self.command_writer.clone() {
            if !sender.is_closed() {
                tokio::spawn(async move {
                    sender.send(SlackTungsteniteWssClientCommand::Message(message_body))
                });

                Ok(())
            } else {
                self.destroy().await;
                Err(Box::new(SlackClientError::EndOfStream(
                    SlackClientEndOfStreamError::new(),
                )))
            }
        } else {
            Err(Box::new(SlackClientError::EndOfStream(
                SlackClientEndOfStreamError::new(),
            )))
        }
    }

    async fn destroy(&mut self) {
        debug!("Destroying {}. Id: {:?}", self.url, self.id);
        self.shutdown_channel().await;
    }
}

#[async_trait]
impl SlackSocketModeWssClientsFactory<SlackTungsteniteWssClient> for SlackClientHyperConnector {
    async fn create_wss_client<'a>(
        &'a self,
        wss_url: &'a SlackWebSocketsUrl,
        client_id: SlackSocketModeWssClientId,
        token: SlackApiToken,
        client_listener: Arc<dyn SlackSocketModeWssClientListener + Sync + Send + 'static>,
    ) -> ClientResult<SlackTungsteniteWssClient> {
        let mut client = SlackTungsteniteWssClient::new(wss_url, client_id, token, client_listener);
        client.connect().await?;
        Ok(client)
    }
}
