use crate::listener::SlackClientEventsListenerEnvironment;
use crate::prelude::hyper_reqresp::HyperReqRespUtils;
use crate::signature_verifier::SlackEventSignatureVerifier;
use crate::SlackClientHttpConnector;
use axum::{body::Body, http::Request, response::Response};
use futures_util::future::BoxFuture;
use futures_util::TryFutureExt;
use std::sync::Arc;
use std::task::{Context, Poll};
use tower::{Layer, Service};

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
    SCHC: SlackClientHttpConnector + Send + Sync + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        if let Some(ref mut service) = self.inner.as_mut() {
            service.poll_ready(cx).map_err(|e| e.into())
        } else {
            Poll::Pending
        }
    }

    fn call(&mut self, mut request: Request<Body>) -> Self::Future {
        let mut service = self.inner.take().unwrap();
        self.inner = Some(service.clone());
        let environment = self.environment.clone();

        let signature_verifier = self.signature_verifier.clone();

        Box::pin(async move {
            match HyperReqRespUtils::decode_signed_response(request, &signature_verifier).await {
                Ok(verified_body) => {
                    let response: Response = service.call(verified_body).await?;
                    Ok(response)
                }
                Err(err) => {
                    (environment.error_handler)(
                        err,
                        environment.client.clone(),
                        environment.user_state.clone(),
                    );
                    todo!()
                }
            }
        })
    }
}
