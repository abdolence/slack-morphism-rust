use slack_morphism_client::api::test::*;
use slack_morphism_client::api::users::*;
use slack_morphism_client::scroller::*;
use slack_morphism_client::*;

use futures::stream::BoxStream;
use futures::TryStreamExt;
use slack_morphism_models::blocks::kit::*;
use slack_morphism_models::common::*;
use slack_morphism_models::*;
use std::time::Duration;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response};
use log::*;
use slack_morphism_client::api::oauth::SlackOAuthV2AccessTokenResponse;
use slack_morphism_client::listener::oauth::*;
use slack_morphism_client::listener::push_events::*;
use slack_morphism_client::listener::*;
use slack_morphism_models::events::SlackPushEvent;
use std::sync::Arc;

#[allow(dead_code)]
async fn test_client() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let sb: SlackSectionBlock = SlackSectionBlock::new().with_block_id("test".into());
    let sb_ser = serde_json::to_string_pretty(&sb).unwrap();
    let sb_des: SlackSectionBlock = serde_json::from_str(&sb_ser).unwrap();
    println!("{} {:?}", sb_ser, sb_des);

    let section_block = SlackSectionBlock::new()
        .with_text(md!("hey, {}", 10))
        .with_fields(slack_items![
            some(md!("hey1")),
            some(pt!("hey2")),
            optionally( sb_ser.is_empty() => md!("hey"))
        ])
        .with_accessory(
            SlackBlockButtonElement::from(SlackBlockButtonElementInit {
                action_id: "-".into(),
                text: pt!("ddd"),
            })
            .into(),
        );

    let context_block: SlackContextBlock = SlackContextBlock::new(slack_blocks![
        some(SlackBlockImageElement::new(
            "http://example.net/img1".into(),
            "text 1".into()
        )),
        some(SlackBlockImageElement::new(
            "http://example.net/img2".into(),
            "text 2".into()
        ))
    ]);

    let blocks: Vec<SlackBlock> = slack_blocks![
       some ( section_block ),
       optionally( !sb_ser.is_empty() => context_block)
    ];

    println!("{:#?}", blocks);

    let client = SlackClient::new();
    let token_value: String = std::env::var("SLACK_TEST_TOKEN")?;
    let token: SlackApiToken = SlackApiToken::new(token_value);
    let session = client.open_session(&token);
    println!("{:#?}", session);

    let test: SlackApiTestResponse = session
        .api_test(&SlackApiTestRequest::new().with_foo("Test".into()))
        .await?;

    println!("{:#?}", test);

    let scroller_req: SlackApiUsersListRequest = SlackApiUsersListRequest::new().with_limit(1);
    let scroller_stream = scroller_req.scroller();

    let mut resp_stream: BoxStream<ClientResult<SlackApiUsersListResponse>> =
        scroller_stream.to_stream(&session);

    while let Some(item) = resp_stream.try_next().await? {
        println!("res: {:#?}", item.members);
    }

    let collected_members: Vec<SlackUser> = scroller_stream
        .collect_items_stream(&session, Duration::from_millis(1000))
        .await?;
    println!("collected res: {:#?}", collected_members);

    let mut items_stream = scroller_stream.to_items_stream(&session);
    while let Some(items) = items_stream.try_next().await? {
        println!("res: {:#?}", items);
    }

    Ok(())
}

async fn test_oauth_install_function(
    resp: Result<SlackOAuthV2AccessTokenResponse, Box<dyn std::error::Error + Send + Sync>>,
) {
    println!("{:#?}", resp);
}

async fn test_push_events_function(
    resp: Result<SlackPushEvent, Box<dyn std::error::Error + Send + Sync>>,
) {
    println!("{:#?}", resp);
}

async fn test_server(
    client: Arc<SlackClient>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8080));
    info!("Loading server: {}", addr);

    async fn hello_world(
        _req: Request<Body>,
    ) -> Result<Response<Body>, Box<dyn std::error::Error + Send + Sync>> {
        Response::builder()
            .body("Hello, World".into())
            .map_err(|e| e.into())
    }

    let oauth_listener_config = Arc::new(SlackOAuthListenerConfig::new(
        std::env::var("SLACK_CLIENT_ID")?,
        std::env::var("SLACK_CLIENT_SECRET")?,
        std::env::var("SLACK_BOT_SCOPE")?,
        std::env::var("SLACK_REDIRECT_HOST")?,
    ));

    let push_events_config = Arc::new(SlackPushEventsListenerConfig::new(std::env::var("SLACK_SIGNING_SECRET")?));

    let make_svc = make_service_fn(move |_| {
        let thread_oauth_config = oauth_listener_config.clone();
        let thread_push_events_config = push_events_config.clone();
        let thread_slack_client = client.clone();
        async move {
            let routes = chain_service_routes_fn(
                create_slack_oauth_service_fn(
                    thread_oauth_config,
                    thread_slack_client,
                    test_oauth_install_function,
                ),
                chain_service_routes_fn(
                    create_slack_push_events_service_fn(
                        thread_push_events_config,
                        test_push_events_function,
                    ),
                    hello_world,
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init();
    let client: Arc<SlackClient> = Arc::new(SlackClient::new());
    test_server(client.clone()).await?;

    Ok(())
}
