# Pagination support
Some Web API methods defines cursors and pagination, to give you an ability to load a lot of data continually
(using batching and continually making many requests).

Examples: `conversations.history`, `conversations.list`, `users.list`, ...

To help with those methods Slack Morphism provides additional a “scroller” implementation,
which deal with all scrolling/batching requests for you.

For example for `users.list`:

```rust,noplaypen

use slack_morphism::*;
use slack_morphism::api::*;
use slack_morphism_models::*;
use std::time::Duration;

async fn example() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    let client = SlackClient::new();
    let token_value: SlackApiTokenValue = "xoxb-89.....".into();
    let token: SlackApiToken = SlackApiToken::new(token_value);
    let session = client.open_session(&token);
    
    // Create a first request and specify a batch limit:
    let scroller_req: SlackApiUsersListRequest = SlackApiUsersListRequest::new().with_limit(5);
    
    // Create a scroller from this request
    let scroller = scroller_req.scroller();
    
    // Option 1: Create a Rust Futures Stream from this scroller and use it
    use futures::stream::BoxStream;
    use futures::TryStreamExt;
    
    let mut items_stream = scroller.to_items_stream(&session);
    while let Some(items) = items_stream.try_next().await? {
        println!("users batch: {:#?}", items);
    }
    
    // Option 2: Collect all of the data in a vector (which internally uses the same approach above)
    let collected_members: Vec<SlackUser> = scroller
        .collect_items_stream(&session, Duration::from_millis(1000))
        .await?;
    Ok(())
}

```
