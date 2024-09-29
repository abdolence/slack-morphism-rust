use crate::errors::*;
use crate::hyper_tokio::connector::SlackClientHyperConnector;
use crate::listener::*;
pub use crate::models::events::*;
use crate::signature_verifier::SlackEventSignatureVerifier;

use crate::blocks::SlackViewSubmissionResponse;
use crate::hyper_tokio::hyper_ext::HyperExtensions;
use crate::hyper_tokio::*;
use futures::future::{BoxFuture, FutureExt, TryFutureExt};
use http_body_util::{BodyExt, Empty, Full};
use hyper::body::Incoming;
use hyper::{Method, Request, Response, StatusCode};
use hyper_util::client::legacy::connect::Connect;
use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;

impl<H: 'static + Send + Sync + Connect + Clone> SlackClientEventsHyperListener<H> {
    pub fn interaction_events_service_fn<'a, D, F, R>(
        &self,
        config: Arc<SlackInteractionEventsListenerConfig>,
        interaction_service_fn: UserCallbackFunction<
            SlackInteractionEvent,
            impl Future<Output = UserCallbackResult<R>> + 'static + Send,
            SlackClientHyperConnector<H>,
        >,
    ) -> impl Fn(Request<Incoming>, D) -> BoxFuture<'a, AnyStdResult<Response<Body>>> + 'a + Send + Clone
    where
        D: Fn(Request<Incoming>) -> F + 'a + Send + Sync + Clone,
        F: Future<Output = AnyStdResult<Response<Body>>> + 'a + Send,
        R: SlackInteractionEventResponse,
    {
        let signature_verifier: Arc<SlackEventSignatureVerifier> = Arc::new(
            SlackEventSignatureVerifier::new(&config.events_signing_secret),
        );
        let client = self.environment.client.clone();
        let error_handler = self.environment.error_handler.clone();
        let user_state_storage = self.environment.user_state.clone();

        move |req: Request<Incoming>, chain: D| {
            let cfg = config.clone();
            let sign_verifier = signature_verifier.clone();
            let sc = client.clone();
            let thread_error_handler = error_handler.clone();
            let thread_user_state_storage = user_state_storage.clone();

            async move {
                match (req.method(), req.uri().path()) {
                    (&Method::POST, url) if url == cfg.events_path => {
                        HyperExtensions::decode_signed_response(req, &sign_verifier)
                            .map_ok(|(body, _)| {
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
                                        match interaction_service_fn(view_submission_event.clone(), sc.clone(), thread_user_state_storage.clone()).await {
                                            Ok(response) => {
                                                response.to_http_response(&view_submission_event)
                                            }
                                            Err(err) => {
                                                let status_code = thread_error_handler(err, sc, thread_user_state_storage);
                                                Response::builder()
                                                    .status(status_code)
                                                    .body(Empty::new().boxed())
                                                    .map_err(|e| e.into())
                                            }
                                        }

                                    }
                                    Ok(block_suggestion_event@SlackInteractionEvent::BlockSuggestion(_)) => {
                                        match interaction_service_fn(block_suggestion_event.clone(), sc.clone(), thread_user_state_storage.clone()).await {
                                            Ok(response) => {
                                                response.to_http_response(&block_suggestion_event)
                                            }
                                            Err(err) => {
                                                let status_code = thread_error_handler(err, sc, thread_user_state_storage);
                                                Response::builder()
                                                    .status(status_code)
                                                    .body(Empty::new().boxed())
                                                    .map_err(|e| e.into())
                                            }
                                        }

                                    }
                                    Ok(interaction_event) => {
                                        match interaction_service_fn(interaction_event.clone(), sc.clone(), thread_user_state_storage.clone()).await {
                                            Ok(response) => response.to_http_response(&interaction_event),
                                            Err(err) => {
                                                let status_code = thread_error_handler(err, sc, thread_user_state_storage);
                                                Response::builder()
                                                    .status(status_code)
                                                    .body(Empty::new().boxed())
                                                    .map_err(|e| e.into())
                                            }
                                        }
                                    }
                                    Err(event_err) => {
                                        let status_code = thread_error_handler(event_err, sc, thread_user_state_storage);
                                        Response::builder()
                                            .status(status_code)
                                            .body(Empty::new().boxed())
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

pub trait SlackInteractionEventResponse {
    fn to_http_response(&self, event: &SlackInteractionEvent) -> AnyStdResult<Response<Body>>;
}

impl SlackInteractionEventResponse for () {
    fn to_http_response(&self, event: &SlackInteractionEvent) -> AnyStdResult<Response<Body>> {
        match event {
            SlackInteractionEvent::ViewSubmission(_) => Response::builder()
                .status(StatusCode::OK)
                .body(Empty::new().boxed())
                .map_err(|e| e.into()),
            _ => Response::builder()
                .status(StatusCode::OK)
                .body(Empty::new().boxed())
                .map_err(|e| e.into()),
        }
    }
}

impl SlackInteractionEventResponse for SlackViewSubmissionResponse {
    fn to_http_response(&self, _event: &SlackInteractionEvent) -> AnyStdResult<Response<Body>> {
        let json_str = serde_json::to_string(&self)?;
        Ok(Response::builder()
            .status(StatusCode::OK)
            .header("content-type", "application/json; charset=utf-8")
            .body(Full::new(json_str.into()).boxed())?)
    }
}

impl SlackInteractionEventResponse for SlackBlockSuggestionResponse {
    fn to_http_response(&self, _event: &SlackInteractionEvent) -> AnyStdResult<Response<Body>> {
        let json_str = serde_json::to_string(&self)?;
        Ok(Response::builder()
            .status(StatusCode::OK)
            .header("content-type", "application/json; charset=utf-8")
            .body(Full::new(json_str.into()).boxed())?)
    }
}
