use crate::connector::SlackClientHyperConnector;
use crate::listener::SlackClientEventsHyperListener;

use slack_morphism::errors::*;
use slack_morphism::listener::*;
use slack_morphism::signature_verifier::SlackEventSignatureVerifier;
use slack_morphism::SlackClient;

use futures::future::{BoxFuture, FutureExt, TryFutureExt};
use hyper::body::*;
use hyper::{Method, Request, Response, StatusCode};
use log::*;
pub use slack_morphism_models::events::*;
use std::future::Future;
use std::sync::{Arc, RwLock};

impl SlackClientEventsHyperListener {
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
        I: Fn(
                SlackPushEvent,
                Arc<SlackClient<SlackClientHyperConnector>>,
                Arc<RwLock<SlackClientEventsUserStateStorage>>,
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
        let client = self.environment.client.clone();
        let error_handler = self.environment.error_handler.clone();
        let user_state_storage = self.environment.user_state_storage.clone();

        move |req: Request<Body>, chain: D| {
            let cfg = config.clone();
            let push_serv = push_service_fn.clone();
            let sign_verifier = signature_verifier.clone();
            let sc = client.clone();
            let thread_error_handler = error_handler.clone();
            let thread_user_state_storage = user_state_storage.clone();
            async move {
                match (req.method(), req.uri().path()) {
                    (&Method::POST, url) if url == cfg.events_path => {
                        SlackClientHyperConnector::decode_signed_response(req, &sign_verifier)
                            .map_ok(|body| {
                                serde_json::from_str::<SlackPushEvent>(body.as_str()).map_err(|e| {
                                    SlackClientProtocolError {
                                        json_error: e,
                                        http_response_body: body.clone(),
                                    }
                                    .into()
                                })
                            })
                            .and_then(|event| async move {
                                match event {
                                    Ok(SlackPushEvent::UrlVerification(url_ver)) => {
                                        debug!(
                                            "Received Slack URL push verification challenge: {}",
                                            url_ver.challenge
                                        );
                                        push_serv(
                                            SlackPushEvent::UrlVerification(url_ver.clone()),
                                            sc,
                                            thread_user_state_storage,
                                        )
                                        .await;
                                        Response::builder()
                                            .status(StatusCode::OK)
                                            .body(url_ver.challenge.into())
                                            .map_err(|e| e.into())
                                    }
                                    other => match other {
                                        Ok(push_event) => {
                                            push_serv(push_event, sc, thread_user_state_storage)
                                                .await;
                                            Ok(Response::new(Body::empty()))
                                        }
                                        Err(err_oush_event) => {
                                            thread_error_handler(
                                                err_oush_event,
                                                sc,
                                                thread_user_state_storage,
                                            );
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
                    _ => chain(req).await,
                }
            }
            .boxed()
        }
    }
}
