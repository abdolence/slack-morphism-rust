use rsb_derive::Builder;

use crate::errors::*;
use crate::listener::signature_verifier::SlackEventSignatureVerifier;
use crate::listener::SlackClientEventsListener;
use crate::{SlackClient, SlackClientHttpApi};
use futures::future::{BoxFuture, FutureExt, TryFutureExt};
use hyper::body::*;
use hyper::{Method, Request, Response, StatusCode};
pub use slack_morphism_models::events::*;
use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;

#[derive(Debug, PartialEq, Clone, Builder)]
pub struct SlackInteractionEventsListenerConfig {
    pub events_signing_secret: String,
    #[default = "SlackInteractionEventsListenerConfig::DEFAULT_EVENTS_URL_VALUE.into()"]
    pub events_path: String,
}

impl SlackInteractionEventsListenerConfig {
    const DEFAULT_EVENTS_URL_VALUE: &'static str = "/interaction";
}

impl SlackClientEventsListener {
    pub fn interaction_events_service_fn<'a, D, F, I, IF>(
        &self,
        config: Arc<SlackInteractionEventsListenerConfig>,
        interaction_service_fn: I,
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
        I: Fn(
                Result<SlackInteractionEvent, Box<dyn std::error::Error + Send + Sync + 'static>>,
                Arc<SlackClient>,
            ) -> IF
            + 'static
            + Send
            + Sync
            + Clone,
        IF: Future<Output = ()> + 'static + Send,
    {
        let signature_verifier: Arc<SlackEventSignatureVerifier> = Arc::new(
            SlackEventSignatureVerifier::new(&config.events_signing_secret),
        );
        let client = self.client.clone();

        move |req: Request<Body>, chain: D| {
            let cfg = config.clone();
            let c = chain.clone();
            let serv = interaction_service_fn.clone();
            let sign_verifier = signature_verifier.clone();
            let sc = client.clone();
            async move {
                match (req.method(), req.uri().path()) {
                    (&Method::POST, url) if url == cfg.events_path => {
                        SlackClientHttpApi::decode_signed_response(req, &sign_verifier)
                            .map_ok(|body| {
                                let body_params: HashMap<String, String> =
                                    url::form_urlencoded::parse(body.as_bytes())
                                        .into_owned()
                                        .collect();

                                let payload = body_params
                                    .get("payload")
                                    .ok_or(SlackClientError::SystemError(
                                        SlackClientSystemError::new(
                                            "Absent payload in the request from Slack".into(),
                                        ),
                                    ))
                                    .map_err(|e| e.into());

                                payload.and_then(|payload_value| {
                                    serde_json::from_str::<SlackInteractionEvent>(payload_value)
                                        .map_err(|e| e.into())
                                })
                            })
                            .and_then(|event| async move {
                                match event {
                                    Ok(SlackInteractionEvent::ViewSubmission(_)) => {
                                        serv(event, sc).await;
                                        Response::builder()
                                            .status(StatusCode::OK)
                                            .body("".into())
                                            .map_err(|e| e.into())
                                    }
                                    Ok(_) => {
                                        serv(event, sc).await;
                                        Ok(Response::new(Body::empty()))
                                    }
                                    Err(_) => {
                                        serv(event, sc).await;
                                        Response::builder()
                                            .status(StatusCode::FORBIDDEN)
                                            .body(Body::empty())
                                            .map_err(|e| e.into())
                                    }
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
