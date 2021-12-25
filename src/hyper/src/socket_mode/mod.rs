use crate::socket_mode::tokio_clients_manager::SlackSocketModeTokioClientsManager;
use crate::SlackClientHyperConnector;
use hyper::client::connect::Connect;
use slack_morphism::clients_manager::{
    SlackSocketModeClientsManager, SlackSocketModeClientsManagerFactory,
};
use slack_morphism::listener::SlackClientEventsListenerEnvironment;
use slack_morphism::SlackSocketModeWssClientId;
use std::sync::Arc;

mod tokio_clients_manager;
mod tungstenite_wss_client;

impl<H: Send + Sync + Clone + Connect + 'static>
    SlackSocketModeClientsManagerFactory<SlackClientHyperConnector<H>>
    for SlackClientHyperConnector<H>
{
    fn new_clients_manager(
        &self,
        listener_environment: Arc<
            SlackClientEventsListenerEnvironment<SlackClientHyperConnector<H>>,
        >,
    ) -> Arc<dyn SlackSocketModeClientsManager + Send + Sync> {
        Arc::new(SlackSocketModeTokioClientsManager::new(
            listener_environment,
        ))
    }
}
