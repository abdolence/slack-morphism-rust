use slack_morphism::prelude::*;
use slack_morphism_hyper::*;

use futures::stream::BoxStream;
use futures::TryStreamExt;
use std::time::Duration;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response};
use log::*;

use std::sync::{Arc, RwLock};

mod templates;
use templates::*;

#[allow(dead_code)]
async fn test_client() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = SlackClient::new(SlackClientHyperConnector::new());
    let token_value: SlackApiTokenValue = config_env_var("SLACK_TEST_TOKEN")?.into();
    let token: SlackApiToken = SlackApiToken::new(token_value);
    let session = client.open_session(&token);
    println!("{:#?}", session);

    let test: SlackApiTestResponse = session
        .api_test(&SlackApiTestRequest::new().with_foo("Test".into()))
        .await?;

    println!("{:#?}", test);

    let message = WelcomeMessageTemplateParams::new("".into());

    let post_chat_req =
        SlackApiChatPostMessageRequest::new("#general".into(), message.render_template());

    let post_chat_resp = session.chat_post_message(&post_chat_req).await?;
    println!("post chat resp: {:#?}", &post_chat_resp);

    let scroller_req: SlackApiUsersListRequest = SlackApiUsersListRequest::new().with_limit(1);
    let scroller = scroller_req.scroller();

    let mut resp_stream: BoxStream<ClientResult<SlackApiUsersListResponse>> =
        scroller.to_stream(&session);

    while let Some(item) = resp_stream.try_next().await? {
        println!("res: {:#?}", item.members);
    }

    let collected_members: Vec<SlackUser> = scroller
        .collect_items_stream(&session, Duration::from_millis(1000))
        .await?;
    println!("collected res: {:#?}", collected_members);

    let mut items_stream = scroller.to_items_stream(&session);
    while let Some(items) = items_stream.try_next().await? {
        println!("res: {:#?}", items);
    }

    let mut items_throttled_stream =
        scroller.to_items_throttled_stream(&session, Duration::from_millis(500));
    while let Some(items) = items_throttled_stream.try_next().await? {
        println!("res: {:#?}", items);
    }

    Ok(())
}

async fn test_oauth_install_function(
    resp: SlackOAuthV2AccessTokenResponse,
    _client: Arc<SlackHyperClient>,
    _states: Arc<RwLock<SlackClientEventsUserStateStorage>>,
) {
    println!("{:#?}", resp);
}

async fn test_push_events_function(
    event: SlackPushEvent,
    _client: Arc<SlackHyperClient>,
    _states: Arc<RwLock<SlackClientEventsUserStateStorage>>,
) {
    // Read state
    let current_state = {
        let states = _states.read().unwrap();
        println!("{:#?}", states.get_user_state::<UserStateExample>());
        println!("{:#?}", states.len());
        UserStateExample(states.get_user_state::<UserStateExample>().unwrap().0 + 1)
    };

    // Write state
    {
        let mut states = _states.write().unwrap();
        states.set_user_state::<UserStateExample>(current_state);
        println!("{:#?}", states.get_user_state::<UserStateExample>());
    }

    println!("{:#?}", event);
}

async fn test_interaction_events_function(
    event: SlackInteractionEvent,
    _client: Arc<SlackHyperClient>,
    _states: Arc<RwLock<SlackClientEventsUserStateStorage>>,
) {
    println!("{:#?}", event);
}

async fn test_command_events_function(
    event: SlackCommandEvent,
    _client: Arc<SlackHyperClient>,
    _states: Arc<RwLock<SlackClientEventsUserStateStorage>>,
) -> Result<SlackCommandEventResponse, Box<dyn std::error::Error + Send + Sync>> {
    println!("{:#?}", event);
    Ok(SlackCommandEventResponse::new(
        SlackMessageContent::new().with_text("Working on it".into()),
    ))
}

