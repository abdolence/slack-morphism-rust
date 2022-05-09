# User state propagation for event listeners and callback functions

It is very common to have some user specific context and state in event handler functions.
So, all listener handlers has access to it using `SlackClientEventsUserStateStorage`.

## Defining user state
```rust,noplaypen

// Defining your state as a struct
struct UserStateExample(u64);

// Initializing it in listener environment:
let listener_environment = Arc::new(
    SlackClientEventsListenerEnvironment::new(client.clone())
        .with_error_handler(test_error_handler)
        .with_user_state(UserStateExample(555)),
); 

```

## Reading user state in listeners

```rust,noplaypen
async fn test_push_events_function(
    event: SlackPushEvent,
    client: Arc<SlackHyperClient>,
    user_state_storage: SlackClientEventsUserState,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    let states = user_state_storage.read().await;

    let user_state: Option<&UserStateExample> = 
        states.get_user_state::<UserStateExample>();

    Ok(())
}
```

## Updating user state in listeners 

```rust,noplaypen
async fn test_push_events_function(
    event: SlackPushEvent,
    client: Arc<SlackHyperClient>,
    user_state_storage: SlackClientEventsUserState,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    let states = user_state_storage.write().await;

    states.set_user_state(UserStateExample(555));

    Ok(())
}
```
