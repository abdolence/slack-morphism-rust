use rsb_derive::Builder;
use serde::de::{MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::skip_serializing_none;
use std::*;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackIcon {
    pub image_original: Option<String>,
    pub image_default: Option<bool>,
    #[serde(flatten)]
    pub images: Option<SlackIconImages>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Builder)]
pub struct SlackIconImages {
    pub resolutions: Vec<(u32, String)>,
}

const SLACK_ICON_JSON_PREFIX: &str = "image_";

impl Serialize for SlackIconImages {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if self.resolutions.is_empty() {
            serializer.serialize_none()
        } else {
            let mut res_map = serializer.serialize_map(Some(self.resolutions.len()))?;
            for (res, link) in &self.resolutions {
                let key: String = format!("{SLACK_ICON_JSON_PREFIX}{res}");
                res_map.serialize_entry(&key, link)?;
            }
            res_map.end()
        }
    }
}

struct SlackIconImagesVisitor {}

impl SlackIconImagesVisitor {
    fn new() -> Self {
        Self {}
    }
}

impl<'de> Visitor<'de> for SlackIconImagesVisitor {
    type Value = SlackIconImages;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a json contains images_ for icon")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut resolutions: Vec<(u32, String)> =
            Vec::with_capacity(access.size_hint().unwrap_or(0));

        while let Some(key) = access.next_key::<String>()? {
            if key.starts_with(SLACK_ICON_JSON_PREFIX) {
                let parsed_key: Vec<_> = key.split('_').collect();
                if parsed_key.len() == 2 {
                    let resolution: u32 = parsed_key[1].parse().unwrap();
                    let value: String = access.next_value()?;
                    resolutions.push((resolution, value).clone());
                }
            }
        }

        Ok(SlackIconImages::new(resolutions))
    }
}

impl<'de> Deserialize<'de> for SlackIconImages {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(SlackIconImagesVisitor::new())
    }
}
