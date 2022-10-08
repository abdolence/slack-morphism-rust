use slack_morphism::prelude::*;
use tracing::*;

async fn test_simple_api_calls_as_predicate() -> Result<(), Box<dyn std::error::Error + Send + Sync>>
{
    let client = SlackClient::new(SlackClientHyperConnector::new());
    let token_value: SlackApiTokenValue = config_env_var("SLACK_TEST_TOKEN")?.into();
    let token: SlackApiToken =
        SlackApiToken::new(token_value).with_team_id(config_env_var("SLACK_TEST_TEAM_ID")?.into()); // While Team ID is optional but still useful for tracing and rate control purposes

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
}

pub fn config_env_var(name: &str) -> Result<String, String> {
    std::env::var(name).map_err(|e| format!("{}: {}", name, e))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter("client_with_tracing=debug,slack_morphism=debug")
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    test_simple_api_calls_as_predicate().await?;

    Ok(())
}
