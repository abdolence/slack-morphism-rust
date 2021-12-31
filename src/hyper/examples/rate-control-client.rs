use log::debug;
use slack_morphism::prelude::*;
use slack_morphism_hyper::*;

async fn test_rate_control_client() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = SlackClient::new(
        SlackClientHyperConnector::new().with_rate_control(SlackApiRateControlConfig::new()),
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

fn init_log() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use fern::colors::{Color, ColoredLevelConfig};

    let colors_level = ColoredLevelConfig::new()
        .info(Color::Green)
        .warn(Color::Magenta);

    fern::Dispatch::new()
        // Perform allocation-free log formatting
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}{}\x1B[0m",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                colors_level.color(record.level()),
                format_args!(
                    "\x1B[{}m",
                    colors_level.get_color(&record.level()).to_fg_str()
                ),
                message
            ))
        })
        // Add blanket level filter -
        .level(log::LevelFilter::Debug)
        // - and per-module overrides
        .level_for("slack_morphism", log::LevelFilter::Debug)
        .level_for("slack_morphism_hyper", log::LevelFilter::Debug)
        .level_for("hyper", log::LevelFilter::Info)
        .level_for("rustls", log::LevelFilter::Info)
        .level_for("hyper_rustls", log::LevelFilter::Info)
        // Output to stdout, files, and other Dispatch configurations
        .chain(std::io::stdout())
        // Apply globally
        .apply()?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    init_log()?;
    test_rate_control_client().await?;

    Ok(())
}
