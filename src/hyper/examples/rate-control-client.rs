use slack_morphism::prelude::*;
use slack_morphism_hyper::*;

async fn test_rate_control_client() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let _client = SlackClient::new(
        SlackClientHyperConnector::new().with_rate_control(SlackApiRateControlConfig::new()),
    );

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    test_proxy_client().await?;

    Ok(())
}
