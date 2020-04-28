use rsb_derive::Builder;
use rvs_derive::ValueStruct;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::blocks::block_text::*;
use crate::common::*;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackBlockId(pub String);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SlackBlock {
    #[serde(rename = "section")]
    Section(SlackSectionBlock),
    #[serde(rename = "divider")]
    Divider(SlackDividerBlock),
    #[serde(rename = "image")]
    Image(SlackImageBlock),
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackSectionBlock {
    pub block_id: Option<SlackBlockId>,
    pub text: Option<SlackBlockText>,
    pub fields: Option<Vec<SlackBlockText>>,
    pub accessory: Option<SlackSectionBlockElement>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackDividerBlock{
    pub block_id: Option<SlackBlockId>
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackImageBlock {
    pub block_id: Option<SlackBlockId>,
    pub image_url: String,
    pub alt_text: String,
    pub title: Option<SlackBlockPlainText>
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SlackSectionBlockElement {
    #[serde(rename = "image")]
    Image(SlackBlockImageElement),
    #[serde(rename = "button")]
    Button(SlackBlockButtonElement),
    #[serde(rename = "static_select")]
    StaticSelect(SlackBlockStaticSelectElement),
    #[serde(rename = "multi_static_select")]
    MultiStaticSelect(SlackBlockMultiStaticSelectElement),
    #[serde(rename = "external_select")]
    ExternalSelect(SlackBlockExternalSelectElement),
    #[serde(rename = "multi_external_select")]
    MultiExternalSelect(SlackBlockMultiExternalSelectElement),
    #[serde(rename = "users_select")]
    UsersListSelect(SlackBlockUsersSelectElement),
    #[serde(rename = "multi_users_select")]
    MultiUsersSelect(SlackBlockMultiUsersSelectElement),
    #[serde(rename = "conversations_select")]
    ConversationsSelect(SlackBlockConversationsSelectElement),
    #[serde(rename = "multi_conversations_select")]
    MultiConversationsSelect(SlackBlockMultiConversationsSelectElement),
    #[serde(rename = "channels_select")]
    ChannelsSelect(SlackBlockChannelsSelectElement),
    #[serde(rename = "multi_channels_select")]
    MultiChannelsSelect(SlackBlockMultiChannelsSelectElement),
    #[serde(rename = "overflow")]
    Overflow(SlackBlockOverflowElement),
    #[serde(rename = "datepicker")]
    DatePicker(SlackBlockDatePickerElement),
    #[serde(rename = "plain_text_input")]
    PlainTextInput(SlackBlockPlainTextInputElement),
    #[serde(rename = "radio_buttons")]
    RadioButtons(SlackBlockRadioButtonsElement),
    #[serde(rename = "checkboxes")]
    Checkboxes(SlackBlockCheckboxesElement),
    #[serde(rename = "rich_text_section")]
    RichTextSection,
    #[serde(rename = "rich_text_preformatted")]
    RichTextPreformatted,
    #[serde(rename = "rich_text_list")]
    RichTextList,
    #[serde(rename = "rich_text_quote")]
    RichTextQuote
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockImageElement {
    pub image_url: String,
    pub alt_text: String,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockButtonElement {
    pub action_id: SlackActionId,
    pub text: SlackBlockPlainTextOnly,
    pub url: Option<String>,
    pub value: Option<String>,
    pub style: Option<String>,
    pub confirm: Option<SlackBlockConfirmItem>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockConfirmItem {
    pub title: SlackBlockPlainTextOnly,
    pub text: SlackBlockText,
    pub confirm: SlackBlockPlainTextOnly,
    pub deny: SlackBlockPlainTextOnly,
    pub style: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockChoiceItem<T: Into<SlackBlockText>> {
    pub text: T,
    pub value: String,
    pub url: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockOptionGroup<T: Into<SlackBlockText>> {
    pub label: SlackBlockPlainTextOnly,
    pub options: Vec<SlackBlockChoiceItem<T>>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockStaticSelectElement {
    pub action_id: SlackActionId,
    pub placeholder: SlackBlockPlainTextOnly,
    pub options: Option<Vec<SlackBlockChoiceItem<SlackBlockPlainTextOnly>>>,
    pub option_groups: Option<Vec<SlackBlockOptionGroup<SlackBlockPlainTextOnly>>>,
    pub initial_option: Option<SlackBlockChoiceItem<SlackBlockPlainTextOnly>>,
    pub confirm: Option<SlackBlockConfirmItem>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockMultiStaticSelectElement {
    pub action_id: SlackActionId,
    pub placeholder: SlackBlockPlainTextOnly,
    pub options: Option<Vec<SlackBlockChoiceItem<SlackBlockPlainTextOnly>>>,
    pub option_groups: Option<Vec<SlackBlockOptionGroup<SlackBlockPlainTextOnly>>>,
    pub initial_options: Option<Vec<SlackBlockChoiceItem<SlackBlockPlainTextOnly>>>,
    pub confirm: Option<SlackBlockConfirmItem>,
    pub max_selected_items: Option<u64>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockExternalSelectElement {
    pub action_id: SlackActionId,
    pub placeholder: SlackBlockPlainTextOnly,
    pub initial_option: Option<SlackBlockChoiceItem<SlackBlockPlainTextOnly>>,
    pub confirm: Option<SlackBlockConfirmItem>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockMultiExternalSelectElement {
    pub action_id: SlackActionId,
    pub placeholder: SlackBlockPlainTextOnly,
    pub initial_options: Option<Vec<SlackBlockChoiceItem<SlackBlockPlainTextOnly>>>,
    pub confirm: Option<SlackBlockConfirmItem>,
    pub max_selected_items: Option<u64>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockUsersSelectElement {
    pub action_id: SlackActionId,
    pub placeholder: SlackBlockPlainTextOnly,
    pub initial_user: Option<String>,
    pub confirm: Option<SlackBlockConfirmItem>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockMultiUsersSelectElement {
    pub action_id: SlackActionId,
    pub placeholder: SlackBlockPlainTextOnly,
    pub initial_users: Option<Vec<String>>,
    pub confirm: Option<SlackBlockConfirmItem>,
    pub max_selected_items: Option<u64>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockConversationsSelectElement {
    pub action_id: SlackActionId,
    pub placeholder: SlackBlockPlainTextOnly,
    pub initial_conversation: Option<SlackConversationId>,
    pub default_to_current_conversation: Option<bool>,
    pub confirm: Option<SlackBlockConfirmItem>,
    pub response_url_enabled: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockMultiConversationsSelectElement {
    pub action_id: SlackActionId,
    pub placeholder: SlackBlockPlainTextOnly,
    pub initial_conversations: Option<Vec<SlackConversationId>>,
    pub default_to_current_conversation: Option<bool>,
    pub confirm: Option<SlackBlockConfirmItem>,
    pub max_selected_items: Option<u64>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockChannelsSelectElement {
    pub action_id: SlackActionId,
    pub placeholder: SlackBlockPlainTextOnly,
    pub initial_channel: Option<SlackChannelId>,
    pub confirm: Option<SlackBlockConfirmItem>,
    pub response_url_enabled: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockMultiChannelsSelectElement {
    pub action_id: SlackActionId,
    pub placeholder: SlackBlockPlainTextOnly,
    pub initial_channels: Option<Vec<SlackChannelId>>,
    pub confirm: Option<SlackBlockConfirmItem>,
    pub max_selected_items: Option<u64>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockOverflowElement {
    pub action_id: SlackActionId,
    pub options: Vec<SlackBlockChoiceItem<SlackBlockPlainTextOnly>>,
    pub confirm: Option<SlackBlockConfirmItem>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockDatePickerElement {
    pub action_id: SlackActionId,
    pub placeholder: SlackBlockPlainTextOnly,
    pub initial_date: Option<String>,
    pub confirm: Option<SlackBlockConfirmItem>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockPlainTextInputElement {
    pub action_id: SlackActionId,
    pub placeholder: SlackBlockPlainTextOnly,
    pub initial_value: Option<String>,
    pub multiline: Option<bool>,
    pub min_length: Option<u64>,
    pub max_length: Option<u64>
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockRadioButtonsElement {
    pub action_id: SlackActionId,
    pub options: Vec<SlackBlockChoiceItem<SlackBlockText>>,
    pub initial_option: Option<SlackBlockChoiceItem<SlackBlockText>>,
    pub confirm: Option<SlackBlockConfirmItem>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockCheckboxesElement {
    pub action_id: SlackActionId,
    pub options: Vec<SlackBlockChoiceItem<SlackBlockText>>,
    pub initial_options: Option<Vec<SlackBlockChoiceItem<SlackBlockText>>>,
    pub confirm: Option<SlackBlockConfirmItem>,
}
