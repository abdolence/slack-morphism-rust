# Slack Web API client

## Create a client instance

```rust,noplaypen
use slack_morphism::prelude::*;

let client = SlackClient::new( SlackClientHyperConnector::new() );

```

### Make Web API methods calls

For most of Slack Web API methods (except for OAuth methods, Incoming Webhooks and event replies)
you need a Slack token to make a call.
For simple bots you can have it in your config files, or you can obtain
workspace tokens using Slack OAuth.

In the example below, we’re using a hardcoded Slack token, but don’t do that for your production bots and apps.
You should securely and properly store all of Slack tokens.

```rust,noplaypen

use slack_morphism::prelude::*;

async fn example() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
   
    let client = SlackClient::new(SlackClientHyperConnector::new());
    
    // Create our Slack API token
    let token_value: SlackApiTokenValue = "xoxb-89.....".into();
    let token: SlackApiToken = SlackApiToken::new(token_value);
    
    // Create a Slack session with this token
    // A session is just a lightweight wrapper around your token
    // not to specify it all the time for series of calls.
    let session = client.open_session(&token);
    
    // Make your first API call (which is `api.test` here)
    let test: SlackApiTestResponse = session
            .api_test(&SlackApiTestRequest::new().with_foo("Test".into()))
            .await?;

    // Send a simple text message
    let post_chat_req =
        SlackApiChatPostMessageRequest::new("#general".into(),
               SlackMessageContent::new().with_text("Hey there!".into())
        );

    let post_chat_resp = session.chat_post_message(&post_chat_req).await?;

    Ok(())
}
```
