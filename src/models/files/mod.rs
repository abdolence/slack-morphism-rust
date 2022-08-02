use crate::*;

use rsb_derive::Builder;
use rvstruct::*;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use url::Url;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackFileId(pub String);

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackFileType(pub String);

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackFilePrettyType(pub String);

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackFileExternalType(pub String);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackFile {
    pub id: SlackFileId,
    pub created: SlackDateTime,
    pub timestamp: SlackDateTime,
    pub name: String,
    pub title: Option<String>,
    pub mimetype: Option<SlackMimeType>,
    pub filetype: Option<SlackFileType>,
    pub pretty_type: Option<SlackFilePrettyType>,
    pub external_type: Option<SlackFileExternalType>,
    pub user: Option<SlackUserId>,
    pub username: Option<String>,
    pub url_private: Option<Url>,
    pub url_private_download: Option<Url>,
    pub permalink: Option<Url>,
    pub permalink_public: Option<Url>,
    pub reactions: Option<Vec<SlackReaction>>,
    #[serde(flatten)]
    pub flags: SlackFileFlags,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackFileFlags {
    pub editable: Option<bool>,
    pub is_external: Option<bool>,
    pub is_public: Option<bool>,
    pub public_url_shared: Option<bool>,
    pub display_as_bot: Option<bool>,
    pub is_starred: Option<bool>,
    pub has_rich_preview: Option<bool>,
}
