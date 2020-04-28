use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/**
* 'plain_text' type of https://api.slack.com/reference/block-kit/composition-objects#text
*/
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockPlainText {
    pub text: String,
    pub emoji: Option<bool>,
}

/**
 * 'mrkdwn' type of https://api.slack.com/reference/block-kit/composition-objects#text
 */
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockMarkDownText {
    pub text: String,
    pub verbatim: Option<bool>,
}

/**
 * https://api.slack.com/reference/block-kit/composition-objects#text
 */
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SlackBlockText {
    #[serde(rename = "plain_text")]
    Plain(SlackBlockPlainText),
    #[serde(rename = "mrkdwn")]
    MarkDown(SlackBlockMarkDownText),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename = "plain_text")]
pub struct SlackBlockPlainTextOnly {
    #[serde(flatten)]
    value: SlackBlockPlainText,
}

impl SlackBlockPlainText {
    pub fn as_block_text(&self) -> SlackBlockText {
        SlackBlockText::Plain(self.clone())
    }
}

impl From<String> for SlackBlockPlainText {
    fn from(value: String) -> Self {
        SlackBlockPlainText::new(value)
    }
}

impl From<&str> for SlackBlockPlainText {
    fn from(value: &str) -> Self {
        SlackBlockPlainText::new(String::from(value))
    }
}

impl SlackBlockMarkDownText {
    pub fn as_block_text(&self) -> SlackBlockText {
        SlackBlockText::MarkDown(self.clone())
    }
}

impl From<String> for SlackBlockMarkDownText {
    fn from(value: String) -> Self {
        SlackBlockMarkDownText::new(value)
    }
}

impl From<&str> for SlackBlockMarkDownText {
    fn from(value: &str) -> Self {
        SlackBlockMarkDownText::new(String::from(value))
    }
}

impl From<&SlackBlockPlainText> for SlackBlockPlainTextOnly {
    fn from(pt: &SlackBlockPlainText) -> Self {
        SlackBlockPlainTextOnly { value: pt.clone() }
    }
}

impl From<SlackBlockPlainText> for SlackBlockText {
    fn from(text: SlackBlockPlainText) -> Self {
        SlackBlockText::Plain(text)
    }
}

impl From<SlackBlockMarkDownText> for SlackBlockText {
    fn from(text: SlackBlockMarkDownText) -> Self {
        SlackBlockText::MarkDown(text)
    }
}

impl From<SlackBlockPlainTextOnly> for SlackBlockText {
    fn from(text: SlackBlockPlainTextOnly) -> Self {
        SlackBlockText::Plain(text.value)
    }
}
