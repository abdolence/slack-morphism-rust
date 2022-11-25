use crate::axum_support::slack_events_extractors::{
    SlackEventsEmptyExtractor, SlackEventsExtractor,
};
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
use tracing::*;

#[derive(Clone)]
pub struct SlackEventsApiMiddlewareService<S, SCHC, SE>
where
    SCHC: SlackClientHttpConnector + Send + Sync,
    SE: SlackEventsExtractor + Clone,
{
    inner: Option<S>,
    environment: Arc<SlackClientEventsListenerEnvironment<SCHC>>,
    signature_verifier: SlackEventSignatureVerifier,
    extractor: SE,
}

impl<S, SCHC, I, SE> SlackEventsApiMiddlewareService<S, SCHC, SE>
where
    S: Service<Request<Body>, Response = I> + Send + 'static + Clone,
    S::Future: Send + 'static,
    S::Error: std::error::Error + 'static + Send + Sync,
    I: IntoResponse,
    SCHC: SlackClientHttpConnector + Send + Sync + 'static,
    SE: SlackEventsExtractor + Clone,
{
    pub fn new(
        service: S,
        environment: Arc<SlackClientEventsListenerEnvironment<SCHC>>,
        secret: &SlackSigningSecret,
        extractor: SE,
    ) -> Self {
        Self {
            inner: Some(service),
            environment,
            signature_verifier: SlackEventSignatureVerifier::new(secret),
            extractor,
        }
    }
}

impl<S, SCHC, SE> Service<Request<Body>> for SlackEventsApiMiddlewareService<S, SCHC, SE>
where
    S: Service<Request<Body>, Response = Response, Error = Infallible> + Send + 'static + Clone,
    S::Future: Send + 'static,
    SCHC: SlackClientHttpConnector + Send + Sync + 'static,
    SE: SlackEventsExtractor + Clone + Send + Sync + 'static,
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
        let extractor = self.extractor.clone();
        let request_uri = request.uri().clone();

        debug!("Received Slack event: {}", &request_uri);

        Box::pin(async move {
            match HyperExtensions::decode_signed_response(request, &signature_verifier).await {
                Ok((verified_body, parts)) => {
                    let mut verified_request = Request::from_parts(parts, Body::empty());

                    verified_request
                        .extensions_mut()
                        .insert(environment.clone());

                    if let Err(err) =
                        extractor.extract(verified_body.as_str(), verified_request.extensions_mut())
                    {
                        let http_status = (environment.error_handler)(
                            err,
                            environment.client.clone(),
                            environment.user_state.clone(),
                        );
                        Ok(Response::builder()
                            .status(http_status)
                            .body(BoxBody::default())
                            .unwrap())
                    } else {
                        *verified_request.body_mut() = Body::from(verified_body);

                        debug!("Calling a route service with Slack event: {}", &request_uri);

                        match service.call(verified_request).await {
                            Ok(response) => {
                                debug!("Route service finished successfully for: {}", &request_uri);
                                Ok(response)
                            }
                            Err(err) => {
                                debug!("A route service failed: {} with {}", &request_uri, err);

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
                }
                Err(err) => {
                    debug!("Slack event error: {}", err);
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

#[derive(Clone)]
pub struct SlackEventsApiMiddleware<SCHC, S, SE>
where
    SCHC: SlackClientHttpConnector + Send + Sync + Clone,
    SE: SlackEventsExtractor + Clone,
{
    slack_signing_secret: SlackSigningSecret,
    environment: Arc<SlackClientEventsListenerEnvironment<SCHC>>,
    extractor: SE,
    _ph_s: PhantomData<S>,
}

impl<SCHC, S> SlackEventsApiMiddleware<SCHC, S, SlackEventsEmptyExtractor>
where
    SCHC: SlackClientHttpConnector + Send + Sync + Clone,
{
    pub fn new(
        environment: Arc<SlackClientEventsListenerEnvironment<SCHC>>,
        slack_signing_secret: &SlackSigningSecret,
    ) -> Self {
        Self {
            slack_signing_secret: slack_signing_secret.clone(),
            environment,
            extractor: SlackEventsEmptyExtractor::new(),
            _ph_s: PhantomData::default(),
        }
    }

    pub fn with_event_extractor<SE>(self, extractor: SE) -> SlackEventsApiMiddleware<SCHC, S, SE>
    where
        SE: SlackEventsExtractor + Clone,
    {
        SlackEventsApiMiddleware {
            slack_signing_secret: self.slack_signing_secret,
            environment: self.environment,
            extractor,
            _ph_s: PhantomData::default(),
        }
    }
}

impl<S, SCHC, I, SE> Layer<S> for SlackEventsApiMiddleware<SCHC, S, SE>
where
    S: Service<Request<Body>, Response = I> + Send + 'static + Clone,
    S::Future: Send + 'static,
    S::Error: std::error::Error + 'static + Send + Sync,
    I: IntoResponse,
    SCHC: SlackClientHttpConnector + Send + Sync + 'static + Clone,
    SE: SlackEventsExtractor + Clone,
{
    type Service = SlackEventsApiMiddlewareService<S, SCHC, SE>;

    fn layer(&self, service: S) -> SlackEventsApiMiddlewareService<S, SCHC, SE> {
        SlackEventsApiMiddlewareService::new(
            service,
            self.environment.clone(),
            &self.slack_signing_secret,
            self.extractor.clone(),
        )
    }
}

impl<H: 'static + Send + Sync + Connect + Clone> SlackEventsAxumListener<H> {
    pub fn events_layer<S, ReqBody, I>(
        &self,
        slack_signing_secret: &SlackSigningSecret,
    ) -> SlackEventsApiMiddleware<SlackClientHyperConnector<H>, S, SlackEventsEmptyExtractor>
    where
        S: Service<Request<ReqBody>, Response = I> + Send + 'static + Clone,
        S::Future: Send + 'static,
        S::Error: std::error::Error + 'static + Send + Sync,
        I: IntoResponse,
    {
        SlackEventsApiMiddleware::new(self.environment.clone(), slack_signing_secret)
    }
}
