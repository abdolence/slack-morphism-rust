use crate::errors::*;
use crate::hyper_tokio::connector::SlackClientHyperConnector;
use crate::hyper_tokio::hyper_ext::HyperExtensions;
use crate::hyper_tokio::*;
use crate::listener::*;
use crate::signature_verifier::SlackEventSignatureVerifier;
use futures::future::{BoxFuture, FutureExt, TryFutureExt};
use http_body_util::{BodyExt, Empty, Full};
use hyper::body::Incoming;
use hyper::{Method, Request, Response};
use hyper_util::client::legacy::connect::Connect;
use std::future::Future;
use std::sync::Arc;
use tracing::*;

pub use crate::models::events::*;

impl<H: 'static + Send + Sync + Connect + Clone> SlackClientEventsHyperListener<H> {
    pub fn push_events_service_fn<'a, D, F>(
        &self,
        config: Arc<SlackPushEventsListenerConfig>,
        push_service_fn: UserCallbackFunction<
            SlackPushEvent,
            impl Future<Output = UserCallbackResult<()>> + 'static + Send,
            SlackClientHyperConnector<H>,
        >,
    ) -> impl Fn(Request<Incoming>, D) -> BoxFuture<'a, AnyStdResult<Response<Body>>> + 'a + Send + Clone
    where
        D: Fn(Request<Incoming>) -> F + 'a + Send + Sync + Clone,
        F: Future<Output = AnyStdResult<Response<Body>>> + 'a + Send,
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
                                serde_json::from_str::<SlackPushEvent>(body.as_str()).map_err(|e| {
                                    SlackClientProtocolError::new(e)
                                        .with_json_body(body.clone())
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
                                        match push_service_fn(
                                            SlackPushEvent::UrlVerification(url_ver.clone()),
                                            sc.clone(),
                                            thread_user_state_storage.clone(),
                                        )
                                        .await
                                        {
                                            Ok(_) => Ok(Response::new(
                                                Full::new(url_ver.challenge.into()).boxed(),
                                            )),
                                            Err(err) => {
                                                let status_code = thread_error_handler(
                                                    err,
                                                    sc,
                                                    thread_user_state_storage,
                                                );
                                                Response::builder()
                                                    .status(status_code)
                                                    .body(Empty::new().boxed())
                                                    .map_err(|e| e.into())
                                            }
                                        }
                                    }
                                    other => match other {
                                        Ok(push_event) => {
                                            match push_service_fn(
                                                push_event,
                                                sc.clone(),
                                                thread_user_state_storage.clone(),
                                            )
                                            .await
                                            {
                                                Ok(_) => Ok(Response::new(Empty::new().boxed())),
                                                Err(err) => {
                                                    let status_code = thread_error_handler(
                                                        err,
                                                        sc,
                                                        thread_user_state_storage,
                                                    );
                                                    Response::builder()
                                                        .status(status_code)
                                                        .body(Empty::new().boxed())
                                                        .map_err(|e| e.into())
                                                }
                                            }
                                        }
                                        Err(err_oush_event) => {
                                            let status_code = thread_error_handler(
                                                err_oush_event,
                                                sc,
                                                thread_user_state_storage,
                                            );
                                            Response::builder()
                                                .status(status_code)
                                                .body(Empty::new().boxed())
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
