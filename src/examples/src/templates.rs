use chrono::prelude::*;
use rsb_derive::Builder;
use slack_morphism_models::blocks::*;
use slack_morphism_models::*;

#[derive(Debug, Clone, Builder)]
pub struct WelcomeMessageTemplateParams {
    user_id: SlackUserId,
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
                some_into(SlackContextBlock::new(slack_blocks![
                    some(md!("This is an example of block message")),
                    some(md!(
                        "Current time is: {}",
                        fmt_slack_date(
                            Local::now(),
                            SlackDateTimeFormats::DatePretty.to_string().as_str(),
                            None
                        )
                    ))
                ])),
                some_into(SlackDividerBlock::new()),
                some_into(SlackImageBlock::new(
                    "https://www.gstatic.com/webp/gallery3/2_webp_ll.png".into(),
                    "Test Image".into()
                )),
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
                        Local::now(),
                        SlackDateTimeFormats::DatePretty.to_string().as_str(),
                        None
                    )
                ))
            ]))
        ]
    }
}
