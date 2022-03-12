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
    id: SlackFileId,
    created: SlackDateTime,
    timestamp: SlackDateTime,
    name: String,
    title: Option<String>,
    mimetype: Option<SlackMimeType>,
    filetype: Option<SlackFileType>,
    pretty_type: Option<SlackFilePrettyType>,
    external_type: Option<SlackFileExternalType>,
    user: Option<SlackUserId>,
    username: Option<String>,
    url_private: Option<Url>,
    url_private_download: Option<Url>,
    permalink: Option<Url>,
    permalink_public: Option<Url>,
    #[serde(flatten)]
    flags: SlackFileFlags,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackFileFlags {
    editable: Option<bool>,
    is_external: Option<bool>,
    is_public: Option<bool>,
    public_url_shared: Option<bool>,
    display_as_bot: Option<bool>,
    is_starred: Option<bool>,
    has_rich_preview: Option<bool>,
}
