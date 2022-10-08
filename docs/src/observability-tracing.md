# Observability and tracing

The library uses popular `tracing` crate for logs and distributed traces (spans).
To improve observability for your specific cases, additionally to the fields provided by library,  you can inject your own trace fields:

```rust,noplaypen
use slack_morphism::prelude::*;
use tracing::*;

// While Team ID is optional but still useful for tracing and rate control purposes
let token: SlackApiToken =
    SlackApiToken::new(token_value).with_team_id(config_env_var("SLACK_TEST_TEAM_ID")?.into());

// Sessions are lightweight and basically just a reference to client and token
let my_custom_span = span!(Level::DEBUG, "My scope", my_scope_attr = "my-scope-value");
debug!("Testing tracing abilities");

client
    .run_in_session(&token, |session| async move {
        let test: SlackApiTestResponse = session
            .api_test(&SlackApiTestRequest::new().with_foo("Test".into()))
            .await?;
        println!("{:#?}", test);

        let auth_test = session.auth_test().await?;
        println!("{:#?}", auth_test);

        Ok(())
    })
    .instrument(my_custom_span.or_current())
    .await
```
