use rsb_derive::Builder;
use slack_morphism::prelude::*;
use std::sync::Arc;
use url::Url;

// A tiny in-memory catalogue the external select menu below searches through.
const EXAMPLE_ISSUES: &[(&str, &str)] = &[
    ("AI-2323", "Unexpected sentience"),
    ("AI-2324", "The model refuses to answer"),
    ("OPS-101", "Disk pressure on the build host"),
];

fn find_example_issues(query: &str) -> Vec<SlackBlockChoiceItem<SlackBlockPlainTextOnly>> {
    let query = query.to_lowercase();
    EXAMPLE_ISSUES
        .iter()
        .filter(|(id, title)| {
            id.to_lowercase().contains(&query) || title.to_lowercase().contains(&query)
        })
        .map(|(id, title)| {
            SlackBlockChoiceItem::new(pt!(*title), id.to_string())
                .with_description(pt!(format!("Issue {}", id)))
        })
        .collect()
}

async fn test_interaction_events_function(
    event: SlackInteractionEvent,
    _client: Arc<SlackHyperClient>,
    _states: SlackClientEventsUserState,
) -> Result<SlackInteractionResponse, Box<dyn std::error::Error + Send + Sync>> {
    println!("Interaction event: {:#?}", event);

    match event {
        // Slack sends this while a user types into an `external_select` /
        // `multi_external_select` menu. Answer it with up to 100 plain text options.
        SlackInteractionEvent::BlockSuggestion(suggestion_event) => Ok(
            SlackBlockSuggestionResponse::Options(SlackBlockSuggestionOptions::new(
                find_example_issues(&suggestion_event.value),
            ))
            .into(),
        ),
        // Everything else only needs an acknowledgement.
        _ => Ok(SlackInteractionResponse::Empty),
    }
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

    println!("{:#?}", user_info_resp);

    let blocks: Vec<SlackBlock> = slack_blocks![
        SlackSectionBlock::new().with_text(md!(
            "Working section for {}. Team ID: {:?}",
            event.user_id.to_slack_format(),
            user_info_resp.user.team_id
        )),
        SlackActionsBlock::new(slack_blocks![
            SlackBlockButtonElement::new("my-simple-action-button".into(), pt!("Action button"))
                .with_style(SlackBlockButtonStyle::Primary)
                .with_accessibility_label(SlackAccessibilityLabel(
                    "Perform the main action".into()
                )),
            SlackBlockStaticSelectElement::new("my-simple-static-menu".into()).with_options(vec![
                SlackBlockChoiceItem::new(pt!("my-option1"), "my-option1-value".to_string())
            ]),
            SlackBlockExternalSelectElement::new("my-external-select-action".into())
                .with_placeholder(pt!("Start typing to search"))
                .with_min_query_length(1),
        ]),
        SlackCardBlock::new()
            .with_title(md!("Library status"))
            .with_body(md!("slack-morphism is up and running.")),
        SlackContextActionsBlock::new(vec![SlackBlockIconButtonElement::new(
            "delete_card".into(),
            "trash".into(),
            pt!("Delete")
        )
        .with_value("delete_item".into())
        .with_accessibility_label(SlackAccessibilityLabel("Delete this item".into()))
        .into()]),
        SlackTableBlock::new(vec![
            vec!["Name".into(), "Status".into()],
            vec![
                "Slack Morphism".into(),
                SlackTableRichTextCell::new(vec![SlackRichTextSection::new(vec![
                    SlackRichTextText::new("Active".into()).bold().into()
                ])
                .into()])
                .into(),
            ],
        ])
        .with_column_settings(vec![
            SlackTableColumnSetting::new(),
            SlackTableColumnSetting::new().with_align(SlackTableColumnAlign::Right),
        ]),
        SlackTaskCardBlock::new("task_demo".into(), "Checking library status".into())
            .with_status(SlackTaskCardStatus::Complete)
            .with_output(
                SlackRichTextBlock::new(vec![SlackRichTextSection::new(vec![
                    "All systems operational".into()
                ])
                .into()])
                .into()
            )
            .with_sources(vec![SlackUrlSourceElement::new(
                Url::parse("https://slack-rust.abdolence.dev").expect("a hard-coded, valid URL"),
                "slack-morphism docs".into()
            )
            .into()]),
    ];

    Ok(SlackCommandEventResponse::new(
        SlackMessageContent::new()
            .with_text(format!("Working on it: {:?}", user_info_resp.user.team_id))
            .with_blocks(blocks),
    ))
}

