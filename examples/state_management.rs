use slack_morphism::prelude::*;

use std::sync::Arc;

#[derive(Debug, Clone)]
struct UserStateExample(u64);

async fn test_push_events_sm_function(
    _event: SlackPushEventCallback,
    _client: Arc<SlackHyperClient>,
    user_state: SlackClientEventsUserState,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Read state
    let current_state_example: UserStateExample = {
        let storage = user_state.read().await;
        storage
            .get_user_state::<UserStateExample>()
            .unwrap()
            .clone()
    };

    // Write state
    {
        let mut storage = user_state.write().await;
        let updated_state = UserStateExample(current_state_example.0 + 1);
        storage.set_user_state::<UserStateExample>(updated_state);
        println!(
            "Updating user state from {:#?} to {:#?}",
            current_state_example,
            storage.get_user_state::<UserStateExample>()
        );
    }

    Ok(())
}

fn test_error_handler(
    err: Box<dyn std::error::Error + Send + Sync>,
    _client: Arc<SlackHyperClient>,
    _states: SlackClientEventsUserState,
) -> http::StatusCode {
    println!("{:#?}", err);

    // This return value should be OK if we want to return successful ack to the Slack server using Web-sockets
    // https://api.slack.com/apis/connections/socket-implement#acknowledge
    // so that Slack knows whether to retry
    http::StatusCode::OK
}

async fn test_client_with_socket_mode() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Arc::new(SlackClient::new(SlackClientHyperConnector::new()));

    let socket_mode_callbacks =
        SlackSocketModeListenerCallbacks::new().with_push_events(test_push_events_sm_function);

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

    socket_mode_listener.listen_for(&app_token).await?;

    socket_mode_listener.serve().await;

    Ok(())
}

pub fn config_env_var(name: &str) -> Result<String, String> {
    std::env::var(name).map_err(|e| format!("{}: {}", name, e))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter("slack_morphism=debug")
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    test_client_with_socket_mode().await?;

    Ok(())
}
