use crate::blocks::SlackBlock;
use crate::SlackMessageContent;

pub trait SlackMessageTemplate {
    fn render_template(&self) -> SlackMessageContent;
}

pub trait SlackBlocksTemplate {
    fn render_template(&self) -> Vec<SlackBlock>;
}
