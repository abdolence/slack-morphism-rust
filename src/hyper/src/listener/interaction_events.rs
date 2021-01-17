use crate::listener::SlackClientEventsHyperListener;

pub use slack_morphism_models::events::*;

use crate::connector::SlackClientHyperConnector;

use slack_morphism::errors::*;
use slack_morphism::listener::*;
use slack_morphism::signature_verifier::SlackEventSignatureVerifier;
use slack_morphism::SlackClient;

use futures::future::{BoxFuture, FutureExt, TryFutureExt};
use hyper::body::*;
use hyper::{Method, Request, Response, StatusCode};
use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;

impl SlackClientEventsHyperListener {
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
        I: Fn(SlackInteractionEvent, Arc<SlackClient<SlackClientHyperConnector>>) -> IF
            + 'static
            + Send
            + Sync
            + Clone,
        IF: Future<Output = ()> + 'static + Send,
    {
        let signature_verifier: Arc<SlackEventSignatureVerifier> = Arc::new(
            SlackEventSignatureVerifier::new(&config.events_signing_secret),
        );
        let client = self.environment.client.clone();
        let error_handler = self.environment.error_handler.clone();

        move |req: Request<Body>, chain: D| {
            let cfg = config.clone();
            let serv = interaction_service_fn.clone();
            let sign_verifier = signature_verifier.clone();
            let sc = client.clone();
            let thread_error_handler = error_handler.clone();

            async move {
                match (req.method(), req.uri().path()) {
                    (&Method::POST, url) if url == cfg.events_path => {
                        SlackClientHyperConnector::decode_signed_response(req, &sign_verifier)
                            .map_ok(|body| {
                                let body_params: HashMap<String, String> =
                                    url::form_urlencoded::parse(body.as_bytes())
                                        .into_owned()
                                        .collect();

                                let payload = body_params
                                    .get("payload")
                                    .ok_or_else( || SlackClientError::SystemError(
                                        SlackClientSystemError::new(
                                            "Absent payload in the request from Slack".into(),
                                        ),
                                    ))
                                    .map_err(|e| e.into());

                                payload.and_then(|payload_value| {
                                    serde_json::from_str::<SlackInteractionEvent>(payload_value)
                                        .map_err(|e| SlackClientProtocolError{ json_error: e, http_response_body: payload_value.clone() }.into())
                                })
                            })
                            .and_then(|event| async move {
                                match event {
                                    Ok(view_submission_event@SlackInteractionEvent::ViewSubmission(_)) => {
                                        serv(view_submission_event, sc).await;
                                        Response::builder()
                                            .status(StatusCode::OK)
                                            .body("".into())
                                            .map_err(|e| e.into())
                                    }
                                    Ok(interaction_event) => {
                                        serv(interaction_event, sc).await;
                                        Ok(Response::new(Body::empty()))
                                    }
                                    Err(event_err) => {
                                        thread_error_handler(event_err, sc);
                                        Response::builder()
                                            .status(StatusCode::FORBIDDEN)
                                            .body(Body::empty())
                                            .map_err(|e| e.into())
                                    }
                                }
                            })
                            .await
                    }
                    _ => chain(req).await,
                }
            }
            .boxed()
        }
    }
}
