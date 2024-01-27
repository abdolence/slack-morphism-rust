# Slack Socket Mode support

Slack Morphism supports [Slack Socket Mode](https://api.slack.com/apis/connections/socket) starting with 0.10.x.
Socket Mode allows your app to use the Events API and interactive components 
without exposing a public HTTP endpoint.

The mode is useful if you want to create an app that works with few workspaces 
and don't want to work with HTTP endpoints yourself. 

## Register your event callback functions

```rust,noplaypen
use slack_morphism::prelude::*;

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
    println!("{:#?}", event);
    Ok(SlackCommandEventResponse::new(
        SlackMessageContent::new().with_text("Working on it".into()),
    ))
}

async fn test_push_events_sm_function(
    event: SlackPushEventCallback,
    _client: Arc<SlackHyperClient>,
    _states: SlackClientEventsUserState,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("{:#?}", event);
    Ok(())
}

let client = Arc::new(SlackClient::new(SlackClientHyperConnector::new()?));

let socket_mode_callbacks = SlackSocketModeListenerCallbacks::new()
    .with_command_events(test_command_events_function)
    .with_interaction_events(test_interaction_events_function)
    .with_push_events(test_push_events_sm_function);   

let listener_environment = Arc::new(
        SlackClientEventsListenerEnvironment::new(client.clone())
);

let socket_mode_listener = SlackClientSocketModeListener::new(
      &SlackClientSocketModeConfig::new(),
      listener_environment.clone(),
      socket_mode_callbacks,
);

```

## Connect using socket mode to Slack

The following code initiates Web-sockets based connections to Slack endpoints using Slack Web methods 
and provided user token.

Slack Morphism supports multiple web-socket connections per one token to [gracefully handle disconnects](https://api.slack.com/apis/connections/socket-implement#disconnect).
By default it uses 2 connections to one token. To configure it see `SlackClientSocketModeConfig`;

```rust,noplaypen

// Need to specify App token for Socket Mode:
let app_token_value: SlackApiTokenValue = 
    config_env_var("SLACK_TEST_APP_TOKEN")?.into();
let app_token: SlackApiToken = SlackApiToken::new(app_token_value);

// Register an app token to listen for events, 
socket_mode_listener.listen_for(&app_token).await?;

// Start WS connections calling Slack API to get WS url for the token, 
// and wait for Ctrl-C to shutdown
// There are also `.start()`/`.shutdown()` available to manage manually 
socket_mode_listener.serve().await;

```

## Important caveats

### The time blocking of the SM listener callbacks is important

If your app blocks callbacks more than 2-3 seconds Slack server may decide to repeat requests again 
and also to inform users with errors and timeouts. 
So, if you have something complex and time-consuming in your callbacks 
you should spawn your own future, e.g:

```rust,noplaypen
    async fn test_push_events_sm_function(
        event: SlackPushEventCallback,
        _client: Arc<SlackHyperClient>,
        _states: SlackClientEventsUserState,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        tokio::spawn(async move { process_message(client, event).await; });
        Ok(())
    }
```

### Error handling function
It is highly recommended implementing your own error handling function:

```rust,noplaypen

fn test_error_handler(
    err: Box<dyn std::error::Error + Send + Sync>,
    _client: Arc<SlackHyperClient>,
    _states: SlackClientEventsUserState,
) -> http::StatusCode {
    println!("{:#?}", err);

    // This return value should be OK if we want to return successful ack
    // to the Slack server using Web-sockets
    // https://api.slack.com/apis/connections/socket-implement#acknowledge
    // so that Slack knows whether to retry
    http::StatusCode::OK
}

// Register it:
    let listener_environment = Arc::new(
        SlackClientEventsListenerEnvironment::new(client.clone())
            .with_error_handler(test_error_handler),
    );
```

The implementation allows you:
- Return positive ack using http::StatusCode result / implement complex logic related to it.
  https://api.slack.com/apis/connections/socket-implement#acknowledge
- Increase visibility and observability in general when errors happen in your app and from Slack/library.
