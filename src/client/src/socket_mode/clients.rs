use async_trait::async_trait;
use std::sync::Arc;

use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use slack_morphism_models::*;

use crate::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, Builder)]
pub struct SlackSocketModeWssClientId {
    pub initial_index: u32,
    pub reconnected: u64,
}

impl SlackSocketModeWssClientId {
    pub fn new_reconnected_id(&self) -> Self {
        if self.reconnected < 64 {
            Self {
                reconnected: self.reconnected + 1,
                ..self.clone()
            }
        } else {
            Self {
                reconnected: 0,
                ..self.clone()
            }
        }
    }
}

impl ToString for SlackSocketModeWssClientId {
    fn to_string(&self) -> String {
        format!("{}/{}", self.initial_index, self.reconnected)
    }
}

pub trait SlackSocketModeWssClientsFactory<SCWSS>
where
    SCWSS: SlackSocketModeWssClient + Send + Sync,
{
    fn create_wss_client<'a>(
        &'a self,
        wss_url: &'a SlackWebSocketsUrl,
        client_id: SlackSocketModeWssClientId,
        token: SlackApiToken,
        client_listener: Arc<dyn SlackSocketModeWssClientListener + Sync + Send + 'static>,
    ) -> ClientResult<SCWSS>;
}

#[async_trait]
pub trait SlackSocketModeWssClient {
    async fn message(&self, message_body: String) -> ClientResult<()>;

    async fn start(&self, initial_wait_timeout: u64, reconnect_timeout: u64);
    async fn destroy(&mut self);
}

#[async_trait]
pub trait SlackSocketModeWssClientListener {
    async fn on_message(
        &self,
        client_id: &SlackSocketModeWssClientId,
        message_body: String,
    ) -> Option<String>;

    async fn on_error(&self, error: Box<dyn std::error::Error + Send + Sync>);

    async fn on_disconnect(&self, client_id: &SlackSocketModeWssClientId);
}
