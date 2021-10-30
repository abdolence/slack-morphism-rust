# Slack Socket Mode support

Slack Morphism supports [Slack Socket Mode](https://api.slack.com/apis/connections/socket) starting with 0.10.x.
Socket Mode allows your app to use the Events API and interactive components 
without exposing a public HTTP endpoint.

The mode is useful if you want to create an app that works with few workspaces 
and don't want to work with HTTP endpoints yourself. 

## Register your event callback functions

```rust,noplaypen
use slack_morphism::prelude::*;
use slack_morphism_hyper::*;


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

let client = Arc::new(SlackClient::new(SlackClientHyperConnector::new()));

let socket_mode_callbacks = SlackSocketModeListenerCallbacks::new()
    .with_command_events(test_command_events_function)
    .with_interaction_events(test_interaction_events_function)
    .with_interaction_events(test_interaction_events_function)
    .with_push_events(test_push_events_sm_function);   

let listener_environment = Arc::new(
        SlackClientEventsListenerEnvironment::new(client.clone())
);

```

## Connect using socket mode to Slack

The following code initiates Web-sockets based connections to Slack endpoints using Slack Web methods 
and provided user token.

Slack Morphism supports multiple web-socket connections per one token to [gracefully handle disconnects](https://api.slack.com/apis/connections/socket-implement#disconnect).
By default it uses 2 connections to one token. To configure it see `SlackClientSocketModeConfig`;

```rust,noplaypen

let socket_mode_listener = SlackClientSocketModeListener::new(
      &SlackClientSocketModeConfig::new(),
      listener_environment.clone(),
      socket_mode_callbacks,
);

// Need to specify App token for Socket Mode:
let app_token_value: SlackApiTokenValue = config_env_var("SLACK_TEST_APP_TOKEN")?.into();
let app_token: SlackApiToken = SlackApiToken::new(app_token_value);

// Start connections for the specified token value
socket_mode_listener.start_for(&app_token).await?;

// Wait for Ctrl-C
// There is also .shutdown() available to manually manage the lifecycle of socket_mode_listener
socket_mode_listener.serve().await;

```
