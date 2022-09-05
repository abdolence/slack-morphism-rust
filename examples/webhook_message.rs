use slack_morphism::prelude::*;
use url::Url;

async fn test_webhook_message() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = SlackClient::new(SlackClientHyperConnector::new());
    let webhook_url: Url = Url::parse(config_env_var("SLACK_TEST_WEBHOOK_URL")?.as_str())?;

    client
        .post_webhook_message(
            &webhook_url,
            &SlackApiPostWebhookMessageRequest::new(
                SlackMessageContent::new().with_text("Hey".to_string()),
            ),
        )
        .await?;

    Ok(())
}

pub fn config_env_var(name: &str) -> Result<String, String> {
    std::env::var(name).map_err(|e| format!("{}: {}", name, e))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter("slack_morphism=debug")
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    test_webhook_message().await?;

    Ok(())
}
