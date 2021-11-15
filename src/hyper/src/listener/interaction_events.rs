use crate::listener::SlackClientEventsHyperListener;

pub use slack_morphism_models::events::*;

use crate::connector::SlackClientHyperConnector;

use slack_morphism::errors::*;
use slack_morphism::listener::*;
use slack_morphism::signature_verifier::SlackEventSignatureVerifier;

use futures::future::{BoxFuture, FutureExt, TryFutureExt};
use hyper::body::*;
use hyper::client::connect::Connect;
use hyper::{Method, Request, Response, StatusCode};
use slack_morphism::UserCallbackResult;
use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;

impl<H: 'static + Send + Sync + Connect + Clone> SlackClientEventsHyperListener<H> {
    pub fn interaction_events_service_fn<'a, D, F>(
        &self,
        config: Arc<SlackInteractionEventsListenerConfig>,
        interaction_service_fn: UserCallbackFunction<
            SlackInteractionEvent,
            impl Future<Output = UserCallbackResult<()>> + 'static + Send,
            SlackClientHyperConnector<H>,
        >,
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
    {
        let signature_verifier: Arc<SlackEventSignatureVerifier> = Arc::new(
            SlackEventSignatureVerifier::new(&config.events_signing_secret),
        );
        let client = self.environment.client.clone();
        let error_handler = self.environment.error_handler.clone();
        let user_state_storage = self.environment.user_state.clone();

        move |req: Request<Body>, chain: D| {
            let cfg = config.clone();
            let sign_verifier = signature_verifier.clone();
            let sc = client.clone();
            let thread_error_handler = error_handler.clone();
            let thread_user_state_storage = user_state_storage.clone();

            async move {
                match (req.method(), req.uri().path()) {
                    (&Method::POST, url) if url == cfg.events_path => {
                        SlackClientHyperConnector::<H>::decode_signed_response(req, &sign_verifier)
                            .map_ok(|body| {
                                let body_params: HashMap<String, String> =
                                    url::form_urlencoded::parse(body.as_bytes())
                                        .into_owned()
                                        .collect();

                                let payload = body_params
                                    .get("payload")
                                    .ok_or_else( || SlackClientError::SystemError(
                                        SlackClientSystemError::new().with_message(
                                            "Absent payload in the request from Slack".into(),
                                        ),
                                    ))
                                    .map_err(|e| e.into());

                                payload.and_then(|payload_value| {
                                    serde_json::from_str::<SlackInteractionEvent>(payload_value)
                                        .map_err(|e| SlackClientProtocolError::new(e).with_json_body(payload_value.clone()).into())
                                })
                            })
                            .and_then(|event| async move {
                                match event {
                                    Ok(view_submission_event@SlackInteractionEvent::ViewSubmission(_)) => {
                                        match interaction_service_fn(view_submission_event, sc.clone(), thread_user_state_storage.clone()).await {
                                            Ok(_) => {
                                                Response::builder()
                                                    .status(StatusCode::OK)
                                                    .body("".into())
                                                    .map_err(|e| e.into())
                                            }
                                            Err(err) => {
                                                let status_code = thread_error_handler(err, sc, thread_user_state_storage);
                                                Response::builder()
                                                    .status(status_code)
                                                    .body(Body::empty())
                                                    .map_err(|e| e.into())
                                            }
                                        }

                                    }
                                    Ok(interaction_event) => {
                                        match interaction_service_fn(interaction_event, sc.clone(), thread_user_state_storage.clone()).await {
                                            Ok(_) => Response::builder()
                                                .status(StatusCode::OK)
                                                .body(Body::empty())
                                                .map_err(|e| e.into()),
                                            Err(err) => {
                                                let status_code = thread_error_handler(err, sc, thread_user_state_storage);
                                                Response::builder()
                                                    .status(status_code)
                                                    .body(Body::empty())
                                                    .map_err(|e| e.into())
                                            }
                                        }
                                    }
                                    Err(event_err) => {
                                        let status_code = thread_error_handler(event_err, sc, thread_user_state_storage);
                                        Response::builder()
                                            .status(status_code)
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
