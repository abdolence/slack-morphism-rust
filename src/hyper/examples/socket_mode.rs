use slack_morphism::prelude::*;
use slack_morphism_hyper::*;

use std::sync::Arc;

async fn test_interaction_events_function(
    event: SlackInteractionEvent,
    _client: Arc<SlackHyperClient>,
    _states: Arc<SlackClientEventsUserState>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("{:#?}", event);
    Ok(())
}

async fn test_command_events_function(
    event: SlackCommandEvent,
    client: Arc<SlackHyperClient>,
    _states: Arc<SlackClientEventsUserState>,
) -> Result<SlackCommandEventResponse, Box<dyn std::error::Error + Send + Sync>> {
    println!("{:#?}", event);

    let token_value: SlackApiTokenValue = config_env_var("SLACK_TEST_TOKEN")?.into();
    let token: SlackApiToken = SlackApiToken::new(token_value);

    // Sessions are lightweight and basically just a reference to client and token
    let session = client.open_session(&token);

    session
        .api_test(&SlackApiTestRequest::new().with_foo("Test".into()))
        .await?;

    Ok(SlackCommandEventResponse::new(
        SlackMessageContent::new().with_text("Working on it".into()),
    ))
}

async fn test_push_events_sm_function(
    event: SlackPushEventCallback,
    _client: Arc<SlackHyperClient>,
    _states: Arc<SlackClientEventsUserState>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("{:#?}", event);
    Ok(())
}

fn test_error_handler(
    err: Box<dyn std::error::Error + Send + Sync>,
    _client: Arc<SlackHyperClient>,
    _states: Arc<SlackClientEventsUserState>,
) -> http::StatusCode {
    println!("{:#?}", err);

    // This return value should be OK if we want to return successful ack to the Slack server using Web-sockets
    // https://api.slack.com/apis/connections/socket-implement#acknowledge
    // so that Slack knows whether to retry
    http::StatusCode::OK
}

async fn test_client_with_socket_mode() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Arc::new(SlackClient::new(SlackClientHyperConnector::new()));

    let socket_mode_callbacks = SlackSocketModeListenerCallbacks::new()
        .with_command_events(test_command_events_function)
        .with_interaction_events(test_interaction_events_function)
        .with_push_events(test_push_events_sm_function);

    let listener_environment = Arc::new(
        SlackClientEventsListenerEnvironment::new(client.clone())
            .with_error_handler(test_error_handler),
    );

    let socket_mode_listener = SlackClientSocketModeListener::new(
        &SlackClientSocketModeConfig::new(),
        listener_environment.clone(),
        socket_mode_callbacks,
    );

    let app_token_value: SlackApiTokenValue = config_env_var("SLACK_TEST_APP_TOKEN")?.into();
    let app_token: SlackApiToken = SlackApiToken::new(app_token_value);

    socket_mode_listener.listen_for(&app_token).await?;

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
        .level_for("slack_morphism", log::LevelFilter::Debug)
        .level_for("slack_morphism_hyper", log::LevelFilter::Debug)
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

    test_client_with_socket_mode().await?;

    Ok(())
}
