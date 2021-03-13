# User state propagation in EventListener handlers

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
    user_states_storage: Arc<RwLock<SlackClientEventsUserStateStorage>>,
) {
    let states = user_states_storage.read().unwrap();
    let user_state: Option<&UserStateExample> = 
        states.get_user_state::<UserStateExample>();
}
```

## Updating user state in listeners 

```rust,noplaypen
async fn test_push_events_function(
    event: SlackPushEvent,
    client: Arc<SlackHyperClient>,
    user_states_storage: Arc<RwLock<SlackClientEventsUserStateStorage>>,
) {
    let states = user_states_storage.write().unwrap();
    states.set_user_state(UserStateExample(555));
}
```