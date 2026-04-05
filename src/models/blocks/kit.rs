use rsb_derive::Builder;
use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use url::Url;

use crate::*;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Eq, Hash, Serialize, Deserialize, ValueStruct)]
pub struct SlackBlockId(pub String);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SlackBlock {
    #[serde(rename = "section")]
    Section(SlackSectionBlock),
    #[serde(rename = "header")]
    Header(SlackHeaderBlock),
    #[serde(rename = "divider")]
    Divider(SlackDividerBlock),
    #[serde(rename = "image")]
    Image(SlackImageBlock),
    #[serde(rename = "actions")]
    Actions(SlackActionsBlock),
    #[serde(rename = "context")]
    Context(SlackContextBlock),
    #[serde(rename = "input")]
    Input(SlackInputBlock),
    #[serde(rename = "file")]
    File(SlackFileBlock),
    #[serde(rename = "video")]
    Video(SlackVideoBlock),
    #[serde(rename = "markdown")]
    Markdown(SlackMarkdownBlock),

    #[serde(rename = "rich_text")]
    RichText(SlackRichTextBlock),
    #[serde(rename = "share_shortcut")]
    ShareShortcut(serde_json::Value),
    #[serde(rename = "event")]
    Event(serde_json::Value),
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackSectionBlock {
    pub block_id: Option<SlackBlockId>,
    pub text: Option<SlackBlockText>,
    pub fields: Option<Vec<SlackBlockText>>,
    pub accessory: Option<SlackSectionBlockElement>,
}

impl From<SlackSectionBlock> for SlackBlock {
    fn from(block: SlackSectionBlock) -> Self {
        SlackBlock::Section(block)
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackHeaderBlock {
    pub block_id: Option<SlackBlockId>,
    pub text: SlackBlockPlainTextOnly,
}

impl From<SlackHeaderBlock> for SlackBlock {
    fn from(block: SlackHeaderBlock) -> Self {
        SlackBlock::Header(block)
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackDividerBlock {
    pub block_id: Option<SlackBlockId>,
}

impl From<SlackDividerBlock> for SlackBlock {
    fn from(block: SlackDividerBlock) -> Self {
        SlackBlock::Divider(block)
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackImageBlock {
    pub block_id: Option<SlackBlockId>,
    #[serde(flatten)]
    pub image_url_or_file: SlackImageUrlOrFile,
    pub alt_text: String,
    pub title: Option<SlackBlockPlainTextOnly>,
}

impl From<SlackImageBlock> for SlackBlock {
    fn from(block: SlackImageBlock) -> Self {
        SlackBlock::Image(block)
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackActionsBlock {
    pub block_id: Option<SlackBlockId>,
    pub elements: Vec<SlackActionBlockElement>,
}

impl From<SlackActionsBlock> for SlackBlock {
    fn from(block: SlackActionsBlock) -> Self {
        SlackBlock::Actions(block)
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackContextBlock {
    pub block_id: Option<SlackBlockId>,
    pub elements: Vec<SlackContextBlockElement>,
}

impl From<SlackContextBlock> for SlackBlock {
    fn from(block: SlackContextBlock) -> Self {
        SlackBlock::Context(block)
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackInputBlock {
    pub block_id: Option<SlackBlockId>,
    pub label: SlackBlockPlainTextOnly,
    pub element: SlackInputBlockElement,
    pub hint: Option<SlackBlockPlainTextOnly>,
    pub optional: Option<bool>,
    pub dispatch_action: Option<bool>,
}

impl From<SlackInputBlock> for SlackBlock {
    fn from(block: SlackInputBlock) -> Self {
        SlackBlock::Input(block)
    }
}

const SLACK_FILE_BLOCK_SOURCE_DEFAULT: &str = "remote";

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackFileBlock {
    pub block_id: Option<SlackBlockId>,
    pub external_id: String,
    #[default = "SLACK_FILE_BLOCK_SOURCE_DEFAULT.into()"]
    pub source: String,
}

impl From<SlackFileBlock> for SlackBlock {
    fn from(block: SlackFileBlock) -> Self {
        SlackBlock::File(block)
    }
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
    UsersSelect(SlackBlockUsersSelectElement),
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
    #[serde(rename = "timepicker")]
    TimePicker(SlackBlockTimePickerElement),
    #[serde(rename = "plain_text_input")]
    PlainTextInput(SlackBlockPlainTextInputElement),
    #[serde(rename = "number_input")]
    NumberInput(SlackBlockNumberInputElement),
    #[serde(rename = "url_text_input")]
    UrlInput(SlackBlockUrlInputElement),
    #[serde(rename = "radio_buttons")]
    RadioButtons(SlackBlockRadioButtonsElement),
    #[serde(rename = "checkboxes")]
    Checkboxes(SlackBlockCheckboxesElement),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SlackActionBlockElement {
    #[serde(rename = "button")]
    Button(SlackBlockButtonElement),
    #[serde(rename = "overflow")]
    Overflow(SlackBlockOverflowElement),
    #[serde(rename = "datepicker")]
    DatePicker(SlackBlockDatePickerElement),
    #[serde(rename = "timepicker")]
    TimePicker(SlackBlockTimePickerElement),
    #[serde(rename = "datetimepicker")]
    DateTimePicker(SlackBlockDateTimePickerElement),
    #[serde(rename = "plain_text_input")]
    PlainTextInput(SlackBlockPlainTextInputElement),
    #[serde(rename = "number_input")]
    NumberInput(SlackBlockNumberInputElement),
    #[serde(rename = "url_text_input")]
    UrlInput(SlackBlockUrlInputElement),
    #[serde(rename = "radio_buttons")]
    RadioButtons(SlackBlockRadioButtonsElement),
    #[serde(rename = "checkboxes")]
    Checkboxes(SlackBlockCheckboxesElement),
    #[serde(rename = "static_select")]
    StaticSelect(SlackBlockStaticSelectElement),
    #[serde(rename = "external_select")]
    ExternalSelect(SlackBlockExternalSelectElement),
    #[serde(rename = "users_select")]
    UsersSelect(SlackBlockUsersSelectElement),
    #[serde(rename = "conversations_select")]
    ConversationsSelect(SlackBlockConversationsSelectElement),
    #[serde(rename = "channels_select")]
    ChannelsSelect(SlackBlockChannelsSelectElement),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SlackContextBlockElement {
    #[serde(rename = "image")]
    Image(SlackBlockImageElement),
    #[serde(rename = "plain_text")]
    Plain(SlackBlockPlainText),
    #[serde(rename = "mrkdwn")]
    MarkDown(SlackBlockMarkDownText),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SlackInputBlockElement {
    #[serde(rename = "static_select")]
    StaticSelect(SlackBlockStaticSelectElement),
    #[serde(rename = "multi_static_select")]
    MultiStaticSelect(SlackBlockMultiStaticSelectElement),
    #[serde(rename = "external_select")]
    ExternalSelect(SlackBlockExternalSelectElement),
    #[serde(rename = "multi_external_select")]
    MultiExternalSelect(SlackBlockMultiExternalSelectElement),
    #[serde(rename = "users_select")]
    UsersSelect(SlackBlockUsersSelectElement),
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
    #[serde(rename = "datepicker")]
    DatePicker(SlackBlockDatePickerElement),
    #[serde(rename = "timepicker")]
    TimePicker(SlackBlockTimePickerElement),
    #[serde(rename = "datetimepicker")]
    DateTimePicker(SlackBlockDateTimePickerElement),
    #[serde(rename = "plain_text_input")]
    PlainTextInput(SlackBlockPlainTextInputElement),
    #[serde(rename = "number_input")]
    NumberInput(SlackBlockNumberInputElement),
    #[serde(rename = "url_text_input")]
    UrlInput(SlackBlockUrlInputElement),
    #[serde(rename = "radio_buttons")]
    RadioButtons(SlackBlockRadioButtonsElement),
    #[serde(rename = "checkboxes")]
    Checkboxes(SlackBlockCheckboxesElement),
    #[serde(rename = "email_text_input")]
    EmailInput(SlackBlockEmailInputElement),
    #[serde(rename = "rich_text_input")]
    RichTextInput(SlackBlockRichTextInputElement),
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockImageElement {
    #[serde(flatten)]
    pub image_url_or_file: SlackImageUrlOrFile,
    pub alt_text: String,
}

impl From<SlackBlockImageElement> for SlackSectionBlockElement {
    fn from(element: SlackBlockImageElement) -> Self {
        SlackSectionBlockElement::Image(element)
    }
}

impl From<SlackBlockImageElement> for SlackContextBlockElement {
    fn from(element: SlackBlockImageElement) -> Self {
        SlackContextBlockElement::Image(element)
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockButtonElement {
    pub action_id: SlackActionId,
    pub text: SlackBlockPlainTextOnly,
    pub url: Option<Url>,
    pub value: Option<String>,
    pub style: Option<String>,
    pub confirm: Option<SlackBlockConfirmItem>,
}

impl From<SlackBlockButtonElement> for SlackSectionBlockElement {
    fn from(element: SlackBlockButtonElement) -> Self {
        SlackSectionBlockElement::Button(element)
    }
}

impl From<SlackBlockButtonElement> for SlackActionBlockElement {
    fn from(element: SlackBlockButtonElement) -> Self {
        SlackActionBlockElement::Button(element)
    }
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
    pub url: Option<Url>,
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
    pub placeholder: Option<SlackBlockPlainTextOnly>,
    pub options: Option<Vec<SlackBlockChoiceItem<SlackBlockPlainTextOnly>>>,
    pub option_groups: Option<Vec<SlackBlockOptionGroup<SlackBlockPlainTextOnly>>>,
    pub initial_option: Option<SlackBlockChoiceItem<SlackBlockPlainTextOnly>>,
    pub confirm: Option<SlackBlockConfirmItem>,
    pub focus_on_load: Option<bool>,
}

impl From<SlackBlockStaticSelectElement> for SlackSectionBlockElement {
    fn from(element: SlackBlockStaticSelectElement) -> Self {
        SlackSectionBlockElement::StaticSelect(element)
    }
}

impl From<SlackBlockStaticSelectElement> for SlackInputBlockElement {
    fn from(element: SlackBlockStaticSelectElement) -> Self {
        SlackInputBlockElement::StaticSelect(element)
    }
}

impl From<SlackBlockStaticSelectElement> for SlackActionBlockElement {
    fn from(element: SlackBlockStaticSelectElement) -> Self {
        SlackActionBlockElement::StaticSelect(element)
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockMultiStaticSelectElement {
    pub action_id: SlackActionId,
    pub placeholder: Option<SlackBlockPlainTextOnly>,
    pub options: Option<Vec<SlackBlockChoiceItem<SlackBlockPlainTextOnly>>>,
    pub option_groups: Option<Vec<SlackBlockOptionGroup<SlackBlockPlainTextOnly>>>,
    pub initial_options: Option<Vec<SlackBlockChoiceItem<SlackBlockPlainTextOnly>>>,
    pub confirm: Option<SlackBlockConfirmItem>,
    pub max_selected_items: Option<u64>,
    pub focus_on_load: Option<bool>,
}

impl From<SlackBlockMultiStaticSelectElement> for SlackSectionBlockElement {
    fn from(element: SlackBlockMultiStaticSelectElement) -> Self {
        SlackSectionBlockElement::MultiStaticSelect(element)
    }
}

impl From<SlackBlockMultiStaticSelectElement> for SlackInputBlockElement {
    fn from(element: SlackBlockMultiStaticSelectElement) -> Self {
        SlackInputBlockElement::MultiStaticSelect(element)
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockExternalSelectElement {
    pub action_id: SlackActionId,
    pub placeholder: Option<SlackBlockPlainTextOnly>,
    pub initial_option: Option<SlackBlockChoiceItem<SlackBlockPlainTextOnly>>,
    pub confirm: Option<SlackBlockConfirmItem>,
    pub focus_on_load: Option<bool>,
    pub min_query_length: Option<u64>,
}

impl From<SlackBlockExternalSelectElement> for SlackSectionBlockElement {
    fn from(element: SlackBlockExternalSelectElement) -> Self {
        SlackSectionBlockElement::ExternalSelect(element)
    }
}

impl From<SlackBlockExternalSelectElement> for SlackInputBlockElement {
    fn from(element: SlackBlockExternalSelectElement) -> Self {
        SlackInputBlockElement::ExternalSelect(element)
    }
}

impl From<SlackBlockExternalSelectElement> for SlackActionBlockElement {
    fn from(element: SlackBlockExternalSelectElement) -> Self {
        SlackActionBlockElement::ExternalSelect(element)
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockMultiExternalSelectElement {
    pub action_id: SlackActionId,
    pub placeholder: Option<SlackBlockPlainTextOnly>,
    pub initial_options: Option<Vec<SlackBlockChoiceItem<SlackBlockPlainTextOnly>>>,
    pub confirm: Option<SlackBlockConfirmItem>,
    pub max_selected_items: Option<u64>,
    pub focus_on_load: Option<bool>,
    pub min_query_length: Option<u64>,
}

impl From<SlackBlockMultiExternalSelectElement> for SlackSectionBlockElement {
    fn from(element: SlackBlockMultiExternalSelectElement) -> Self {
        SlackSectionBlockElement::MultiExternalSelect(element)
    }
}

impl From<SlackBlockMultiExternalSelectElement> for SlackInputBlockElement {
    fn from(element: SlackBlockMultiExternalSelectElement) -> Self {
        SlackInputBlockElement::MultiExternalSelect(element)
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockUsersSelectElement {
    pub action_id: SlackActionId,
    pub placeholder: Option<SlackBlockPlainTextOnly>,
    pub initial_user: Option<String>,
    pub confirm: Option<SlackBlockConfirmItem>,
    pub focus_on_load: Option<bool>,
}

impl From<SlackBlockUsersSelectElement> for SlackSectionBlockElement {
    fn from(element: SlackBlockUsersSelectElement) -> Self {
        SlackSectionBlockElement::UsersSelect(element)
    }
}

impl From<SlackBlockUsersSelectElement> for SlackInputBlockElement {
    fn from(element: SlackBlockUsersSelectElement) -> Self {
        SlackInputBlockElement::UsersSelect(element)
    }
}

impl From<SlackBlockUsersSelectElement> for SlackActionBlockElement {
    fn from(element: SlackBlockUsersSelectElement) -> Self {
        SlackActionBlockElement::UsersSelect(element)
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockMultiUsersSelectElement {
    pub action_id: SlackActionId,
    pub placeholder: Option<SlackBlockPlainTextOnly>,
    pub initial_users: Option<Vec<String>>,
    pub confirm: Option<SlackBlockConfirmItem>,
    pub max_selected_items: Option<u64>,
    pub focus_on_load: Option<bool>,
}

impl From<SlackBlockMultiUsersSelectElement> for SlackSectionBlockElement {
    fn from(element: SlackBlockMultiUsersSelectElement) -> Self {
        SlackSectionBlockElement::MultiUsersSelect(element)
    }
}

impl From<SlackBlockMultiUsersSelectElement> for SlackInputBlockElement {
    fn from(element: SlackBlockMultiUsersSelectElement) -> Self {
        SlackInputBlockElement::MultiUsersSelect(element)
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum SlackConversationFilterInclude {
    #[serde(rename = "im")]
    Im,
    #[serde(rename = "mpim")]
    Mpim,
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "private")]
    Private,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockConversationFilter {
    pub include: Option<Vec<SlackConversationFilterInclude>>,
    pub exclude_external_shared_channels: Option<bool>,
    pub exclude_bot_users: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockConversationsSelectElement {
    pub action_id: SlackActionId,
    pub placeholder: Option<SlackBlockPlainTextOnly>,
    pub initial_conversation: Option<SlackConversationId>,
    pub default_to_current_conversation: Option<bool>,
    pub confirm: Option<SlackBlockConfirmItem>,
    pub response_url_enabled: Option<bool>,
    pub focus_on_load: Option<bool>,
    pub filter: Option<SlackBlockConversationFilter>,
}

impl From<SlackBlockConversationsSelectElement> for SlackSectionBlockElement {
    fn from(element: SlackBlockConversationsSelectElement) -> Self {
        SlackSectionBlockElement::ConversationsSelect(element)
    }
}

impl From<SlackBlockConversationsSelectElement> for SlackInputBlockElement {
    fn from(element: SlackBlockConversationsSelectElement) -> Self {
        SlackInputBlockElement::ConversationsSelect(element)
    }
}

impl From<SlackBlockConversationsSelectElement> for SlackActionBlockElement {
    fn from(element: SlackBlockConversationsSelectElement) -> Self {
        SlackActionBlockElement::ConversationsSelect(element)
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockMultiConversationsSelectElement {
    pub action_id: SlackActionId,
    pub placeholder: Option<SlackBlockPlainTextOnly>,
    pub initial_conversations: Option<Vec<SlackConversationId>>,
    pub default_to_current_conversation: Option<bool>,
    pub confirm: Option<SlackBlockConfirmItem>,
    pub max_selected_items: Option<u64>,
    pub focus_on_load: Option<bool>,
    pub filter: Option<SlackBlockConversationFilter>,
}

impl From<SlackBlockMultiConversationsSelectElement> for SlackSectionBlockElement {
    fn from(element: SlackBlockMultiConversationsSelectElement) -> Self {
        SlackSectionBlockElement::MultiConversationsSelect(element)
    }
}

impl From<SlackBlockMultiConversationsSelectElement> for SlackInputBlockElement {
    fn from(element: SlackBlockMultiConversationsSelectElement) -> Self {
        SlackInputBlockElement::MultiConversationsSelect(element)
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockChannelsSelectElement {
    pub action_id: SlackActionId,
    pub placeholder: Option<SlackBlockPlainTextOnly>,
    pub initial_channel: Option<SlackChannelId>,
    pub confirm: Option<SlackBlockConfirmItem>,
    pub response_url_enabled: Option<bool>,
    pub focus_on_load: Option<bool>,
}

impl From<SlackBlockChannelsSelectElement> for SlackSectionBlockElement {
    fn from(element: SlackBlockChannelsSelectElement) -> Self {
        SlackSectionBlockElement::ChannelsSelect(element)
    }
}

impl From<SlackBlockChannelsSelectElement> for SlackInputBlockElement {
    fn from(element: SlackBlockChannelsSelectElement) -> Self {
        SlackInputBlockElement::ChannelsSelect(element)
    }
}

impl From<SlackBlockChannelsSelectElement> for SlackActionBlockElement {
    fn from(element: SlackBlockChannelsSelectElement) -> Self {
        SlackActionBlockElement::ChannelsSelect(element)
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockMultiChannelsSelectElement {
    pub action_id: SlackActionId,
    pub placeholder: Option<SlackBlockPlainTextOnly>,
    pub initial_channels: Option<Vec<SlackChannelId>>,
    pub confirm: Option<SlackBlockConfirmItem>,
    pub max_selected_items: Option<u64>,
    pub focus_on_load: Option<bool>,
}

impl From<SlackBlockMultiChannelsSelectElement> for SlackSectionBlockElement {
    fn from(element: SlackBlockMultiChannelsSelectElement) -> Self {
        SlackSectionBlockElement::MultiChannelsSelect(element)
    }
}

impl From<SlackBlockMultiChannelsSelectElement> for SlackInputBlockElement {
    fn from(element: SlackBlockMultiChannelsSelectElement) -> Self {
        SlackInputBlockElement::MultiChannelsSelect(element)
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockOverflowElement {
    pub action_id: SlackActionId,
    pub options: Vec<SlackBlockChoiceItem<SlackBlockPlainTextOnly>>,
    pub confirm: Option<SlackBlockConfirmItem>,
}

impl From<SlackBlockOverflowElement> for SlackSectionBlockElement {
    fn from(element: SlackBlockOverflowElement) -> Self {
        SlackSectionBlockElement::Overflow(element)
    }
}

impl From<SlackBlockOverflowElement> for SlackActionBlockElement {
    fn from(element: SlackBlockOverflowElement) -> Self {
        SlackActionBlockElement::Overflow(element)
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockDatePickerElement {
    pub action_id: SlackActionId,
    pub placeholder: Option<SlackBlockPlainTextOnly>,
    pub initial_date: Option<String>,
    pub confirm: Option<SlackBlockConfirmItem>,
    pub focus_on_load: Option<bool>,
}

impl From<SlackBlockDatePickerElement> for SlackSectionBlockElement {
    fn from(element: SlackBlockDatePickerElement) -> Self {
        SlackSectionBlockElement::DatePicker(element)
    }
}

impl From<SlackBlockDatePickerElement> for SlackInputBlockElement {
    fn from(element: SlackBlockDatePickerElement) -> Self {
        SlackInputBlockElement::DatePicker(element)
    }
}

impl From<SlackBlockDatePickerElement> for SlackActionBlockElement {
    fn from(element: SlackBlockDatePickerElement) -> Self {
        SlackActionBlockElement::DatePicker(element)
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockTimePickerElement {
    pub action_id: SlackActionId,
    pub initial_time: Option<String>,
    pub confirm: Option<SlackBlockConfirmItem>,
    pub focus_on_load: Option<bool>,
    pub placeholder: Option<SlackBlockPlainTextOnly>,
    pub timezone: Option<String>,
}

impl From<SlackBlockTimePickerElement> for SlackSectionBlockElement {
    fn from(element: SlackBlockTimePickerElement) -> Self {
        SlackSectionBlockElement::TimePicker(element)
    }
}

impl From<SlackBlockTimePickerElement> for SlackInputBlockElement {
    fn from(element: SlackBlockTimePickerElement) -> Self {
        SlackInputBlockElement::TimePicker(element)
    }
}

impl From<SlackBlockTimePickerElement> for SlackActionBlockElement {
    fn from(element: SlackBlockTimePickerElement) -> Self {
        SlackActionBlockElement::TimePicker(element)
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockDateTimePickerElement {
    pub action_id: SlackActionId,
    pub initial_date_time: Option<SlackDateTime>,
    pub confirm: Option<SlackBlockConfirmItem>,
    pub focus_on_load: Option<bool>,
}

impl From<SlackBlockDateTimePickerElement> for SlackInputBlockElement {
    fn from(element: SlackBlockDateTimePickerElement) -> Self {
        SlackInputBlockElement::DateTimePicker(element)
    }
}

impl From<SlackBlockDateTimePickerElement> for SlackActionBlockElement {
    fn from(element: SlackBlockDateTimePickerElement) -> Self {
        SlackActionBlockElement::DateTimePicker(element)
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockPlainTextInputElement {
    pub action_id: SlackActionId,
    pub placeholder: Option<SlackBlockPlainTextOnly>,
    pub initial_value: Option<String>,
    pub multiline: Option<bool>,
    pub min_length: Option<u64>,
    pub max_length: Option<u64>,
    pub focus_on_load: Option<bool>,
}

impl From<SlackBlockPlainTextInputElement> for SlackSectionBlockElement {
    fn from(element: SlackBlockPlainTextInputElement) -> Self {
        SlackSectionBlockElement::PlainTextInput(element)
    }
}

impl From<SlackBlockPlainTextInputElement> for SlackInputBlockElement {
    fn from(element: SlackBlockPlainTextInputElement) -> Self {
        SlackInputBlockElement::PlainTextInput(element)
    }
}

impl From<SlackBlockPlainTextInputElement> for SlackActionBlockElement {
    fn from(element: SlackBlockPlainTextInputElement) -> Self {
        SlackActionBlockElement::PlainTextInput(element)
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockNumberInputElement {
    pub action_id: SlackActionId,
    pub is_decimal_allowed: bool,
    pub focus_on_load: Option<bool>,
    pub placeholder: Option<SlackBlockPlainTextOnly>,
    pub initial_value: Option<String>,
    pub min_value: Option<String>,
    pub max_value: Option<String>,
}

impl From<SlackBlockNumberInputElement> for SlackSectionBlockElement {
    fn from(element: SlackBlockNumberInputElement) -> Self {
        SlackSectionBlockElement::NumberInput(element)
    }
}

impl From<SlackBlockNumberInputElement> for SlackInputBlockElement {
    fn from(element: SlackBlockNumberInputElement) -> Self {
        SlackInputBlockElement::NumberInput(element)
    }
}

impl From<SlackBlockNumberInputElement> for SlackActionBlockElement {
    fn from(element: SlackBlockNumberInputElement) -> Self {
        SlackActionBlockElement::NumberInput(element)
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockUrlInputElement {
    pub action_id: SlackActionId,
    pub placeholder: Option<SlackBlockPlainTextOnly>,
    pub initial_value: Option<String>,
}

impl From<SlackBlockUrlInputElement> for SlackSectionBlockElement {
    fn from(element: SlackBlockUrlInputElement) -> Self {
        SlackSectionBlockElement::UrlInput(element)
    }
}

impl From<SlackBlockUrlInputElement> for SlackInputBlockElement {
    fn from(element: SlackBlockUrlInputElement) -> Self {
        SlackInputBlockElement::UrlInput(element)
    }
}

impl From<SlackBlockUrlInputElement> for SlackActionBlockElement {
    fn from(element: SlackBlockUrlInputElement) -> Self {
        SlackActionBlockElement::UrlInput(element)
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockEmailInputElement {
    pub action_id: SlackActionId,
    pub focus_on_load: Option<bool>,
    pub placeholder: Option<SlackBlockPlainTextOnly>,
    pub initial_value: Option<EmailAddress>,
}

impl From<SlackBlockEmailInputElement> for SlackInputBlockElement {
    fn from(element: SlackBlockEmailInputElement) -> Self {
        SlackInputBlockElement::EmailInput(element)
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockRadioButtonsElement {
    pub action_id: SlackActionId,
    pub options: Vec<SlackBlockChoiceItem<SlackBlockText>>,
    pub initial_option: Option<SlackBlockChoiceItem<SlackBlockText>>,
    pub confirm: Option<SlackBlockConfirmItem>,
    pub focus_on_load: Option<bool>,
}

impl From<SlackBlockRadioButtonsElement> for SlackSectionBlockElement {
    fn from(element: SlackBlockRadioButtonsElement) -> Self {
        SlackSectionBlockElement::RadioButtons(element)
    }
}

impl From<SlackBlockRadioButtonsElement> for SlackInputBlockElement {
    fn from(element: SlackBlockRadioButtonsElement) -> Self {
        SlackInputBlockElement::RadioButtons(element)
    }
}

impl From<SlackBlockRadioButtonsElement> for SlackActionBlockElement {
    fn from(element: SlackBlockRadioButtonsElement) -> Self {
        SlackActionBlockElement::RadioButtons(element)
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockCheckboxesElement {
    pub action_id: SlackActionId,
    pub options: Vec<SlackBlockChoiceItem<SlackBlockText>>,
    pub initial_options: Option<Vec<SlackBlockChoiceItem<SlackBlockText>>>,
    pub confirm: Option<SlackBlockConfirmItem>,
    pub focus_on_load: Option<bool>,
}

impl From<SlackBlockCheckboxesElement> for SlackSectionBlockElement {
    fn from(element: SlackBlockCheckboxesElement) -> Self {
        SlackSectionBlockElement::Checkboxes(element)
    }
}

impl From<SlackBlockCheckboxesElement> for SlackInputBlockElement {
    fn from(element: SlackBlockCheckboxesElement) -> Self {
        SlackInputBlockElement::Checkboxes(element)
    }
}

impl From<SlackBlockCheckboxesElement> for SlackActionBlockElement {
    fn from(element: SlackBlockCheckboxesElement) -> Self {
        SlackActionBlockElement::Checkboxes(element)
    }
}

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

impl From<SlackBlockPlainText> for SlackBlockPlainTextOnly {
    fn from(pt: SlackBlockPlainText) -> Self {
        SlackBlockPlainTextOnly { value: pt }
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

impl From<SlackBlockPlainText> for SlackContextBlockElement {
    fn from(text: SlackBlockPlainText) -> Self {
        SlackContextBlockElement::Plain(text)
    }
}

impl From<SlackBlockMarkDownText> for SlackContextBlockElement {
    fn from(text: SlackBlockMarkDownText) -> Self {
        SlackContextBlockElement::MarkDown(text)
    }
}

impl From<SlackBlockPlainTextOnly> for SlackBlockText {
    fn from(text: SlackBlockPlainTextOnly) -> Self {
        SlackBlockText::Plain(text.value)
    }
}

impl From<String> for SlackBlockPlainTextOnly {
    fn from(value: String) -> Self {
        SlackBlockPlainTextOnly {
            value: value.into(),
        }
    }
}

impl From<&str> for SlackBlockPlainTextOnly {
    fn from(value: &str) -> Self {
        SlackBlockPlainTextOnly {
            value: value.into(),
        }
    }
}

/**
 * https://api.slack.com/reference/block-kit/blocks#video
 */
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackVideoBlock {
    pub alt_text: String,
    pub author_name: Option<String>,
    pub block_id: Option<SlackBlockId>,
    pub description: Option<SlackBlockPlainTextOnly>,
    pub provider_icon_url: Option<Url>,
    pub provider_name: Option<String>,
    pub title: SlackBlockPlainTextOnly,
    pub title_url: Option<Url>,
    pub thumbnail_url: Url,
    pub video_url: Url,
}

impl From<SlackVideoBlock> for SlackBlock {
    fn from(block: SlackVideoBlock) -> Self {
        SlackBlock::Video(block)
    }
}

/**
 * https://api.slack.com/reference/block-kit/blocks#markdown
 */
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackMarkdownBlock {
    pub block_id: Option<SlackBlockId>,
    pub text: String,
}

impl From<SlackMarkdownBlock> for SlackBlock {
    fn from(block: SlackMarkdownBlock) -> Self {
        SlackBlock::Markdown(block)
    }
}

/**
 * https://api.slack.com/reference/block-kit/blocks#rich_text
 */
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackRichTextBlock {
    pub block_id: Option<SlackBlockId>,
    pub elements: Vec<SlackRichTextElement>,
}

impl From<SlackRichTextBlock> for SlackBlock {
    fn from(block: SlackRichTextBlock) -> Self {
        SlackBlock::RichText(block)
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SlackRichTextElement {
    #[serde(rename = "rich_text_section")]
    Section(SlackRichTextSection),
    #[serde(rename = "rich_text_list")]
    List(SlackRichTextList),
    #[serde(rename = "rich_text_preformatted")]
    Preformatted(SlackRichTextPreformatted),
    #[serde(rename = "rich_text_quote")]
    Quote(SlackRichTextQuote),
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackRichTextSection {
    pub elements: Vec<SlackRichTextInlineElement>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackRichTextList {
    pub style: SlackRichTextListStyle,
    pub elements: Vec<SlackRichTextSection>,
    pub indent: Option<u64>,
    pub offset: Option<u64>,
    pub border: Option<u64>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SlackRichTextListStyle {
    Bullet,
    Ordered,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackRichTextPreformatted {
    pub elements: Vec<SlackRichTextInlineElement>,
    pub border: Option<u64>,
    pub language: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackRichTextQuote {
    pub elements: Vec<SlackRichTextInlineElement>,
    pub border: Option<u64>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SlackRichTextInlineElement {
    #[serde(rename = "text")]
    Text(SlackRichTextText),
    #[serde(rename = "link")]
    Link(SlackRichTextLink),
    #[serde(rename = "user")]
    User(SlackRichTextUser),
    #[serde(rename = "channel")]
    Channel(SlackRichTextChannel),
    #[serde(rename = "usergroup")]
    UserGroup(SlackRichTextUserGroup),
    #[serde(rename = "emoji")]
    Emoji(SlackRichTextEmoji),
    #[serde(rename = "date")]
    Date(SlackRichTextDate),
    #[serde(rename = "broadcast")]
    Broadcast(SlackRichTextBroadcast),
    #[serde(rename = "color")]
    Color(SlackRichTextColor),
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackRichTextStyle {
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub strike: Option<bool>,
    pub code: Option<bool>,
    pub underline: Option<bool>,
    pub highlight: Option<bool>,
    pub client_highlight: Option<bool>,
    pub unlink: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackRichTextText {
    pub text: String,
    pub style: Option<SlackRichTextStyle>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackRichTextLink {
    pub url: String,
    pub text: Option<String>,
    #[serde(rename = "unsafe")]
    pub unsafe_: Option<bool>,
    pub style: Option<SlackRichTextStyle>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackRichTextUser {
    pub user_id: SlackUserId,
    pub style: Option<SlackRichTextStyle>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackRichTextChannel {
    pub channel_id: SlackChannelId,
    pub style: Option<SlackRichTextStyle>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackRichTextUserGroup {
    pub usergroup_id: SlackUserGroupId,
    pub style: Option<SlackRichTextStyle>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackRichTextEmoji {
    pub name: SlackEmojiName,
    pub unicode: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackRichTextDate {
    pub timestamp: i64,
    pub format: String,
    pub fallback: Option<String>,
    pub style: Option<SlackRichTextStyle>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SlackRichTextBroadcastRange {
    Here,
    Channel,
    Everyone,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackRichTextBroadcast {
    pub range: SlackRichTextBroadcastRange,
    pub style: Option<SlackRichTextStyle>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackRichTextColor {
    pub value: String,
}

/**
 * https://api.slack.com/reference/block-kit/block-elements#rich_text_input
 */
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockRichTextInputElement {
    pub action_id: SlackActionId,
    pub initial_value: Option<SlackRichTextBlock>,
    pub focus_on_load: Option<bool>,
    pub placeholder: Option<SlackBlockPlainTextOnly>,
}

impl From<SlackBlockRichTextInputElement> for SlackInputBlockElement {
    fn from(element: SlackBlockRichTextInputElement) -> Self {
        SlackInputBlockElement::RichTextInput(element)
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SlackImageUrlOrFile {
    ImageUrl { image_url: Url },
    SlackFile { slack_file: SlackFileIdOrUrl },
}

impl SlackImageUrlOrFile {
    pub fn image_url(&self) -> Option<&Url> {
        match self {
            SlackImageUrlOrFile::ImageUrl { image_url } => Some(image_url),
            SlackImageUrlOrFile::SlackFile { slack_file } => match slack_file {
                SlackFileIdOrUrl::Url { url } => Some(url),
                _ => None,
            },
        }
    }
}

impl From<Url> for SlackImageUrlOrFile {
    fn from(value: Url) -> Self {
        SlackImageUrlOrFile::ImageUrl { image_url: value }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SlackFileIdOrUrl {
    Id { id: SlackFileId },
    Url { url: Url },
}

impl From<SlackFileId> for SlackFileIdOrUrl {
    fn from(value: SlackFileId) -> Self {
        SlackFileIdOrUrl::Id { id: value }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::blocks::SlackHomeView;

    #[test]
    fn test_conversation_filter_deserialize() -> Result<(), Box<dyn std::error::Error>> {
        let payload = include_str!("./fixtures/slack_conversations_select_with_filter.json");
        let block: SlackBlock = serde_json::from_str(payload)?;
        match block {
            SlackBlock::Section(section) => match section.accessory {
                Some(SlackSectionBlockElement::ConversationsSelect(elem)) => {
                    let filter = elem.filter.expect("filter should be present");
                    let include = filter.include.expect("include should be present");
                    assert_eq!(include.len(), 2);
                    assert_eq!(include[0], SlackConversationFilterInclude::Public);
                    assert_eq!(include[1], SlackConversationFilterInclude::Private);
                    assert_eq!(filter.exclude_external_shared_channels, Some(true));
                    assert_eq!(filter.exclude_bot_users, Some(true));
                }
                _ => panic!("Expected ConversationsSelect accessory"),
            },
            _ => panic!("Expected Section block"),
        }
        Ok(())
    }

    #[test]
    fn test_conversation_filter_serialize() -> Result<(), Box<dyn std::error::Error>> {
        let filter = SlackBlockConversationFilter::new()
            .with_include(vec![
                SlackConversationFilterInclude::Im,
                SlackConversationFilterInclude::Mpim,
            ])
            .with_exclude_bot_users(true);

        let json = serde_json::to_value(&filter)?;
        assert_eq!(
            json,
            serde_json::json!({
                "include": ["im", "mpim"],
                "exclude_bot_users": true
            })
        );
        Ok(())
    }

    #[test]
    fn test_conversation_filter_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
        let elem = SlackBlockConversationsSelectElement::new(SlackActionId("test_action".into()))
            .with_filter(
                SlackBlockConversationFilter::new()
                    .with_include(vec![SlackConversationFilterInclude::Public])
                    .with_exclude_external_shared_channels(true),
            );

        let json = serde_json::to_string(&elem)?;
        let parsed: SlackBlockConversationsSelectElement = serde_json::from_str(&json)?;
        assert_eq!(elem, parsed);
        Ok(())
    }

    #[test]
    fn test_multi_conversations_select_filter() -> Result<(), Box<dyn std::error::Error>> {
        let elem =
            SlackBlockMultiConversationsSelectElement::new(SlackActionId("multi_action".into()))
                .with_filter(
                    SlackBlockConversationFilter::new()
                        .with_include(vec![
                            SlackConversationFilterInclude::Public,
                            SlackConversationFilterInclude::Private,
                        ])
                        .with_exclude_bot_users(true),
                );

        let json = serde_json::to_string(&elem)?;
        let parsed: SlackBlockMultiConversationsSelectElement = serde_json::from_str(&json)?;
        assert_eq!(elem, parsed);
        Ok(())
    }

    #[test]
    fn test_conversation_filter_none_omitted() -> Result<(), Box<dyn std::error::Error>> {
        let elem = SlackBlockConversationsSelectElement::new(SlackActionId("no_filter".into()));

        let json = serde_json::to_value(&elem)?;
        assert!(json.get("filter").is_none());
        Ok(())
    }

    #[test]
    fn test_slack_image_block_deserialize() -> Result<(), Box<dyn std::error::Error>> {
        let payload = include_str!("./fixtures/slack_image_blocks.json");
        let content: SlackMessageContent = serde_json::from_str(payload)?;
        let blocks = content.blocks.expect("Blocks should not be empty");
        match blocks.first() {
            Some(SlackBlock::Section(section)) => match &section.accessory {
                Some(SlackSectionBlockElement::Image(image)) => {
                    assert_eq!(image.alt_text, "alt text for image");
                    match &image.image_url_or_file {
                        SlackImageUrlOrFile::ImageUrl { image_url } => {
                            assert_eq!(image_url.as_str(), "https://s3-media3.fl.yelpcdn.com/bphoto/c7ed05m9lC2EmA3Aruue7A/o.jpg");
                        }
                        SlackImageUrlOrFile::SlackFile { slack_file } => {
                            panic!("Expected an image URL, not a Slack file: {:?}", slack_file);
                        }
                    }
                }
                _ => panic!("Expected a section block with an image accessory"),
            },
            _ => panic!("Expected a section block"),
        }
        Ok(())
    }

    #[test]
    fn test_rich_text_block_deserialize() -> Result<(), Box<dyn std::error::Error>> {
        let payload = include_str!("./fixtures/slack_rich_text_block.json");
        let block: SlackBlock = serde_json::from_str(payload)?;

        let rich = match block {
            SlackBlock::RichText(r) => r,
            _ => panic!("Expected a RichText block"),
        };

        assert_eq!(rich.block_id, Some(SlackBlockId("test_block".into())));
        assert_eq!(rich.elements.len(), 4);

        // section
        let section = match &rich.elements[0] {
            SlackRichTextElement::Section(s) => s,
            _ => panic!("Expected a Section element"),
        };
        assert_eq!(section.elements.len(), 7);

        // bold text
        let text = match &section.elements[0] {
            SlackRichTextInlineElement::Text(t) => t,
            _ => panic!("Expected a Text element"),
        };
        assert_eq!(text.text, "Hello ");
        assert_eq!(text.style.as_ref().and_then(|s| s.bold), Some(true));

        // user
        assert!(matches!(
            &section.elements[1],
            SlackRichTextInlineElement::User(_)
        ));

        // emoji — name should deserialize as SlackEmojiName
        let emoji = match &section.elements[4] {
            SlackRichTextInlineElement::Emoji(e) => e,
            _ => panic!("Expected an Emoji element"),
        };
        assert_eq!(emoji.name, SlackEmojiName::new("wave".into()));

        // list
        let list = match &rich.elements[1] {
            SlackRichTextElement::List(l) => l,
            _ => panic!("Expected a List element"),
        };
        assert_eq!(list.style, SlackRichTextListStyle::Bullet);
        assert_eq!(list.elements.len(), 2);

        // preformatted
        assert!(matches!(
            &rich.elements[2],
            SlackRichTextElement::Preformatted(_)
        ));

        // quote
        assert!(matches!(&rich.elements[3], SlackRichTextElement::Quote(_)));

        Ok(())
    }

    #[test]
    fn test_rich_text_block_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
        let payload = include_str!("./fixtures/slack_rich_text_block.json");
        let block: SlackBlock = serde_json::from_str(payload)?;
        let serialized = serde_json::to_string(&block)?;
        let block2: SlackBlock = serde_json::from_str(&serialized)?;
        assert_eq!(block, block2);
        Ok(())
    }
}
