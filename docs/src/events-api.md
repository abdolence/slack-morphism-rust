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

use slack_morphism::api::*;
use slack_morphism::listener::*;
use slack_morphism::*;
use slack_morphism_models::*;

// Slack Morphism Hyper/Tokio support
use slack_morphism_hyper::*;

// Hyper imports
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response};

// For logging
use log::*;

// For convinience there is an alias SlackHyperClient as SlackClient<SlackClientHyperConnector>

async fn create_slack_events_listener_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8080));
    info!("Loading server: {}", addr);

    // This is our default HTTP route when Slack routes didn't handle incoming request (different/other path).
    async fn your_others_routes(
        _req: Request<Body>,
    ) -> Result<Response<Body>, Box<dyn std::error::Error + Send + Sync>> {
        Response::builder()
            .body("Hey, this is a default users route handler".into())
            .map_err(|e| e.into())
    }
   
    // Our error handler for Slack Events API
    fn slack_listener_error_handler(err: Box<dyn std::error::Error + Send + Sync>, 
       _client: Arc<SlackHyperClient>, 
       _states: Arc<RwLock<SlackClientEventsUserStateStorage>>) {
        error!("Slack Events error: {:#?}", err);
    }

    // We need also a client instance. `Arc` used here because we would like 
    // to share the the same client for all of the requests and all hyper threads    
    
    let hyper_connector = SlackClientHyperConnector::new();
    let client: Arc<SlackClient<SlackClientHyperConnector>> = Arc::new(SlackClient::new(hyper_connector));
    

    // In this example we're going to use all of the events handlers, but
    // you don't have to.

    // Our Slack OAuth handler with a token response after installation
    async fn slack_oauth_install_function(
        resp: SlackOAuthV2AccessTokenResponse,
        _client: Arc<SlackHyperClient>,
        _states: Arc<RwLock<SlackClientEventsUserStateStorage>>
    ) {
        println!("{:#?}", resp);
    }

    // Push events handler
    async fn slack_push_events_function(event: SlackPushEvent, 
       _client: Arc<SlackHyperClient>, 
       _states: Arc<RwLock<SlackClientEventsUserStateStorage>>) {
        println!("{:#?}", event);
    }

    // Interaction events handler
    async fn slack_interaction_events_function(event: SlackInteractionEvent, 
        _client: Arc<SlackHyperClient>,
        _states: Arc<RwLock<SlackClientEventsUserStateStorage>>) {
        println!("{:#?}", event);
    }

    // Commands events handler
    async fn slack_command_events_function(
        event: SlackCommandEvent,
        _client: Arc<SlackHyperClient>,
        _states: Arc<RwLock<SlackClientEventsUserStateStorage>>
    ) -> Result<SlackCommandEventResponse, Box<dyn std::error::Error + Send + Sync>> {
        println!("{:#?}", event);
        Ok(SlackCommandEventResponse::new(
            SlackMessageContent::new().with_text("Working on it".into()),
        ))
    }

    // Now we need some configuration for our Slack listener routes.
    // You can additionally configure HTTP route paths using theses configs,
    // but for simplicity we will skip that part here and configure only required parameters.
    let oauth_listener_config = Arc::new(SlackOAuthListenerConfig::new(
        std::env::var("SLACK_CLIENT_ID")?,
        std::env::var("SLACK_CLIENT_SECRET")?,
        std::env::var("SLACK_BOT_SCOPE")?,
        std::env::var("SLACK_REDIRECT_HOST")?,
    ));

    let push_events_config = Arc::new(SlackPushEventsListenerConfig::new(std::env::var(
        "SLACK_SIGNING_SECRET",
    )?));

    let interactions_events_config = Arc::new(SlackInteractionEventsListenerConfig::new(
        std::env::var("SLACK_SIGNING_SECRET")?,
    ));

    let command_events_config = Arc::new(SlackCommandEventsListenerConfig::new(std::env::var(
        "SLACK_SIGNING_SECRET",
    )?));

    // Creating a shared listener environment with an ability to share client and user state
    let listener_environment = Arc::new(
        SlackClientEventsListenerEnvironment::new(client.clone())
            .with_error_handler(test_error_handler)
    );
   
    let make_svc = make_service_fn(move |_| {
        // Because of threading model you have to create copies of configs.
        let thread_oauth_config = oauth_listener_config.clone();
        let thread_push_events_config = push_events_config.clone();
        let thread_interaction_events_config = interactions_events_config.clone();
        let thread_command_events_config = command_events_config.clone();
 
        // Creating listener
        let listener = SlackClientEventsHyperListener::new(listener_environment.clone());
        
        // Chaining all of the possible routes for Slack.
        // `chain_service_routes_fn` is an auxiliary function from Slack Morphism. 
        async move {
            let routes = chain_service_routes_fn(
                listener.oauth_service_fn(thread_oauth_config, test_oauth_install_function),
                chain_service_routes_fn(
                    listener.push_events_service_fn(
                        thread_push_events_config,
                        slack_push_events_function,
                    ),
                    chain_service_routes_fn(
                        listener.interaction_events_service_fn(
                            thread_interaction_events_config,
                            slack_interaction_events_function,
                        ),
                        chain_service_routes_fn(
                            listener.command_events_service_fn(
                                thread_command_events_config,
                                slack_command_events_function,
                            ),
                            your_others_routes,
                        ),
                    ),
                ),
            );

            Ok::<_, Box<dyn std::error::Error + Send + Sync>>(service_fn(routes))
        }

    )};

    // Starting a server with listener routes
    let server = hyper::server::Server::bind(&addr).serve(make_svc);
    server.await.map_err(|e| {
        error!("Server error: {}", e);
        e.into()
    })
}
``` 

 Also the library provides Slack events signature verifier (`SlackEventSignatureVerifier`),
 which is already integrated in the routes implementation for you and you don't need to use 
 it directly. All you need is provide your client id and secret configuration 
 to route implementation.

 Look at the [examples/test_server](https://github.com/abdolence/slack-morphism-rust/tree/master/src/examples/src) sources for a complete ready to use example.
 