async fn test_push_events_sm_function(
    event: SlackPushEventCallback,
    _client: Arc<SlackHyperClient>,
    _states: Arc<RwLock<SlackClientEventsUserStateStorage>>,
) {
    println!("{:#?}", event);
}

fn test_error_handler(
    err: Box<dyn std::error::Error + Send + Sync>,
    _client: Arc<SlackHyperClient>,
    _states: Arc<RwLock<SlackClientEventsUserStateStorage>>,
) -> http::StatusCode {
    println!("{:#?}", err);

    // Defines what we return Slack server
    http::StatusCode::BAD_REQUEST
}

#[derive(Debug)]
struct UserStateExample(u64);

#[allow(dead_code)]
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
        config_env_var("SLACK_CLIENT_ID")?,
        config_env_var("SLACK_CLIENT_SECRET")?,
        config_env_var("SLACK_BOT_SCOPE")?,
        config_env_var("SLACK_REDIRECT_HOST")?,
    ));

    let push_events_config = Arc::new(SlackPushEventsListenerConfig::new(config_env_var(
        "SLACK_SIGNING_SECRET",
    )?));

    let interactions_events_config = Arc::new(SlackInteractionEventsListenerConfig::new(
        config_env_var("SLACK_SIGNING_SECRET")?,
    ));

    let command_events_config = Arc::new(SlackCommandEventsListenerConfig::new(config_env_var(
        "SLACK_SIGNING_SECRET",
    )?));

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

    let server = hyper::server::Server::bind(&addr).serve(make_svc);
    server.await.map_err(|e| {
        error!("Server error: {}", e);
        e.into()
    })
}

#[allow(dead_code)]
async fn test_client_with_socket_mode() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Arc::new(SlackClient::new(SlackClientHyperConnector::new()));

    let socket_mode_callbacks = SlackSocketModeListenerCallbacks::new()
        .with_command_events(test_command_events_function)
        .with_interaction_events(test_interaction_events_function)
        .with_interaction_events(test_interaction_events_function)
        .with_push_events(test_push_events_sm_function);

    let listener_environment = Arc::new(
        SlackClientEventsListenerEnvironment::new(client.clone())
            .with_error_handler(test_error_handler)
            .with_user_state(UserStateExample(0)),
    );

    let socket_mode_listener = SlackClientSocketModeListener::new(
        &SlackClientSocketModeConfig::new(),
        listener_environment.clone(),
        socket_mode_callbacks,
    );

    let app_token_value: SlackApiTokenValue = config_env_var("SLACK_TEST_APP_TOKEN")?.into();
    let app_token: SlackApiToken = SlackApiToken::new(app_token_value);

    socket_mode_listener.start_for(&app_token).await?;

    socket_mode_listener.serve().await;

    Ok(())
}

fn init_log() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use fern::colors::{Color, ColoredLevelConfig};

    let colors_level = ColoredLevelConfig::new()
        .info(Color::Green)
        .warn(Color::Magenta);

    fern::Dispatch::new()
        // Perform allocation-free log formatting
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}{}\x1B[0m",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                colors_level.color(record.level()),
                format_args!(
                    "\x1B[{}m",
                    colors_level.get_color(&record.level()).to_fg_str()
                ),
                message
            ))
        })
        // Add blanket level filter -
        .level(log::LevelFilter::Debug)
        // - and per-module overrides
        .level_for("slack_morphism", log::LevelFilter::Trace)
        .level_for("slack_morphism_hyper", log::LevelFilter::Trace)
        .level_for("hyper", log::LevelFilter::Info)
        .level_for("rustls", log::LevelFilter::Info)
        // Output to stdout, files, and other Dispatch configurations
        .chain(std::io::stdout())
        // Apply globally
        .apply()?;

    Ok(())
}

pub fn config_env_var(name: &str) -> Result<String, String> {
    std::env::var(name).map_err(|e| format!("{}: {}", name, e))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    init_log()?;
    //test_server().await?;

    test_client_with_socket_mode().await?;

    Ok(())
}
