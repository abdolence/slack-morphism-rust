use slack_morphism::prelude::*;
use tracing::*;

async fn test_rate_control_client() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = SlackClient::new(
        SlackClientHyperConnector::new()
            .with_rate_control(SlackApiRateControlConfig::new().with_max_retries(5)),
    );

    let token_value: SlackApiTokenValue = config_env_var("SLACK_TEST_TOKEN")?.into();
    let team_id: SlackTeamId = config_env_var("SLACK_TEST_TEAM_ID")?.into();
    let token: SlackApiToken = SlackApiToken::new(token_value).with_team_id(team_id);

    let session = client.open_session(&token);
    println!("{:#?}", session);

    for tries in 0..100 {
        let test = session.team_info(&SlackApiTeamInfoRequest::new()).await?;
        debug!("Tried: {}, {:?}", tries, test);
    }

    Ok(())
}

pub fn config_env_var(name: &str) -> Result<String, String> {
    std::env::var(name).map_err(|e| format!("{}: {}", name, e))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter("slack_morphism_hyper=debug,slack_morphism=debug")
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    test_rate_control_client().await?;

    Ok(())
}
