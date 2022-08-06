use crate::hyper_tokio::SlackClientHyperConnector;
use crate::listener::SlackClientEventsListenerEnvironment;
use hyper::client::connect::Connect;
use std::sync::Arc;

mod slack_events_middleware;
pub use slack_events_middleware::SlackEventsApiMiddleware;

pub struct SlackEventsAxumListener<H: 'static + Send + Sync + Connect + Clone> {
    pub environment: Arc<SlackClientEventsListenerEnvironment<SlackClientHyperConnector<H>>>,
}

impl<H: 'static + Send + Sync + Connect + Clone> SlackEventsAxumListener<H> {
    pub fn new(
        environment: Arc<SlackClientEventsListenerEnvironment<SlackClientHyperConnector<H>>>,
    ) -> Self {
        Self { environment }
    }
}

mod slack_oauth_routes;
pub use slack_oauth_routes::*;

mod slack_event_extractors;
pub use slack_event_extractors::SlackEventExtractors;
