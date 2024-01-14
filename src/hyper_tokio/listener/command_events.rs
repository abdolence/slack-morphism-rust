use crate::errors::*;
use crate::hyper_tokio::connector::SlackClientHyperConnector;
use crate::listener::*;
use crate::signature_verifier::SlackEventSignatureVerifier;

use crate::hyper_tokio::hyper_ext::HyperExtensions;
use crate::hyper_tokio::*;
pub use crate::models::events::*;
pub use crate::models::SlackResponseUrl;
use futures::future::{BoxFuture, FutureExt, TryFutureExt};
use http_body_util::Empty;
use http_body_util::{BodyExt, Full};
use hyper::body::Incoming;
use hyper::{Method, Request, Response, StatusCode};
use hyper_util::client::legacy::connect::Connect;
use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;

impl<H: 'static + Send + Sync + Connect + Clone> SlackClientEventsHyperListener<H> {
    pub fn command_events_service_fn<'a, D, F, R>(
        &self,
        config: Arc<SlackCommandEventsListenerConfig>,
        command_service_fn: UserCallbackFunction<
            SlackCommandEvent,
            impl Future<Output = UserCallbackResult<R>> + 'static + Send,
            SlackClientHyperConnector<H>,
        >,
    ) -> impl Fn(Request<Incoming>, D) -> BoxFuture<'a, AnyStdResult<Response<Body>>> + 'a + Send + Clone
    where
        D: Fn(Request<Incoming>) -> F + 'a + Send + Sync + Clone,
        F: Future<Output = AnyStdResult<Response<Body>>> + 'a + Send,
        R: Into<Option<SlackCommandEventResponse>>,
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
                                        url::Url::parse(response_url)?.into(),
                                        trigger_id.into(),
                                    )
                                    .opt_text(text.cloned())),
                                    _ => Err(SlackClientError::SystemError(
                                        SlackClientSystemError::new().with_message(
                                            "Absent payload in the request from Slack".into(),
                                        ),
                                    ))
                                    .map_err(|e| e.into()),
                                }
                            })
                            .and_then(|event| async move {
                                match event {
                                    Ok(command_event) => {
                                        match command_service_fn(
                                            command_event,
                                            sc.clone(),
                                            thread_user_state_storage.clone(),
                                        )
                                        .await
                                        {
                                            Ok(cresp) => match cresp.into() {
                                                Some(cresp) => Response::builder()
                                                    .status(StatusCode::OK)
                                                    .header(
                                                        "content-type",
                                                        "application/json; charset=utf-8",
                                                    )
                                                    .body(
                                                        Full::new(
                                                            serde_json::to_string(&cresp)
                                                                .unwrap()
                                                                .into(),
                                                        )
                                                        .boxed(),
                                                    ),
                                                None => Response::builder()
                                                    .status(StatusCode::OK)
                                                    .body(Empty::new().boxed()),
                                            }
                                            .map_err(|e| e.into()),
                                            Err(err) => {
                                                let status_code = thread_error_handler(
                                                    err,
                                                    sc.clone(),
                                                    thread_user_state_storage.clone(),
                                                );
                                                Response::builder()
                                                    .status(status_code)
                                                    .body(Empty::new().boxed())
                                                    .map_err(|e| e.into())
                                            }
                                        }
                                    }
                                    Err(command_event_err) => {
                                        let status_code = thread_error_handler(
                                            command_event_err,
                                            sc,
                                            thread_user_state_storage,
                                        );
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
