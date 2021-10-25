use async_trait::async_trait;
use std::sync::Arc;

use rvstruct::*;
use serde::{Deserialize, Serialize};
use slack_morphism_models::*;

use crate::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackSocketModeWssClientId(pub u8);

#[async_trait]
pub trait SlackSocketModeWssClientsFactory<SCWSS>
where
    SCWSS: SlackSocketModeWssClient + Send + Sync,
{
    async fn create_wss_client<'a>(
        &'a self,
        wss_url: &'a SlackWebSocketsUrl,
        client_id: SlackSocketModeWssClientId,
        token: SlackApiToken,
        client_listener: Arc<dyn SlackSocketModeWssClientListener + Sync + Send + 'static>,
    ) -> ClientResult<SCWSS>;
}

#[async_trait]
pub trait SlackSocketModeWssClient {
    fn id(&self) -> &SlackSocketModeWssClientId;
    fn token(&self) -> &SlackApiToken;
    fn listener(&self) -> Arc<dyn SlackSocketModeWssClientListener + Sync + Send>;

    async fn message(&mut self, message_body: String) -> ClientResult<()>;

    async fn destroy(&mut self);
}

#[async_trait]
pub trait SlackSocketModeWssClientListener {
    async fn on_message(&self, client_id: &SlackSocketModeWssClientId, message_body: String);
    async fn on_disconnect(&self, client_id: &SlackSocketModeWssClientId);
}
