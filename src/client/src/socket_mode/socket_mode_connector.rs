use std::sync::{Arc, RwLock};

use async_trait::async_trait;

use slack_morphism_models::*;

use crate::*;

#[async_trait]
pub trait SlackClientSocketModeConnector<SCWSS>
where
    SCWSS: SlackSocketModeWssClient + Send + Sync,
{
    async fn create_wss_client(&self, wss_url: &SlackWebSocketsUrl) -> ClientResult<SCWSS>;
}

#[async_trait]
pub trait SlackSocketModeWssClient {
    async fn message<RQ>(&mut self, message_body: &RQ) -> ClientResult<()>
    where
        RQ: serde::ser::Serialize + Send + Sync;

    async fn destroy(&mut self);
}

pub type SlackClientSocketModeClientsStorage<SCWSS> =
    Arc<RwLock<SlackClientSocketModeClients<SCWSS>>>;

pub struct SlackClientSocketModeClients<SCWSS>
where
    SCWSS: SlackSocketModeWssClient + Send + Sync,
{
    active_clients: Vec<SCWSS>,
}

impl<SCWSS> SlackClientSocketModeClients<SCWSS>
where
    SCWSS: SlackSocketModeWssClient + Send + Sync,
{
    pub fn new() -> Self {
        Self {
            active_clients: vec![],
        }
    }

    pub fn add_new_client(&mut self, wss_client: SCWSS) {
        self.active_clients.push(wss_client);
    }

    pub fn all_active_clients(&self) -> &Vec<SCWSS> {
        &self.active_clients
    }

    pub fn clear(&mut self) -> Vec<SCWSS> {
        let existing_vec = self.active_clients.drain(..).collect();
        self.active_clients = vec![];
        existing_vec
    }
}
