use futures::future::{BoxFuture, FutureExt};
use hyper::{Body, Request, Response};
use std::future::Future;

mod command_events;
mod interaction_events;
mod oauth;
mod push_events;
mod signature_verifier;

use crate::SlackClient;
pub use command_events::*;
pub use interaction_events::*;
pub use oauth::*;
pub use push_events::*;
pub use signature_verifier::*;
use std::sync::Arc;

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

pub type ErrorHandler = Box<fn(Box<dyn std::error::Error + Send + Sync + 'static>, Arc<SlackClient>)>;

#[derive(Clone)]
pub struct SlackClientEventsListener {
    client: Arc<SlackClient>,
    error_handler: ErrorHandler,
}

impl SlackClientEventsListener {
    pub fn new(client: Arc<SlackClient>) -> Self {
        Self {
            client,
            error_handler: Box::new(Self::empty_error_handler),
        }
    }

    pub fn with_error_handler(
        self,
        error_handler: fn(Box<dyn std::error::Error + Send + Sync + 'static>, Arc<SlackClient>),
    ) -> Self {
        Self {
            error_handler: Box::new(error_handler),
            ..self
        }
    }

    fn empty_error_handler(
        _err: Box<dyn std::error::Error + Send + Sync>,
        _client: Arc<SlackClient>,
    ) {
    }
}
