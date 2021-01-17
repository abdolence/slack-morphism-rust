# Block Kit support

To support Slack Block Kit rich messages and views, the library provides:

- Well-typed models
- Macros to help build blocks, block elements

Let’s take some very simple block example:

```json
{
  "blocks": [
      {
        "type": "section",
        "text": {
            "type": "mrkdwn",
            "text": "A message *with some bold text* and _some italicized text_."
        }
      }
  ]
}
```

Now, let’s look at how it looks with type-safe code using Slack Morphism Blocks macro support:

```rust,noplaypen
use slack_morphism_models::*;
use slack_morphism_models::blocks::*;

let blocks : Vec<SlackBlock> = slack_blocks![
 some_into(
    SlackSectionBlock::new()
        .with_text(md!("A message *with some bold text* and _some italicized text_."))
 )
];
```

Let’s look at another more complex example for welcoming message:

```rust,noplaypen

use slack_morphism::*;
use slack_morphism::api::*;
use slack_morphism_models::*;
use slack_morphism_models::blocks::*;

use slack_morphism_hyper::*;

async fn example() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    use std::time::Duration;
    use rsb_derive::Builder;

    let hyper_connector = SlackClientHyperConnector::new();
    let client = SlackClient::new(hyper_connector);

    let token_value: SlackApiTokenValue = "xoxb-89.....".into();
    let token: SlackApiToken = SlackApiToken::new(token_value);
    let session = client.open_session(&token);

    // Our welcome message params as a struct
    #[derive(Debug, Clone, Builder)]
    pub struct WelcomeMessageTemplateParams {
        pub user_id: SlackUserId,
    }

    // Define our welcome message template using block macros, a trait and models from the library
    impl SlackMessageTemplate for WelcomeMessageTemplateParams {
        fn render_template(&self) -> SlackMessageContent {
            SlackMessageContent::new()
                .with_text(format!("Hey {}", self.user_id.to_slack_format()))
                .with_blocks(slack_blocks![
                    some_into(
                        SlackSectionBlock::new()
                            .with_text(md!("Hey {}", self.user_id.to_slack_format()))
                    ),
                    some_into(SlackDividerBlock::new()),
                    some_into(SlackImageBlock::new(
                        "https://www.gstatic.com/webp/gallery3/2_webp_ll.png".into(),
                        "Test Image".into()
                    )),
                    some_into(SlackActionsBlock::new(slack_blocks![some_into(
                        SlackBlockButtonElement::new(
                            "simple-message-button".into(),
                            pt!("Simple button text")
                        )
                    )]))
                ])
        }
    }

    // Use it
    let message = WelcomeMessageTemplateParams::new("some-slack-uid".into());

    let post_chat_req =
        SlackApiChatPostMessageRequest::new("#general".into(), message.render_template());

    Ok(())
}
```

Look other examples in examples/templates.rs.
