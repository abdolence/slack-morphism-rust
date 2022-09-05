use slack_morphism::prelude::*;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response};
use tracing::*;

use std::sync::Arc;

async fn test_oauth_install_function(
    resp: SlackOAuthV2AccessTokenResponse,
    _client: Arc<SlackHyperClient>,
    _states: SlackClientEventsUserState,
) {
    println!("{:#?}", resp);
}

async fn test_push_events_function(
    event: SlackPushEvent,
    _client: Arc<SlackHyperClient>,
    _states: SlackClientEventsUserState,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Read state
    let current_state = {
        let states = _states.read().await;
        println!("{:#?}", states.get_user_state::<UserStateExample>());
        println!("{:#?}", states.len());
        UserStateExample(states.get_user_state::<UserStateExample>().unwrap().0 + 1)
    };

    // Write state
    {
        let mut states = _states.write().await;
        states.set_user_state::<UserStateExample>(current_state);
        println!("{:#?}", states.get_user_state::<UserStateExample>());
    }

    println!("{:#?}", event);
    Ok(())
}

async fn test_interaction_events_function(
    event: SlackInteractionEvent,
    _client: Arc<SlackHyperClient>,
    _states: SlackClientEventsUserState,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("{:#?}", event);
    Ok(())
}

async fn test_command_events_function(
    event: SlackCommandEvent,
    _client: Arc<SlackHyperClient>,
    _states: SlackClientEventsUserState,
) -> Result<SlackCommandEventResponse, Box<dyn std::error::Error + Send + Sync>> {
    let token_value: SlackApiTokenValue = config_env_var("SLACK_TEST_TOKEN")?.into();
    let token: SlackApiToken = SlackApiToken::new(token_value);
    let session = _client.open_session(&token);

    session
        .api_test(&SlackApiTestRequest::new().with_foo("Test".into()))
        .await?;

    println!("{:#?}", event);
    Ok(SlackCommandEventResponse::new(
        SlackMessageContent::new().with_text("Working on it".into()),
    ))
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

#[derive(Debug)]
struct UserStateExample(u64);

async fn test_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client: Arc<SlackHyperClient> =
        Arc::new(SlackClient::new(SlackClientHyperConnector::new()));

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8080));
    info!("Loading server: {}", addr);

    async fn your_others_routes(
        _req: Request<Body>,
    ) -> Result<Response<Body>, Box<dyn std::error::Error + Send + Sync>> {
        Response::builder()
            .body("Hey, this is a default users route handler".into())
            .map_err(|e| e.into())
    }

    let oauth_listener_config = Arc::new(SlackOAuthListenerConfig::new(
        config_env_var("SLACK_CLIENT_ID")?.into(),
        config_env_var("SLACK_CLIENT_SECRET")?.into(),
        config_env_var("SLACK_BOT_SCOPE")?,
        config_env_var("SLACK_REDIRECT_HOST")?,
    ));

    let push_events_config = Arc::new(SlackPushEventsListenerConfig::new(
        config_env_var("SLACK_SIGNING_SECRET")?.into(),
    ));

    let interactions_events_config = Arc::new(SlackInteractionEventsListenerConfig::new(
        config_env_var("SLACK_SIGNING_SECRET")?.into(),
    ));

    let command_events_config = Arc::new(SlackCommandEventsListenerConfig::new(
        config_env_var("SLACK_SIGNING_SECRET")?.into(),
    ));

    let listener_environment = Arc::new(
        SlackClientEventsListenerEnvironment::new(client.clone())
            .with_error_handler(test_error_handler)
            .with_user_state(UserStateExample(0)),
    );

    let make_svc = make_service_fn(move |_| {
        let thread_oauth_config = oauth_listener_config.clone();
        let thread_push_events_config = push_events_config.clone();
        let thread_interaction_events_config = interactions_events_config.clone();
        let thread_command_events_config = command_events_config.clone();
        let listener = SlackClientEventsHyperListener::new(listener_environment.clone());
        async move {
            let routes = chain_service_routes_fn(
                listener.oauth_service_fn(thread_oauth_config, test_oauth_install_function),
                chain_service_routes_fn(
                    listener.push_events_service_fn(
                        thread_push_events_config,
                        test_push_events_function,
                    ),
                    chain_service_routes_fn(
                        listener.interaction_events_service_fn(
                            thread_interaction_events_config,
                            test_interaction_events_function,
                        ),
                        chain_service_routes_fn(
                            listener.command_events_service_fn(
                                thread_command_events_config,
                                test_command_events_function,
                            ),
                            your_others_routes,
                        ),
                    ),
                ),
            );

            Ok::<_, Box<dyn std::error::Error + Send + Sync>>(service_fn(routes))
        }
    });

    info!("Server is listening on {}", &addr);

    let server = hyper::server::Server::bind(&addr).serve(make_svc);
    server.await.map_err(|e| {
        error!("Server error: {}", e);
        e.into()
    })
}

pub fn config_env_var(name: &str) -> Result<String, String> {
    std::env::var(name).map_err(|e| format!("{}: {}", name, e))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter("events_api_server=debug,slack_morphism=debug")
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    test_server().await?;

    Ok(())
}
