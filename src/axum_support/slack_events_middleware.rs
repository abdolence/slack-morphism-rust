use crate::listener::SlackClientEventsListenerEnvironment;
use crate::prelude::hyper_ext::HyperExtensions;
use crate::signature_verifier::SlackEventSignatureVerifier;
use crate::SlackClientHttpConnector;
use axum::body::BoxBody;
use axum::{body::Body, http::Request, response::Response};
use futures_util::future::BoxFuture;
use std::sync::Arc;
use std::task::{Context, Poll};
use tower::Service;

#[derive(Clone)]
struct SlackEventsApiMiddleware<S, SCHC>
where
    SCHC: SlackClientHttpConnector + Send + Sync,
{
    inner: Option<S>,
    environment: Arc<SlackClientEventsListenerEnvironment<SCHC>>,
    signature_verifier: SlackEventSignatureVerifier,
}

impl<S, SCHC> Service<Request<Body>> for SlackEventsApiMiddleware<S, SCHC>
where
    S: Service<String, Response = Response> + Send + 'static + Clone,
    S::Future: Send + 'static,
    S::Error: std::error::Error + 'static + Send + Sync,
    SCHC: SlackClientHttpConnector + Send + Sync + 'static,
{
    type Response = S::Response;
    type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        if let Some(ref mut service) = self.inner.as_mut() {
            service.poll_ready(cx).map_err(|e| e.into())
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
                Ok(verified_body) => match service.call(verified_body).await {
                    Ok(response) => Ok(response),
                    Err(err) => {
                        let http_status = (environment.error_handler)(
                            Box::new(err),
                            environment.client.clone(),
                            environment.user_state.clone(),
                        );
                        Response::builder()
                            .status(http_status)
                            .body(BoxBody::default())
                            .map_err(|e| e.into())
                    }
                },
                Err(err) => {
                    let http_status = (environment.error_handler)(
                        err,
                        environment.client.clone(),
                        environment.user_state.clone(),
                    );
                    Response::builder()
                        .status(http_status)
                        .body(BoxBody::default())
                        .map_err(|e| e.into())
                }
            }
        })
    }
}
