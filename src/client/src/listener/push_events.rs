use rsb_derive::Builder;

use crate::listener::signature_verifier::SlackEventSignatureVerifier;
use crate::listener::SlackClientEventsListener;
use crate::{SlackClient, SlackClientHttpApi};
use futures::future::{BoxFuture, FutureExt, TryFutureExt};
use hyper::body::*;
use hyper::{Method, Request, Response, StatusCode};
use log::*;
pub use slack_morphism_models::events::*;
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

impl SlackClientEventsListener {
    pub fn push_events_service_fn<'a, D, F, I, IF>(
        &self,
        config: Arc<SlackPushEventsListenerConfig>,
        push_service_fn: I,
    ) -> impl Fn(
        Request<Body>,
        D,
    ) -> BoxFuture<
        'a,
        Result<Response<Body>, Box<dyn std::error::Error + Send + Sync + 'a>>,
    >
           + 'a
           + Send
           + Clone
    where
        D: Fn(Request<Body>) -> F + 'a + Send + Sync + Clone,
        F: Future<Output = Result<Response<Body>, Box<dyn std::error::Error + Send + Sync + 'a>>>
            + 'a
            + Send,
        I: Fn(SlackPushEvent, Arc<SlackClient>) -> IF + 'static + Send + Sync + Clone,
        IF: Future<Output = ()> + 'static + Send,
    {
        let signature_verifier: Arc<SlackEventSignatureVerifier> = Arc::new(
            SlackEventSignatureVerifier::new(&config.events_signing_secret),
        );
        let client = self.client.clone();
        let error_handler = self.error_handler.clone();

        move |req: Request<Body>, chain: D| {
            let cfg = config.clone();
            let c = chain.clone();
            let push_serv = push_service_fn.clone();
            let sign_verifier = signature_verifier.clone();
            let sc = client.clone();
            let thread_error_handler = error_handler.clone();
            async move {
                match (req.method(), req.uri().path()) {
                    (&Method::POST, url) if url == cfg.events_path => {
                        SlackClientHttpApi::decode_signed_response(req, &sign_verifier)
                            .map_ok(|body| {
                                serde_json::from_str::<SlackPushEvent>(body.as_str())
                                    .map_err(|e| e.into())
                            })
                            .and_then(|event| async move {
                                match event {
                                    Ok(SlackPushEvent::UrlVerification(url_ver)) => {
                                        debug!(
                                            "Received Slack URL push verification challenge: {}",
                                            url_ver.challenge
                                        );
                                        push_serv(SlackPushEvent::UrlVerification(url_ver.clone()), sc).await;
                                        Response::builder()
                                            .status(StatusCode::OK)
                                            .body(url_ver.challenge.into())
                                            .map_err(|e| e.into())
                                    }
                                    other => match other {
                                        Ok(push_event) => {
                                            push_serv(push_event, sc).await;
                                            Ok(Response::new(Body::empty()))
                                        }
                                        Err(err_oush_event) => {
                                            thread_error_handler(err_oush_event, sc);
                                            Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::empty())
                                                .map_err(|e| e.into())
                                        }
                                    },
                                }
                            })
                            .await
                    }
                    _ => c(req).await,
                }
            }
            .boxed()
        }
    }
}
