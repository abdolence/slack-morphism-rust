use chrono::prelude::*;
use slack_morphism::prelude::*;
use slack_morphism_hyper::*;

use rsb_derive::Builder;

use std::time::Duration;

use futures::stream::BoxStream;
use futures::TryStreamExt;

async fn test_client() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = SlackClient::new(SlackClientHyperConnector::new());
    let token_value: SlackApiTokenValue = config_env_var("SLACK_TEST_TOKEN")?.into();
    let token: SlackApiToken = SlackApiToken::new(token_value);

    // Sessions are lightweight and basically just a reference to client and token
    let session = client.open_session(&token);
    println!("{:#?}", session);

    let test: SlackApiTestResponse = session
        .api_test(&SlackApiTestRequest::new().with_foo("Test".into()))
        .await?;

    println!("{:#?}", test);

    let message = WelcomeMessageTemplateParams::new("".into());

    let post_chat_req =
        SlackApiChatPostMessageRequest::new("#general".into(), message.render_template());

    let post_chat_resp = session.chat_post_message(&post_chat_req).await?;
    println!("post chat resp: {:#?}", &post_chat_resp);

    let scroller_req: SlackApiUsersListRequest = SlackApiUsersListRequest::new().with_limit(1);
    let scroller = scroller_req.scroller();

    let mut resp_stream: BoxStream<ClientResult<SlackApiUsersListResponse>> =
        scroller.to_stream(&session);

    while let Some(item) = resp_stream.try_next().await? {
        println!("res: {:#?}", item.members);
    }

    let collected_members: Vec<SlackUser> = scroller
        .collect_items_stream(&session, Duration::from_millis(1000))
        .await?;
    println!("collected res: {:#?}", collected_members);

    let mut items_stream = scroller.to_items_stream(&session);
    while let Some(items) = items_stream.try_next().await? {
        println!("res: {:#?}", items);
    }

    let mut items_throttled_stream =
        scroller.to_items_throttled_stream(&session, Duration::from_millis(500));
    while let Some(items) = items_throttled_stream.try_next().await? {
        println!("res: {:#?}", items);
    }

    Ok(())
}

#[derive(Debug, Clone, Builder)]
pub struct WelcomeMessageTemplateParams {
    pub user_id: SlackUserId,
}

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
                some_into(SlackHeaderBlock::new(pt!("Simple header"))),
                some_into(SlackDividerBlock::new()),
                some_into(SlackContextBlock::new(slack_blocks![
                    some(md!("This is an example of block message")),
                    some(md!(
                        "Current time is: {}",
                        fmt_slack_date(
                            &Local::now(),
                            SlackDateTimeFormats::DatePretty.to_string().as_str(),
                            None
                        )
                    ))
                ])),
                some_into(SlackDividerBlock::new()),
                some_into(
                    SlackImageBlock::new(
                        "https://www.gstatic.com/webp/gallery3/2_webp_ll.png".into(),
                        "Test Image".into(),
                    )
                    .with_title("Test Image".into())
                ),
                some_into(SlackActionsBlock::new(slack_blocks![some_into(
                    SlackBlockButtonElement::new(
                        "simple-message-button".into(),
                        pt!("Simple button text")
                    )
                )]))
            ])
    }
}

#[derive(Debug, Clone, Builder)]
pub struct SlackHomeNewsItem {
    pub title: String,
    pub body: String,
    pub published: DateTime<Utc>,
}

#[derive(Debug, Clone, Builder)]
pub struct SlackHomeTabBlocksTemplateExample {
    pub latest_news: Vec<SlackHomeNewsItem>,
    pub user_id: SlackUserId,
}

impl SlackBlocksTemplate for SlackHomeTabBlocksTemplateExample {
    fn render_template(&self) -> Vec<SlackBlock> {
        slack_blocks![
            some_into(
                SlackSectionBlock::new()
                    .with_text(md!("Home tab for {}", self.user_id.to_slack_format()))
            ),
            some_into(SlackContextBlock::new(slack_blocks![
                some(md!("This is an example of home tab")),
                some(md!(
                    "Current time is: {}",
                    fmt_slack_date(
                        &Local::now(),
                        SlackDateTimeFormats::DatePretty.to_string().as_str(),
                        None
                    )
                ))
            ]))
        ]
    }
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
        // Output to stdout, files, and other Dispatch configurations
        .chain(std::io::stdout())
        // Apply globally
        .apply()?;

    Ok(())
}

pub fn config_env_var(name: &str) -> Result<String, String> {
    std::env::var(name).map_err(|e| format!("{}: {}", name, e))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    init_log()?;

    test_client().await?;

    Ok(())
}
