use rsb_derive::Builder;

use futures::future::{BoxFuture, FutureExt, TryFutureExt};
use hyper::body::*;
use hyper::{Method, Request, Response, StatusCode};
use std::future::Future;
use std::sync::Arc;
use slack_morphism_models::events::SlackPushEvent;
use crate::SlackClientHttpApi;
use crate::listener::signature_verifier::SlackEventSignatureVerifier;


#[derive(Debug, PartialEq, Clone, Builder)]
pub struct SlackPushEventsListenerConfig {
    pub events_signing_secret: String,
    #[default = "SlackPushEventsListenerConfig::DEFAULT_EVENTS_URL_VALUE.into()"]
    pub events_url: String,
}

impl SlackPushEventsListenerConfig {
    const DEFAULT_EVENTS_URL_VALUE: &'static str = "/events";
}

pub fn create_slack_push_events_service_fn<'a, D, F, I, IF>(
    config: Arc<SlackPushEventsListenerConfig>,
    push_service_fn: I,
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
    I: Fn(Result<SlackPushEvent,Box<dyn std::error::Error + Send + Sync + 'static>>) -> IF + 'static + Send + Sync + Clone,
    IF: Future<Output = ()> + 'static + Send,
{
    let signature_verifier : Arc<SlackEventSignatureVerifier> = Arc::new(
        SlackEventSignatureVerifier::new(
            &config.events_signing_secret
        )
    );

    move |req: Request<Body>, chain: D| {
        let cfg = config.clone();
        let c = chain.clone();
        let push_serv = push_service_fn.clone();
        let sign_verifier = signature_verifier.clone();
        async move {
            match (req.method(), req.uri().path()) {
                (&Method::GET, url) if url == cfg.events_url => {
                    let headers = &req.headers().clone();
                    let req_body = req.into_body();
                    match (
                        headers.get(SlackEventSignatureVerifier::SLACK_SIGNED_HASH_HEADER),
                        headers.get(SlackEventSignatureVerifier::SLACK_SIGNED_TIMESTAMP)
                    ) {
                        (Some(received_hash),Some(received_ts)) => {
                            SlackClientHttpApi::http_body_to_string(req_body)
                                .and_then(|body| {
                                    async {
                                        sign_verifier.verify(
                                            &received_hash.to_str().unwrap(),
                                            &body,
                                            &received_ts.to_str().unwrap()
                                        )
                                            .map(|_| body)
                                            .map_err(|e| e.into())
                                    }
                                })
                                .map_ok(|body| {
                                    serde_json::from_str::<SlackPushEvent>(body.as_str()).map_err(|e| e.into())
                                })
                                .and_then( |event|
                                    push_serv(event).map(|_| Ok(()))
                                ).
                                await?;
                            Ok(Response::new(Body::empty()))
                        }
                        _ => {
                            Response::builder().status(StatusCode::FORBIDDEN).body(Body::empty()).map_err(|e| e.into())
                        }
                    }

                }
                _ => c(req).await,
            }
        }
        .boxed()
    }
}
