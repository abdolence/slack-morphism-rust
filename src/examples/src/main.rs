use slack_morphism_client::api::test::*;
use slack_morphism_client::api::users::*;
use slack_morphism_client::scroller::*;
use slack_morphism_client::*;

use futures::stream::BoxStream;
use futures::TryStreamExt;
use slack_morphism_models::blocks::kit::*;
use slack_morphism_models::common::SlackCursorId;
use slack_morphism_models::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let sb: SlackSectionBlock = SlackSectionBlock::new().with_block_id("test".into());
    let sb_ser = serde_json::to_string_pretty(&sb).unwrap();
    let sb_des: SlackSectionBlock = serde_json::from_str(&sb_ser).unwrap();
    println!("{} {:?}", sb_ser, sb_des);

    let section_block = SlackSectionBlock::new()
        .with_text(md!("hey, {}", 10))
        .with_fields(slack_items! [
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

    let blocks: Vec<SlackBlock> = slack_blocks! [
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

    let scroller_stream: Box<
        dyn SlackApiResponseScroller<
            ResponseType = SlackApiUsersListResponse,
            CursorType = SlackCursorId,
        >,
    > = SlackApiUsersListRequest::new().with_limit(1).scroller();

    let mut resp_stream: BoxStream<ClientResult<SlackApiUsersListResponse>> =
        scroller_stream.to_stream(&session);

    while let Some(item) = resp_stream.try_next().await? {
        println!("res: {:#?}", item);
    }

    Ok(())
}
