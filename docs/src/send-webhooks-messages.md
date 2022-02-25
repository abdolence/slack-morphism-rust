# Send Slack Webhook Messages

You can use `client..post_webhook_message` to post [Slack Incoming Webhook](https://api.slack.com/messaging/webhooks) messages:

```rust,noplaypen

use slack_morphism::prelude::*;
use slack_morphism_hyper::*;
use url::Url;

let client = SlackClient::new(SlackClientHyperConnector::new());

// Your incoming webhook url from config or OAuth/events (ResponseURL)
let webhook_url: Url = Url::parse("https://hooks.slack.com/services/...")?; 

client
    .post_webhook_message(
        &webhook_url,
        &SlackApiPostWebhookMessageRequest::new(
            SlackMessageContent::new()
                .with_text(format!("Hey")),
        ),
    )
    .await?;
    
```

