use crate::axum_support::SlackEventsAxumListener;
use crate::hyper_tokio::SlackClientHyperConnector;
use crate::listener::SlackClientEventsListenerEnvironment;
use crate::prelude::hyper_ext::HyperExtensions;
use crate::signature_verifier::SlackEventSignatureVerifier;
use crate::{SlackClientHttpConnector, SlackSigningSecret};
use axum::body::BoxBody;
use axum::response::IntoResponse;
use axum::{body::Body, http::Request, response::Response};
use futures_util::future::BoxFuture;
use hyper::client::connect::Connect;
use std::convert::Infallible;
use std::marker::PhantomData;
use std::sync::Arc;
use std::task::{Context, Poll};
use tower::{Layer, Service};

#[derive(Clone)]
pub struct SlackEventsApiMiddlewareService<S, SCHC>
where
    SCHC: SlackClientHttpConnector + Send + Sync,
{
    inner: Option<S>,
    environment: Arc<SlackClientEventsListenerEnvironment<SCHC>>,
    signature_verifier: SlackEventSignatureVerifier,
}

impl<S, SCHC, I> SlackEventsApiMiddlewareService<S, SCHC>
where
    S: Service<Request<Body>, Response = I> + Send + 'static + Clone,
    S::Future: Send + 'static,
    S::Error: std::error::Error + 'static + Send + Sync,
    I: IntoResponse,
    SCHC: SlackClientHttpConnector + Send + Sync + 'static,
{
    pub fn new(
        service: S,
        environment: Arc<SlackClientEventsListenerEnvironment<SCHC>>,
        secret: &SlackSigningSecret,
    ) -> Self {
        Self {
            inner: Some(service),
            environment,
            signature_verifier: SlackEventSignatureVerifier::new(secret),
        }
    }
}

impl<S, SCHC> Service<Request<Body>> for SlackEventsApiMiddlewareService<S, SCHC>
where
    S: Service<Request<Body>, Response = Response, Error = Infallible> + Send + 'static + Clone,
    S::Future: Send + 'static,
    SCHC: SlackClientHttpConnector + Send + Sync + 'static,
{
    type Response = S::Response;
    type Error = Infallible;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Infallible>> {
        if let Some(ref mut service) = self.inner.as_mut() {
            service.poll_ready(cx)
        } else {
            Poll::Pending
        }
    }

    fn call(&mut self, request: Request<Body>) -> Self::Future {
        let mut service = self.inner.take().unwrap();
        self.inner = Some(service.clone());
        let environment = self.environment.clone();

        let signature_verifier = self.signature_verifier.clone();

        Box::pin(async move {
            match HyperExtensions::decode_signed_response(request, &signature_verifier).await {
                Ok(verified_body) => {
                    let verified_request = Request::new(Body::from(verified_body));
                    match service.call(verified_request).await {
                        Ok(response) => Ok(response),
                        Err(err) => {
                            let http_status = (environment.error_handler)(
                                Box::new(err),
                                environment.client.clone(),
                                environment.user_state.clone(),
                            );
                            Ok(Response::builder()
                                .status(http_status)
                                .body(BoxBody::default())
                                .unwrap())
                        }
                    }
                }
                Err(err) => {
                    let http_status = (environment.error_handler)(
                        err,
                        environment.client.clone(),
                        environment.user_state.clone(),
                    );
                    Ok(Response::builder()
                        .status(http_status)
                        .body(BoxBody::default())
                        .unwrap())
                }
            }
        })
    }
}

pub struct SlackEventsApiMiddleware<SCHC, S>
where
    SCHC: SlackClientHttpConnector + Send + Sync,
{
    slack_signing_secret: SlackSigningSecret,
    environment: Arc<SlackClientEventsListenerEnvironment<SCHC>>,
    _ph: PhantomData<S>,
}

impl<SCHC, S> SlackEventsApiMiddleware<SCHC, S>
where
    SCHC: SlackClientHttpConnector + Send + Sync,
{
    pub fn new(
        environment: Arc<SlackClientEventsListenerEnvironment<SCHC>>,
        slack_signing_secret: &SlackSigningSecret,
    ) -> Self {
        Self {
            slack_signing_secret: slack_signing_secret.clone(),
            environment,
            _ph: PhantomData::default(),
        }
    }
}

impl<S, SCHC, I> Layer<S> for SlackEventsApiMiddleware<SCHC, S>
where
    S: Service<Request<Body>, Response = I> + Send + 'static + Clone,
    S::Future: Send + 'static,
    S::Error: std::error::Error + 'static + Send + Sync,
    I: IntoResponse,
    SCHC: SlackClientHttpConnector + Send + Sync + 'static,
{
    type Service = SlackEventsApiMiddlewareService<S, SCHC>;

    fn layer(&self, service: S) -> SlackEventsApiMiddlewareService<S, SCHC> {
        SlackEventsApiMiddlewareService::new(
            service,
            self.environment.clone(),
            &self.slack_signing_secret,
        )
    }
}

impl<H: 'static + Send + Sync + Connect + Clone> SlackEventsAxumListener<H> {
    pub fn events_layer<S, ReqBody, I>(
        &self,
        slack_signing_secret: &SlackSigningSecret,
    ) -> SlackEventsApiMiddleware<SlackClientHyperConnector<H>, S>
    where
        S: Service<Request<ReqBody>, Response = I> + Send + 'static + Clone,
        S::Future: Send + 'static,
        S::Error: std::error::Error + 'static + Send + Sync,
        I: IntoResponse,
    {
        SlackEventsApiMiddleware::new(self.environment.clone(), slack_signing_secret)
    }
}
