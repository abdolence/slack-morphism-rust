use chrono::prelude::*;
use rsb_derive::Builder;
use slack_morphism::prelude::*;
use std::sync::Arc;

async fn test_interaction_events_function(
    event: SlackInteractionEvent,
    _client: Arc<SlackHyperClient>,
    _states: SlackClientEventsUserState,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Interaction event: {:#?}", event);
    Ok(())
}

async fn test_command_events_function(
    event: SlackCommandEvent,
    client: Arc<SlackHyperClient>,
    _states: SlackClientEventsUserState,
) -> Result<SlackCommandEventResponse, Box<dyn std::error::Error + Send + Sync>> {
    println!("{:#?}", event);

    let token_value: SlackApiTokenValue = config_env_var("SLACK_TEST_TOKEN")?.into();
    let token: SlackApiToken = SlackApiToken::new(token_value);

    // Sessions are lightweight and basically just a reference to client and token
    let session = client.open_session(&token);

    session
        .api_test(&SlackApiTestRequest::new().with_foo("Test".into()))
        .await?;

    let user_info_resp = session
        .users_info(&SlackApiUsersInfoRequest::new(event.user_id.clone()))
        .await?;

    Ok(SlackCommandEventResponse::new(
        SlackMessageContent::new()
            .with_text(format!("Working on it: {:?}", user_info_resp.user.team_id).into())
            .with_blocks(slack_blocks![
                some_into(SlackSectionBlock::new().with_text(md!(
                    "Working section for {}. Team ID: {:?}",
                    event.user_id.to_slack_format(),
                    user_info_resp.user.teams
                ))),
                some_into(SlackActionsBlock::new(slack_blocks![
                    some_into(SlackBlockButtonElement::new(
                        "my-simple-action-button".into(),
                        pt!("Action button")
                    )),
                    some_into(
                        SlackBlockStaticSelectElement::new("my-simple-static-menu".into())
                            .with_options(vec![SlackBlockChoiceItem::new(
                                pt!("my-option1"),
                                "my-option1-value".to_string()
                            )])
                    )
                ]))
            ]),
    ))
}

async fn test_push_events_sm_function(
    event: SlackPushEventCallback,
    client: Arc<SlackHyperClient>,
    _states: SlackClientEventsUserState,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Push event: {:#?}", event);
    match event.event {
        SlackEventCallbackBody::AppHomeOpened(home_event) if home_event.tab == "home" => {
            let token_value: SlackApiTokenValue = config_env_var("SLACK_TEST_TOKEN")?.into();
            let token: SlackApiToken = SlackApiToken::new(token_value);

            let session = client.open_session(&token);

            let home_tab = SlackHomeTabBlocksTemplateExample::new(
                    vec![
                        SlackHomeNewsItem::new(
                        "Google claimed quantum supremacy in 2019 — and sparked controversy".into(),
                        "In October, researchers from Google claimed to have achieved a milestone known as quantum supremacy. They had created the first quantum computer that could perform a calculation that is impossible for a standard computer.".into(),
                        DateTime::parse_from_rfc3339("2019-12-16T12:00:09Z").unwrap().into()),
                        SlackHomeNewsItem::new(
                            "Quantum jitter lets heat travel across a vacuum".into(),
                            "A new experiment shows that quantum fluctuations permit heat to bridge empty space.".into(),
                            DateTime::parse_from_rfc3339("2019-12-16T12:00:09Z").unwrap().into())
                    ],
                    home_event.user.clone(),
                );
            session
                .views_publish(&SlackApiViewsPublishRequest::new(
                    home_event.user,
                    SlackView::Home(SlackHomeView::new(home_tab.render_template())),
                ))
                .await?;
            Ok(())
        }
        _ => Ok(()),
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
        let new_blocks: Vec<SlackBlock> = self
            .latest_news
            .clone()
            .into_iter()
            .map(|news_item| {
                vec![
                    SlackSectionBlock::new()
                        .with_text(md!(" • *{}*\n>{}", news_item.title, news_item.body))
                        .into(),
                    SlackContextBlock::new(slack_blocks![some(md!(
                        "Published: {}",
                        fmt_slack_date(
                            &news_item.published,
                            SlackDateTimeFormats::DatePretty.to_string().as_str(),
                            None
                        )
                    ))])
                    .into(),
                ]
            })
            .flatten()
            .collect();

        [
            slack_blocks![
                some_into(
                    SlackSectionBlock::new()
                        .with_text(md!("Home tab for {}", self.user_id.to_slack_format()))
                ),
                some_into(SlackImageBlock::new(
                    "https://www.gstatic.com/webp/gallery/4.png"
                        .try_into()
                        .unwrap(),
                    "Test image".into()
                )),
                some_into(SlackSectionBlock::new().with_text(md!("Latest news:")))
            ],
            new_blocks,
        ]
        .concat()
    }
}

fn test_error_handler(
    err: Box<dyn std::error::Error + Send + Sync>,
    _client: Arc<SlackHyperClient>,
    _states: SlackClientEventsUserState,
) -> http::StatusCode {
    println!("{:#?}", err);

    // This return value should be OK if we want to return successful ack to the Slack server using Web-sockets
    // https://api.slack.com/apis/connections/socket-implement#acknowledge
    // so that Slack knows whether to retry
    http::StatusCode::OK
}

async fn test_client_with_socket_mode() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Arc::new(SlackClient::new(SlackClientHyperConnector::new()));

    let socket_mode_callbacks = SlackSocketModeListenerCallbacks::new()
        .with_command_events(test_command_events_function)
        .with_interaction_events(test_interaction_events_function)
        .with_push_events(test_push_events_sm_function);

    let listener_environment = Arc::new(
        SlackClientEventsListenerEnvironment::new(client.clone())
            .with_error_handler(test_error_handler),
    );

    let socket_mode_listener = SlackClientSocketModeListener::new(
        &SlackClientSocketModeConfig::new(),
        listener_environment.clone(),
        socket_mode_callbacks,
    );

    let app_token_value: SlackApiTokenValue = config_env_var("SLACK_TEST_APP_TOKEN")?.into();
    let app_token: SlackApiToken = SlackApiToken::new(app_token_value);

    socket_mode_listener.listen_for(&app_token).await?;

    socket_mode_listener.serve().await;

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

    test_client_with_socket_mode().await?;

    Ok(())
}
