use crate::listener::SlackClientEventsHyperListener;

use crate::connector::SlackClientHyperConnector;
use slack_morphism::errors::*;
use slack_morphism::listener::*;
use slack_morphism::signature_verifier::SlackEventSignatureVerifier;
use slack_morphism::SlackClient;

use futures::future::{BoxFuture, FutureExt, TryFutureExt};
use hyper::body::*;
use hyper::{Method, Request, Response, StatusCode};
pub use slack_morphism_models::events::*;
use std::collections::HashMap;
use std::future::Future;
use std::sync::{Arc, RwLock};

impl SlackClientEventsHyperListener {
    pub fn command_events_service_fn<'a, D, F, I, IF>(
        &self,
        config: Arc<SlackCommandEventsListenerConfig>,
        command_service_fn: I,
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
                SlackCommandEvent,
                Arc<SlackClient<SlackClientHyperConnector>>,
                Arc<RwLock<SlackClientEventsUserStateStorage>>,
            ) -> IF
            + 'static
            + Send
            + Sync
            + Clone,
        IF: Future<
                Output = Result<
                    SlackCommandEventResponse,
                    Box<dyn std::error::Error + Send + Sync + 'static>,
                >,
            >
            + 'static
            + Send,
    {
        let signature_verifier: Arc<SlackEventSignatureVerifier> = Arc::new(
            SlackEventSignatureVerifier::new(&config.events_signing_secret),
        );
        let client = self.environment.client.clone();
        let error_handler = self.environment.error_handler.clone();
        let user_state_storage = self.environment.user_state_storage.clone();

        move |req: Request<Body>, chain: D| {
            let cfg = config.clone();
            let serv = command_service_fn.clone();
            let sign_verifier = signature_verifier.clone();
            let sc = client.clone();
            let thread_error_handler = error_handler.clone();
            let thread_user_state_storage = user_state_storage.clone();

            async move {
                match (req.method(), req.uri().path()) {
                    (&Method::POST, url) if url == cfg.events_path => {
                        SlackClientHyperConnector::decode_signed_response(req, &sign_verifier)
                            .map_ok(|body| {
                                let body_params: HashMap<String, String> =
                                    url::form_urlencoded::parse(body.as_bytes())
                                        .into_owned()
                                        .collect();

                                match (
                                    body_params.get("team_id"),
                                    body_params.get("channel_id"),
                                    body_params.get("user_id"),
                                    body_params.get("command"),
                                    body_params.get("text"),
                                    body_params.get("response_url"),
                                    body_params.get("trigger_id"),
                                ) {
                                    (
                                        Some(team_id),
                                        Some(channel_id),
                                        Some(user_id),
                                        Some(command),
                                        text,
                                        Some(response_url),
                                        Some(trigger_id),
                                    ) => Ok(SlackCommandEvent::new(
                                        team_id.into(),
                                        channel_id.into(),
                                        user_id.into(),
                                        command.into(),
                                        response_url.clone(),
                                        trigger_id.into(),
                                    )
                                    .opt_text(text.cloned())),
                                    _ => Err(SlackClientError::SystemError(
                                        SlackClientSystemError::new(
                                            "Absent payload in the request from Slack".into(),
                                        ),
                                    ))
                                    .map_err(|e| e.into()),
                                }
                            })
                            .and_then(|event| async move {
                                match event {
                                    Ok(command_event) => {
                                        match serv(command_event, sc, thread_user_state_storage)
                                            .await
                                        {
                                            Ok(cresp) => Response::builder()
                                                .status(StatusCode::OK)
                                                .header(
                                                    "content-type",
                                                    "application/json; charset=utf-8",
                                                )
                                                .body(serde_json::to_string(&cresp).unwrap().into())
                                                .map_err(|e| e.into()),
                                            Err(_) => Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::empty())
                                                .map_err(|e| e.into()),
                                        }
                                    }
                                    Err(command_event_err) => {
                                        thread_error_handler(
                                            command_event_err,
                                            sc,
                                            thread_user_state_storage,
                                        );
                                        Ok(Response::new(Body::empty()))
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
