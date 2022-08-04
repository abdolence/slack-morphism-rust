use crate::hyper_tokio::SlackClientHyperConnector;
use crate::listener::SlackClientEventsListenerEnvironment;
use axum::response::Response;
use hyper::client::connect::Connect;
use std::sync::Arc;

mod slack_events_middleware;

pub struct SlackEventsAxumListener<H: 'static + Send + Sync + Connect + Clone> {
    pub environment: Arc<SlackClientEventsListenerEnvironment<SlackClientHyperConnector<H>>>,
}

impl<H: 'static + Send + Sync + Connect + Clone> SlackEventsAxumListener<H> {
    pub fn new(
        environment: Arc<SlackClientEventsListenerEnvironment<SlackClientHyperConnector<H>>>,
    ) -> Self {
        Self { environment }
    }

    fn handle_error(
        environment: Arc<SlackClientEventsListenerEnvironment<SlackClientHyperConnector<H>>>,
        result: AnyStdResult<Response<hyper::Body>>,
    ) -> Response<hyper::Body> {
        match result {
            Err(err) => {
                let http_status = (environment.error_handler)(
                    err,
                    environment.client.clone(),
                    environment.user_state.clone(),
                );
                Response::builder()
                    .status(http_status)
                    .body(hyper::Body::empty())
                    .unwrap()
            }
            Ok(result) => result,
        }
    }
}

mod slack_oauth_routes;
use crate::AnyStdResult;
pub use slack_oauth_routes::*;
