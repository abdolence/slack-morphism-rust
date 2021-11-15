use crate::connector::SlackClientHyperConnector;

use std::future::Future;

use futures::future::{BoxFuture, FutureExt};
use hyper::{Body, Request, Response};
use hyper::client::connect::Connect;

pub use command_events::*;
pub use interaction_events::*;
pub use oauth::*;
pub use push_events::*;
use slack_morphism::listener::SlackClientEventsListenerEnvironment;
pub use slack_morphism::signature_verifier::*;
use std::sync::Arc;

mod command_events;
mod interaction_events;
mod oauth;
mod push_events;

pub struct SlackClientEventsHyperListener<H: 'static + Send + Sync + Connect + Clone> {
    pub environment: Arc<SlackClientEventsListenerEnvironment<SlackClientHyperConnector<H>>>,
}

impl<H: 'static + Send + Sync + Connect + Clone> SlackClientEventsHyperListener<H> {
    pub fn new(
        environment: Arc<SlackClientEventsListenerEnvironment<SlackClientHyperConnector<H>>>,
    ) -> Self {
        Self { environment }
    }
}

pub fn chain_service_routes_fn<'a, R, D, FR, FD>(
    route: R,
    default: D,
) -> impl Fn(
    Request<Body>,
) -> BoxFuture<'a, Result<Response<Body>, Box<dyn std::error::Error + Send + Sync + 'a>>>
       + 'a
       + Send
       + Clone
where
    R: Fn(Request<Body>, D) -> FR + 'a + Clone + Send,
    D: Fn(Request<Body>) -> FD + 'a + Clone + Send,
    FR: Future<Output = Result<Response<Body>, Box<dyn std::error::Error + Send + Sync + 'a>>>
        + 'a
        + Send,
    FD: Future<Output = Result<Response<Body>, Box<dyn std::error::Error + Send + Sync + 'a>>>
        + 'a
        + Send,
{
    move |req: Request<Body>| route(req, default.clone()).boxed()
}