async fn test_push_events_sm_function(
    event: SlackPushEventCallback,
    client: Arc<SlackHyperClient>,
    _states: SlackClientEventsUserState,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Push event: {:#?}", event);
    match event.event {
        SlackEventCallbackBody::AppHomeOpened(home_event)
            if home_event.tab == Some("home".to_string()) =>
        {
            let token_value: SlackApiTokenValue = config_env_var("SLACK_TEST_TOKEN")?.into();
            let token: SlackApiToken = SlackApiToken::new(token_value);

            let session = client.open_session(&token);

            let home_tab = SlackHomeTabBlocksTemplateExample::new(
                    vec![
                        SlackHomeNewsItem::new(
                        "Google claimed quantum supremacy in 2019 — and sparked controversy".into(),
                        "In October, researchers from Google claimed to have achieved a milestone known as quantum supremacy. They had created the first quantum computer that could perform a calculation that is impossible for a standard computer.".into(),
                        "2019-12-16T12:00:09Z".parse::<SlackUtcDateTime>().unwrap()),
                        SlackHomeNewsItem::new(
                            "Quantum jitter lets heat travel across a vacuum".into(),
                            "A new experiment shows that quantum fluctuations permit heat to bridge empty space.".into(),
                            "2019-12-16T12:00:09Z".parse::<SlackUtcDateTime>().unwrap())
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
    pub published: SlackUtcDateTime,
}

#[derive(Debug, Clone, Builder)]
pub struct SlackHomeTabBlocksTemplateExample {
    pub latest_news: Vec<SlackHomeNewsItem>,
    pub user_id: SlackUserId,
}

impl SlackBlocksTemplate for SlackHomeTabBlocksTemplateExample {
    fn render_template(&self) -> Vec<SlackBlock> {
        slack_blocks![
            SlackSectionBlock::new()
                .with_text(md!("Home tab for {}", self.user_id.to_slack_format())),
            SlackImageBlock::new(
                Url::parse("https://www.gstatic.com/webp/gallery/4.png")
                    .expect("a hard-coded, valid URL")
                    .into(),
                "Test image".into()
            ),
            SlackSectionBlock::new().with_text(md!("Latest news:")),
            ..self.latest_news.iter().flat_map(|news_item| {
                let news_blocks: [SlackBlock; 2] = [
                    SlackSectionBlock::new()
                        .with_text(md!(" • *{}*\n>{}", news_item.title, news_item.body))
                        .into(),
                    SlackContextBlock::new(vec![md!(
                        "Published: {}",
                        fmt_slack_date(
                            &news_item.published,
                            SlackDateTimeFormats::DatePretty.to_string().as_str(),
                            None
                        )
                    )])
                    .into(),
                ];
                news_blocks
            }),
            SlackDividerBlock::new(),
            SlackRichTextBlock::new(vec![
                SlackRichTextSection::new(vec![
                    SlackRichTextText::new("Let's use some rich text: ".into())
                        .bold()
                        .into(),
                    SlackRichTextEmoji::new("slightly_smiling_face".into()).into(),
                ])
                .into(),
                SlackRichTextSection::new(vec![SlackRichTextDate::new(
                    SlackDateTime::now(),
                    SlackDateTimeFormats::DateLong.to_string()
                )
                .into()])
                .into(),
                SlackRichTextQuote::new(vec![
                    "While there is life, there is a hope. ".into(),
                    SlackRichTextLink::new(
                        Url::parse("https://slack-rust.abdolence.dev")
                            .expect("a hard-coded, valid URL")
                            .into()
                    )
                    .into(),
                ])
                .into(),
                SlackRichTextList::new(
                    SlackRichTextListStyle::Bullet,
                    vec!["Item 1".into(), "Item 2".into()]
                )
                .into(),
                SlackRichTextPreformatted::new(vec!["Let's use some preformatted text: ".into()])
                    .into(),
            ]),
        ]
    }
}

fn test_error_handler(
    err: Box<dyn std::error::Error + Send + Sync>,
    _client: Arc<SlackHyperClient>,
    _states: SlackClientEventsUserState,
) -> HttpStatusCode {
    println!("{:#?}", err);

    // This return value should be OK if we want to return successful ack to the Slack server using Web-sockets
    // https://api.slack.com/apis/connections/socket-implement#acknowledge
    // so that Slack knows whether to retry
    HttpStatusCode::OK
}

async fn test_client_with_socket_mode() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Arc::new(SlackClient::new(SlackClientHyperConnector::new()?));

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
