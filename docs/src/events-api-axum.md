 # Events API and OAuth

 The library provides route implementation in `SlackClientEventsListener` based on Hyper/Tokio for:
 - Push Events
 - Interaction Events
 - Command Events
 - OAuth v2 redirects and client functions

 You can chain all of the routes using `chain_service_routes_fn` from the library.

## Hyper configuration
In order to use Events API/OAuth you need to configure Hyper HTTP server. 
There is nothing special about how to do that, and you can use [the official hyper docs](https://hyper.rs/).
This is just merely a quick example how to use it with Slack Morphism routes.

To create a server, you need hyper `make_service_fn` and `service_fn`.

## Example
```rust,noplaypen

use slack_morphism::prelude::*;

use hyper::{Body, Response};
use tracing::*;

use axum::Extension;
use std::sync::Arc;

async fn test_oauth_install_function(
    resp: SlackOAuthV2AccessTokenResponse,
    _client: Arc<SlackHyperClient>,
    _states: SlackClientEventsUserState,
) {
    println!("{:#?}", resp);
}

async fn test_push_event(
    Extension(_environment): Extension<Arc<SlackHyperListenerEnvironment>>,
    Extension(event): Extension<SlackPushEvent>,
) -> Response<Body> {
    println!("Received push event: {:?}", event);

    match event {
        SlackPushEvent::UrlVerification(url_ver) => Response::new(Body::from(url_ver.challenge)),
        _ => Response::new(Body::empty()),
    }
}

async fn test_command_event(
    Extension(_environment): Extension<Arc<SlackHyperListenerEnvironment>>,
    Extension(event): Extension<SlackCommandEvent>,
) -> axum::Json<SlackCommandEventResponse> {
    println!("Received command event: {:?}", event);
    axum::Json(SlackCommandEventResponse::new(
        SlackMessageContent::new().with_text("Working on it".into()),
    ))
}

async fn test_interaction_event(
    Extension(_environment): Extension<Arc<SlackHyperListenerEnvironment>>,
    Extension(event): Extension<SlackInteractionEvent>,
) {
    println!("Received interaction event: {:?}", event);
}

fn test_error_handler(
    err: Box<dyn std::error::Error + Send + Sync>,
    _client: Arc<SlackHyperClient>,
    _states: SlackClientEventsUserState,
) -> http::StatusCode {
    println!("{:#?}", err);

    // Defines what we return Slack server
    http::StatusCode::BAD_REQUEST
}

async fn test_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client: Arc<SlackHyperClient> =
        Arc::new(SlackClient::new(SlackClientHyperConnector::new()));

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8080));
    info!("Loading server: {}", addr);

    let oauth_listener_config = SlackOAuthListenerConfig::new(
        config_env_var("SLACK_CLIENT_ID")?.into(),
        config_env_var("SLACK_CLIENT_SECRET")?.into(),
        config_env_var("SLACK_BOT_SCOPE")?,
        config_env_var("SLACK_REDIRECT_HOST")?,
    );

    let listener_environment: Arc<SlackHyperListenerEnvironment> = Arc::new(
        SlackClientEventsListenerEnvironment::new(client.clone())
            .with_error_handler(test_error_handler),
    );
    let signing_secret: SlackSigningSecret = config_env_var("SLACK_SIGNING_SECRET")?.into();

    let listener: SlackEventsAxumListener<SlackHyperHttpsConnector> =
        SlackEventsAxumListener::new(listener_environment.clone());

    // build our application route with OAuth nested router and Push/Command/Interaction events
    let app = axum::routing::Router::new()
        .nest(
            "/auth",
            listener.oauth_router("/auth", &oauth_listener_config, test_oauth_install_function),
        )
        .route(
            "/push",
            axum::routing::post(test_push_event).layer(
                listener
                    .events_layer(&signing_secret)
                    .with_event_extractor(SlackEventsExtractors::push_event()),
            ),
        )
        .route(
            "/command",
            axum::routing::post(test_command_event).layer(
                listener
                    .events_layer(&signing_secret)
                    .with_event_extractor(SlackEventsExtractors::command_event()),
            ),
        )
        .route(
            "/interaction",
            axum::routing::post(test_interaction_event).layer(
                listener
                    .events_layer(&signing_secret)
                    .with_event_extractor(SlackEventsExtractors::interaction_event()),
            ),
        );

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

``` 
Complete example look at [github](https://github.com/abdolence/slack-morphism-rust/tree/master/examples)
