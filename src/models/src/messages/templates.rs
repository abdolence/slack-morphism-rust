use crate::SlackMessageContent;

pub trait SlackMessageTemplate {
    fn render_template(&self) -> SlackMessageContent;
}
