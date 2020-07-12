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
            .with_text(format!("Hey {}", Self::fmt_user_id(&self.user_id)))
            .with_blocks(slack_blocks![
                some_into(
                    SlackSectionBlock::new()
                        .with_text(md!("Hey {}", Self::fmt_user_id(&self.user_id)))
                ),
                some_into(SlackDividerBlock::new()),
                some_into(SlackContextBlock::new(slack_blocks![
                    some_into(SlackBlockImageElement::new("".into(), "".into())),
                    some(md!("This is an example of block message"))
                ]))
            ])
    }
}
