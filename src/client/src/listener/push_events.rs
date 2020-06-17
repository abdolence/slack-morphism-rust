use rsb_derive::Builder;

use crate::listener::signature_verifier::SlackEventSignatureVerifier;
use crate::SlackClientHttpApi;
use futures::future::{BoxFuture, FutureExt, TryFutureExt};
use hyper::body::*;
use hyper::{Method, Request, Response, StatusCode};
use log::*;
use slack_morphism_models::events::*;
use std::future::Future;
use std::sync::Arc;

#[derive(Debug, PartialEq, Clone, Builder)]
pub struct SlackPushEventsListenerConfig {
    pub events_signing_secret: String,
    #[default = "SlackPushEventsListenerConfig::DEFAULT_EVENTS_URL_VALUE.into()"]
    pub events_path: String,
}

impl SlackPushEventsListenerConfig {
    const DEFAULT_EVENTS_URL_VALUE: &'static str = "/push";
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
    I: Fn(Result<SlackPushEvent, Box<dyn std::error::Error + Send + Sync + 'static>>) -> IF
        + 'static
        + Send
        + Sync
        + Clone,
    IF: Future<Output = ()> + 'static + Send,
{
    let signature_verifier: Arc<SlackEventSignatureVerifier> = Arc::new(
        SlackEventSignatureVerifier::new(&config.events_signing_secret),
    );

    move |req: Request<Body>, chain: D| {
        let cfg = config.clone();
        let c = chain.clone();
        let push_serv = push_service_fn.clone();
        let sign_verifier = signature_verifier.clone();
        async move {
            match (req.method(), req.uri().path()) {
                (&Method::POST, url) if url == cfg.events_path => {
                    let headers = &req.headers().clone();
                    let req_body = req.into_body();
                    match (
                        headers.get(SlackEventSignatureVerifier::SLACK_SIGNED_HASH_HEADER),
                        headers.get(SlackEventSignatureVerifier::SLACK_SIGNED_TIMESTAMP),
                    ) {
                        (Some(received_hash), Some(received_ts)) => {
                            SlackClientHttpApi::http_body_to_string(req_body)
                                .and_then(|body| async {
                                    sign_verifier
                                        .verify(
                                            &received_hash.to_str().unwrap(),
                                            &body,
                                            &received_ts.to_str().unwrap(),
                                        )
                                        .map(|_| body)
                                        .map_err(|e| e.into())
                                })
                                .map_ok(|body| {
                                    serde_json::from_str::<SlackPushEvent>(body.as_str())
                                        .map_err(|e| e.into())
                                })
                                .and_then(|event|
                                    async move {
                                        match event {
                                            Ok(SlackPushEvent::UrlVerification(url_ver)) => {
                                                debug!("Received Slack URL push verification challenge: {}", url_ver.challenge);
                                                Response::builder()
                                                    .status(StatusCode::OK)
                                                    .body(url_ver.challenge.into())
                                                    .map_err(|e| e.into())
                                            }
                                            other => {
                                                push_serv(other).map(|_| Ok(Response::new(Body::empty()))).await
                                            }
                                        }
                                    }
                                )
                                .await
                        }
                        _ => Response::builder()
                            .status(StatusCode::FORBIDDEN)
                            .body(Body::empty())
                            .map_err(|e| e.into()),
                    }
                }
                _ => c(req).await,
            }
        }
        .boxed()
    }
}
