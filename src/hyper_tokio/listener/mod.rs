use std::future::Future;
use std::sync::Arc;

use futures::future::{BoxFuture, FutureExt};
use hyper::body::Incoming;
use hyper::{Request, Response};
use hyper_util::client::legacy::connect::Connect;

use crate::hyper_tokio::connector::SlackClientHyperConnector;
use crate::hyper_tokio::Body;
use crate::listener::SlackClientEventsListenerEnvironment;
use crate::AnyStdResult;

pub use command_events::*;
pub use interaction_events::*;
pub use oauth::*;
pub use push_events::*;

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
) -> impl Fn(Request<Incoming>) -> BoxFuture<'a, AnyStdResult<Response<Body>>> + 'a + Send + Clone
where
    R: Fn(Request<Incoming>, D) -> FR + 'a + Clone + Send,
    D: Fn(Request<Incoming>) -> FD + 'a + Clone + Send,
    FR: Future<Output = AnyStdResult<Response<Body>>> + 'a + Send,
    FD: Future<Output = AnyStdResult<Response<Body>>> + 'a + Send,
{
    move |req: Request<Incoming>| route(req, default.clone()).boxed()
}
