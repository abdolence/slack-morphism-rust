use crate::socket_mode::tokio_clients_manager::SlackSocketModeTungsteniteClientsManager;
use crate::socket_mode::tungstenite_wss_client::SlackTungsteniteWssClient;
use crate::SlackClientHyperConnector;
use hyper::client::connect::Connect;
use slack_morphism::clients::{
    SlackSocketModeClientsManagerFactory, SlackSocketModeClientsManagerT,
    SlackSocketModeWssClientListener, SlackSocketModeWssClientsFactory,
};
use slack_morphism::listener::SlackClientEventsListenerEnvironment;
use slack_morphism::{ClientResult, SlackSocketModeWssClientId};
use slack_morphism_models::SlackWebSocketsUrl;
use std::sync::Arc;

mod tokio_clients_manager;
mod tungstenite_wss_client;

impl<H: Send + Sync + Clone + Connect> SlackSocketModeWssClientsFactory<SlackTungsteniteWssClient>
    for SlackClientHyperConnector<H>
{
    fn create_wss_client(
        &self,
        _wss_url: &SlackWebSocketsUrl,
        _client_id: SlackSocketModeWssClientId,
        _client_listener: Arc<dyn SlackSocketModeWssClientListener + Sync + Send + 'static>,
    ) -> ClientResult<SlackTungsteniteWssClient> {
        todo!()
    }
}

impl<H: Send + Sync + Clone + Connect + 'static>
    SlackSocketModeClientsManagerFactory<SlackClientHyperConnector<H>>
    for SlackClientHyperConnector<H>
{
    fn new_clients_manager(
        &self,
        listener_environment: Arc<
            SlackClientEventsListenerEnvironment<SlackClientHyperConnector<H>>,
        >,
    ) -> Arc<dyn SlackSocketModeClientsManagerT + Send + Sync> {
        Arc::new(SlackSocketModeTungsteniteClientsManager::new(
            listener_environment,
        ))
    }
}
