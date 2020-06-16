use rsb_derive::Builder;

use futures::future::{BoxFuture, FutureExt};
use hyper::body::*;
use hyper::{Method, Request, Response};
use std::future::Future;
use std::sync::Arc;

#[derive(Debug, PartialEq, Clone, Builder)]
pub struct SlackPushEventsListenerConfig {
    pub events_signing_secret: String,
    #[default = "SlackPushEventsListenerConfig::DEFAULT_EVENTS_URL_VALUE.into()"]
    pub events_url: String,
}

impl SlackPushEventsListenerConfig {
    const DEFAULT_EVENTS_URL_VALUE: &'static str = "/events";
}

pub fn create_slack_push_events_service_fn<'a, D, F>(
    config: Arc<SlackPushEventsListenerConfig>,
) -> impl Fn(
    Request<Body>,
    D,
) -> BoxFuture<'a, Result<Response<Body>, Box<dyn std::error::Error + Send + Sync + 'a>>>
       + 'a
       + Send
       + Clone
where
    D: Fn(Request<Body>) -> F + 'a + Send + Sync + Clone,
    F: Future<Output = Result<Response<Body>, Box<dyn std::error::Error + Send + Sync + 'a>>>
        + 'a
        + Send,
{
    move |req: Request<Body>, chain: D| {
        let cfg = config.clone();
        let c = chain.clone();
        async move {
            match (req.method(), req.uri().path()) {
                (&Method::GET, url) if url == cfg.events_url => {
                    Ok(Response::new("Events url".into()))
                }
                _ => c(req).await,
            }
        }
        .boxed()
    }
}
