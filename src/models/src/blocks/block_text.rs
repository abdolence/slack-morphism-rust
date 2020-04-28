use serde::{Serialize, Deserialize};

/**
* 'plain_text' type of https://api.slack.com/reference/block-kit/composition-objects#text
*/
#[serde_with::skip_serializing_none]
#[derive(Debug,PartialEq,Clone,Serialize,Deserialize)]
pub struct SlackBlockPlainText {
    pub text : String,
    pub emoji : Option<bool>
}

/**
 * 'mrkdwn' type of https://api.slack.com/reference/block-kit/composition-objects#text
 */
#[serde_with::skip_serializing_none]
#[derive(Debug,PartialEq,Clone,Serialize,Deserialize)]
pub struct SlackBlockMarkDownText {
    pub text : String,
    pub verbatim : Option<bool>
}

/**
 * https://api.slack.com/reference/block-kit/composition-objects#text
 */
#[derive(Debug,PartialEq,Clone,Serialize,Deserialize)]
#[serde(tag = "type")]
pub enum SlackBlockText {
    #[serde(rename = "plain_text")]
    Plain (SlackBlockPlainText),
    #[serde(rename = "mrkdwn")]
    MarkDown (SlackBlockMarkDownText)
}

#[derive(Debug,PartialEq,Clone,Serialize,Deserialize)]
#[serde(tag = "type")]
pub enum SlackBlockPlainTextOnly {
    #[serde(rename = "plain_text")]
    Plain (SlackBlockPlainText)
}

impl SlackBlockPlainText {

    pub fn new ( text : &String ) -> Self {
        SlackBlockPlainText {
            text : text.clone(),
            emoji : None
        }
    }

    pub fn with_emoji (&mut self, emoji : bool ) -> &Self {
        self.emoji = Some(emoji);
        self
    }

    pub fn as_block_text(&self) -> SlackBlockText {
        SlackBlockText::Plain(self.clone())
    }

}

impl From<String> for SlackBlockPlainText {
    fn from(value: String) -> Self {
        SlackBlockPlainText::new(&value).clone()
    }
}

impl From<&str> for SlackBlockPlainText {
    fn from(value: &str) -> Self {
        SlackBlockPlainText::new(&String::from(value)).clone()
    }
}

impl SlackBlockMarkDownText {

    pub fn new ( text : &String ) -> Self {
        SlackBlockMarkDownText {
            text : text.clone(),
            verbatim : None
        }
    }

    pub fn with_verbatim (&mut self, verbatim : bool ) -> &Self {
        self.verbatim = Some(verbatim);
        self
    }

    pub fn as_block_text(&self) -> SlackBlockText {
        SlackBlockText::MarkDown(self.clone())
    }

}

impl From<String> for SlackBlockMarkDownText {
    fn from(value: String) -> Self {
        SlackBlockMarkDownText::new(&value).clone()
    }
}

impl From<&str> for SlackBlockMarkDownText {
    fn from(value: &str) -> Self {
        SlackBlockMarkDownText::new(&String::from(value)).clone()
    }
}