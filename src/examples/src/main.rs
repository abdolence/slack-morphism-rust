use slack_morphism::api::*;
use slack_morphism::listener::*;
use slack_morphism::*;
use slack_morphism_models::*;

use futures::stream::BoxStream;
use futures::TryStreamExt;
use std::time::Duration;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response};
use log::*;

use std::sync::Arc;

mod templates;
use templates::*;

#[allow(dead_code)]
async fn test_client() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = SlackClient::new();
    let token_value: SlackApiTokenValue = std::env::var("SLACK_TEST_TOKEN")?.into();
    let token: SlackApiToken = SlackApiToken::new(token_value);
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

    Ok(())
}

async fn test_oauth_install_function(
    resp: Result<SlackOAuthV2AccessTokenResponse, Box<dyn std::error::Error + Send + Sync>>,
    _client: Arc<SlackClient>,
) {
    println!("{:#?}", resp);
}

async fn test_push_events_function(
    resp: Result<SlackPushEvent, Box<dyn std::error::Error + Send + Sync>>,
    _client: Arc<SlackClient>,
) {
    println!("{:#?}", resp);
}

async fn test_interaction_events_function(
    resp: Result<SlackInteractionEvent, Box<dyn std::error::Error + Send + Sync>>,
    _client: Arc<SlackClient>,
) {
    println!("{:#?}", resp);
}

async fn test_command_events_function(
    resp: Result<SlackCommandEvent, Box<dyn std::error::Error + Send + Sync>>,
    _client: Arc<SlackClient>,
) -> Result<SlackCommandEventResponse, Box<dyn std::error::Error + Send + Sync>> {
    resp.map(|event| {
        println!("{:#?}", event);
        SlackCommandEventResponse::new(SlackMessageContent::new().with_text("Working on it".into()))
    })
}

async fn test_server(
    client: Arc<SlackClient>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8080));
    info!("Loading server: {}", addr);

    async fn your_others_routes(
        _req: Request<Body>,
    ) -> Result<Response<Body>, Box<dyn std::error::Error + Send + Sync>> {
        Response::builder()
            .body("Hey, this is a default users route handler".into())
            .map_err(|e| e.into())
    }

    let oauth_listener_config = Arc::new(SlackOAuthListenerConfig::new(
        std::env::var("SLACK_CLIENT_ID")?,
        std::env::var("SLACK_CLIENT_SECRET")?,
        std::env::var("SLACK_BOT_SCOPE")?,
        std::env::var("SLACK_REDIRECT_HOST")?,
    ));

    let push_events_config = Arc::new(SlackPushEventsListenerConfig::new(std::env::var(
        "SLACK_SIGNING_SECRET",
    )?));

    let interactions_events_config = Arc::new(SlackInteractionEventsListenerConfig::new(
        std::env::var("SLACK_SIGNING_SECRET")?,
    ));

    let command_events_config = Arc::new(SlackCommandEventsListenerConfig::new(std::env::var(
        "SLACK_SIGNING_SECRET",
    )?));

    let make_svc = make_service_fn(move |_| {
        let thread_oauth_config = oauth_listener_config.clone();
        let thread_push_events_config = push_events_config.clone();
        let thread_interaction_events_config = interactions_events_config.clone();
        let thread_command_events_config = command_events_config.clone();
        let listener = SlackClientEventsListener::new(client.clone());
        async move {
            let routes = chain_service_routes_fn(
                listener.oauth_service_fn(thread_oauth_config, test_oauth_install_function),
                chain_service_routes_fn(
                    listener.push_events_service_fn(
                        thread_push_events_config,
                        test_push_events_function,
                    ),
                    chain_service_routes_fn(
                        listener.interaction_events_service_fn(
                            thread_interaction_events_config,
                            test_interaction_events_function,
                        ),
                        chain_service_routes_fn(
                            listener.command_events_service_fn(
                                thread_command_events_config,
                                test_command_events_function,
                            ),
                            your_others_routes,
                        ),
                    ),
                ),
            );

            Ok::<_, Box<dyn std::error::Error + Send + Sync>>(service_fn(routes))
        }
    });

    let server = hyper::server::Server::bind(&addr).serve(make_svc);
    server.await.map_err(|e| {
        error!("Server error: {}", e);
        e.into()
    })
}

fn init_log() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use fern::colors::{Color, ColoredLevelConfig};

    let colors_level = ColoredLevelConfig::new()
        .info(Color::Green)
        .warn(Color::Magenta);

    let colors_line = colors_level.clone();

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
                    colors_line.get_color(&record.level()).to_fg_str()
                ),
                message
            ))
        })
        // Add blanket level filter -
        .level(log::LevelFilter::Debug)
        // - and per-module overrides
        .level_for("hyper", log::LevelFilter::Info)
        // Output to stdout, files, and other Dispatch configurations
        .chain(std::io::stdout())
        // Apply globally
        .apply()?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    init_log()?;
    let client: Arc<SlackClient> = Arc::new(SlackClient::new());
    test_server(client.clone()).await?;

    Ok(())
}
