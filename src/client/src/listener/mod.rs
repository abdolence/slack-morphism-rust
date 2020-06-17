use futures::future::{BoxFuture, FutureExt};
use hyper::{Body, Request, Response};
use std::future::Future;

pub mod oauth;
pub mod push_events;

pub mod signature_verifier;


pub fn chain_service_routes_fn<'a, R, D, FR, FD>(
    route: R,
    default: D,
) -> impl Fn(
    Request<Body>,
) -> BoxFuture<'a, Result<Response<Body>, Box<dyn std::error::Error + Send + Sync + 'a>>>
       + 'a
       + Send
       + Clone
where
    R: Fn(Request<Body>, D) -> FR + 'a + Clone + Send,
    D: Fn(Request<Body>) -> FD + 'a + Clone + Send,
    FR: Future<Output = Result<Response<Body>, Box<dyn std::error::Error + Send + Sync + 'a>>>
        + 'a
        + Send,
    FD: Future<Output = Result<Response<Body>, Box<dyn std::error::Error + Send + Sync + 'a>>>
        + 'a
        + Send,
{
    move |req: Request<Body>| route(req, default.clone()).boxed()
}
