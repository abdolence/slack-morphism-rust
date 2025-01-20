use std::fmt;

use rvstruct::ValueStruct;
use serde::{
    de::{MapAccess, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use url::Url;

/// Represent a Slack custom emoji name without the leading `:` and trailing `:`
#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackEmojiName(pub String);

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum SlackEmojiRef {
    Url(Url),
    Alias(SlackEmojiName),
}

const SLACK_EMOJI_ALIAS_PREFIX: &str = "alias:";

impl Serialize for SlackEmojiRef {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            SlackEmojiRef::Url(url) => serializer.serialize_str(url.as_ref()),
            SlackEmojiRef::Alias(alias) => {
                serializer.serialize_str(&format!("{SLACK_EMOJI_ALIAS_PREFIX}{alias}"))
            }
        }
    }
}

struct SlackEmojiRefVisitor {}

impl SlackEmojiRefVisitor {
    fn new() -> Self {
        Self {}
    }
}

impl<'de> Visitor<'de> for SlackEmojiRefVisitor {
    type Value = SlackEmojiRef;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a Slack custom emoji URL or alias in the form of 'alias:<name>'")
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if v.starts_with(SLACK_EMOJI_ALIAS_PREFIX) {
            let parsed_value: Vec<_> = v.split(':').collect();
            if parsed_value.len() == 2 {
                return Ok(SlackEmojiRef::Alias(SlackEmojiName(
                    parsed_value[1].to_string(),
                )));
            }
        }

        let emoji_url: Url = v.parse().unwrap();
        Ok(SlackEmojiRef::Url(emoji_url))
    }
}

impl<'de> Deserialize<'de> for SlackEmojiRef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(SlackEmojiRefVisitor::new())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod serialize {
        use super::*;

        #[test]
        fn test_serialize_emoji_url() {
            let emoji_url = SlackEmojiRef::Url(Url::parse("https://example.com").unwrap());
            assert_eq!(
                serde_json::to_string(&emoji_url).unwrap(),
                "\"https://example.com/\""
            );
        }

        #[test]
        fn test_serialize_emoji_alias() {
            let emoji_alias = SlackEmojiRef::Alias(SlackEmojiName::new("smile".to_string()));
            assert_eq!(
                serde_json::to_string(&emoji_alias).unwrap(),
                "\"alias:smile\""
            );
        }
    }

    mod deserialize {
        use super::*;

        #[test]
        fn test_deserialize_emoji_url() {
            assert_eq!(
                serde_json::from_str::<SlackEmojiRef>("\"https://example.com\"").unwrap(),
                SlackEmojiRef::Url(Url::parse("https://example.com").unwrap())
            );
        }

        #[test]
        fn test_serialize_emoji_alias() {
            assert_eq!(
                serde_json::from_str::<SlackEmojiRef>("\"alias:smile\"").unwrap(),
                SlackEmojiRef::Alias(SlackEmojiName::new("smile".to_string()))
            );
        }
    }
}
