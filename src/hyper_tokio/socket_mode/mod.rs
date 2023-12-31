use crate::clients_manager::{SlackSocketModeClientsManager, SlackSocketModeClientsManagerFactory};
use crate::hyper_tokio::connector::SlackClientHyperConnector;
use crate::hyper_tokio::socket_mode::tokio_clients_manager::SlackSocketModeTokioClientsManager;
use crate::listener::SlackClientEventsListenerEnvironment;
use hyper_util::client::legacy::connect::Connect;
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
