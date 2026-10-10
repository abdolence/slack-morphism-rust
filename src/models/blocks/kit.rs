use rsb_derive::Builder;
use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use url::Url;

use super::workflow::SlackBlockWorkflowButtonElement;
use crate::*;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Eq, Hash, Serialize, Deserialize, ValueStruct)]
pub struct SlackBlockId(pub String);

#[derive(Debug, PartialEq, Clone, Eq, Hash, Serialize, Deserialize, ValueStruct)]
pub struct SlackTaskId(pub String);

#[derive(Debug, PartialEq, Clone, Eq, Hash, Serialize, Deserialize, ValueStruct)]
pub struct SlackAccessibilityLabel(pub String);

/// Header block heading level: 1–4 = H1–H4.
#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Serialize, Deserialize, ValueStruct,
)]
pub struct SlackHeaderLevel(pub u8);

/// Blocks are internally tagged by `type`. A block whose `type` the crate does
/// not model — or whose body does not match its known `type` — lands in
/// [`SlackBlock::Unknown`] with its raw JSON instead of failing the whole
/// `Vec<SlackBlock>`. `Unknown` must stay the last variant.
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
    #[serde(rename = "table")]
    Table(SlackTableBlock),
    #[serde(rename = "task_card")]
    TaskCard(SlackTaskCardBlock),
    #[serde(rename = "alert")]
    Alert(SlackAlertBlock),
    #[serde(rename = "card")]
    Card(SlackCardBlock),
    #[serde(rename = "carousel")]
    Carousel(SlackCarouselBlock),
    #[serde(rename = "context_actions")]
    ContextActions(SlackContextActionsBlock),
    #[serde(rename = "container")]
    Container(SlackContainerBlock),
    #[serde(rename = "plan")]
    Plan(SlackPlanBlock),
    #[serde(rename = "data_table")]
    DataTable(SlackDataTableBlock),
    #[serde(rename = "data_visualization")]
    DataVisualization(SlackDataVisualizationBlock),
    #[serde(rename = "share_shortcut")]
    ShareShortcut(serde_json::Value),
    #[serde(rename = "event")]
    Event(serde_json::Value),
    #[serde(untagged)]
    Unknown(serde_json::Value),
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackSectionBlock {
    pub block_id: Option<SlackBlockId>,
    pub text: Option<SlackBlockText>,
    pub fields: Option<Vec<SlackBlockText>>,
    pub accessory: Option<SlackSectionBlockElement>,
    pub expand: Option<bool>,
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
    /// 1–4 = H1–H4; Slack renders the default size when absent.
    /// <https://docs.slack.dev/reference/block-kit/blocks/header-block>
    pub level: Option<SlackHeaderLevel>,
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
    #[serde(rename = "workflow_button")]
    WorkflowButton(SlackBlockWorkflowButtonElement),
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
    #[serde(rename = "workflow_button")]
    WorkflowButton(SlackBlockWorkflowButtonElement),
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
    #[serde(rename = "file_input")]
    FileInput(SlackBlockFileInputElement),
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

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SlackBlockButtonStyle {
    Primary,
    Danger,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockButtonElement {
    pub action_id: Option<SlackActionId>,
    pub text: SlackBlockPlainTextOnly,
    pub url: Option<Url>,
    pub value: Option<String>,
    pub style: Option<SlackBlockButtonStyle>,
    pub confirm: Option<SlackBlockConfirmItem>,
    pub accessibility_label: Option<SlackAccessibilityLabel>,
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
    pub description: Option<SlackBlockPlainTextOnly>,
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
    pub action_id: Option<SlackActionId>,
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
    pub action_id: Option<SlackActionId>,
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
    pub action_id: Option<SlackActionId>,
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
    pub action_id: Option<SlackActionId>,
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
    pub action_id: Option<SlackActionId>,
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
    pub action_id: Option<SlackActionId>,
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
    pub action_id: Option<SlackActionId>,
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
    pub action_id: Option<SlackActionId>,
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
    pub action_id: Option<SlackActionId>,
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
    pub action_id: Option<SlackActionId>,
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
    pub action_id: Option<SlackActionId>,
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
    pub action_id: Option<SlackActionId>,
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
    pub action_id: Option<SlackActionId>,
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
    pub action_id: Option<SlackActionId>,
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

/**
 * https://docs.slack.dev/reference/block-kit/composition-objects/dispatch-action-configuration-object
 */
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SlackDispatchActionTrigger {
    OnEnterPressed,
    OnCharacterEntered,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackDispatchActionConfig {
    pub trigger_actions_on: Option<Vec<SlackDispatchActionTrigger>>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockPlainTextInputElement {
    pub action_id: Option<SlackActionId>,
    pub placeholder: Option<SlackBlockPlainTextOnly>,
    pub initial_value: Option<String>,
    pub multiline: Option<bool>,
    pub min_length: Option<u64>,
    pub max_length: Option<u64>,
    pub focus_on_load: Option<bool>,
    pub dispatch_action_config: Option<SlackDispatchActionConfig>,
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
    pub action_id: Option<SlackActionId>,
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
    pub action_id: Option<SlackActionId>,
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
    pub action_id: Option<SlackActionId>,
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
    pub action_id: Option<SlackActionId>,
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
    pub action_id: Option<SlackActionId>,
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
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SlackRichTextInlineContent {
    #[serde(rename = "rich_text")]
    RichText(SlackRichTextBlock),
}

impl From<SlackRichTextBlock> for SlackRichTextInlineContent {
    fn from(block: SlackRichTextBlock) -> Self {
        SlackRichTextInlineContent::RichText(block)
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

impl From<SlackRichTextSection> for SlackRichTextElement {
    fn from(element: SlackRichTextSection) -> Self {
        SlackRichTextElement::Section(element)
    }
}

impl From<SlackRichTextList> for SlackRichTextElement {
    fn from(list: SlackRichTextList) -> Self {
        SlackRichTextElement::List(list)
    }
}

impl From<SlackRichTextPreformatted> for SlackRichTextElement {
    fn from(element: SlackRichTextPreformatted) -> Self {
        SlackRichTextElement::Preformatted(element)
    }
}

impl From<SlackRichTextQuote> for SlackRichTextElement {
    fn from(element: SlackRichTextQuote) -> Self {
        SlackRichTextElement::Quote(element)
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackRichTextSection {
    pub elements: Vec<SlackRichTextInlineElement>,
}

/// A bare string becomes a section holding a single unstyled text run.
impl From<&str> for SlackRichTextSection {
    fn from(value: &str) -> Self {
        SlackRichTextSection::new(vec![value.into()])
    }
}

impl From<String> for SlackRichTextSection {
    fn from(value: String) -> Self {
        SlackRichTextSection::new(vec![value.into()])
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackRichTextList {
    pub style: SlackRichTextListStyle,
    pub elements: Vec<SlackRichTextListElement>,
    pub indent: Option<u64>,
    pub offset: Option<u64>,
    pub border: Option<u64>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SlackRichTextListElement {
    #[serde(rename = "rich_text_section")]
    Section(SlackRichTextSection),
}

impl From<SlackRichTextSection> for SlackRichTextListElement {
    fn from(element: SlackRichTextSection) -> Self {
        SlackRichTextListElement::Section(element)
    }
}

/// A bare string becomes a single-run section, the same as `SlackRichTextSection::from`.
impl From<&str> for SlackRichTextListElement {
    fn from(value: &str) -> Self {
        SlackRichTextListElement::Section(value.into())
    }
}

impl From<String> for SlackRichTextListElement {
    fn from(value: String) -> Self {
        SlackRichTextListElement::Section(value.into())
    }
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
    #[serde(rename = "message_mention")]
    MessageMention(SlackRichTextMessageMention),
    #[serde(untagged)]
    Unknown(serde_json::Value),
}

/// A bare string becomes an unstyled text run.
impl From<&str> for SlackRichTextInlineElement {
    fn from(value: &str) -> Self {
        SlackRichTextInlineElement::Text(SlackRichTextText::new(value.to_string()))
    }
}

impl From<String> for SlackRichTextInlineElement {
    fn from(value: String) -> Self {
        SlackRichTextInlineElement::Text(SlackRichTextText::new(value))
    }
}

impl From<SlackRichTextText> for SlackRichTextInlineElement {
    fn from(element: SlackRichTextText) -> Self {
        SlackRichTextInlineElement::Text(element)
    }
}

impl From<SlackRichTextLink> for SlackRichTextInlineElement {
    fn from(element: SlackRichTextLink) -> Self {
        SlackRichTextInlineElement::Link(element)
    }
}

impl From<SlackRichTextUser> for SlackRichTextInlineElement {
    fn from(element: SlackRichTextUser) -> Self {
        SlackRichTextInlineElement::User(element)
    }
}

impl From<SlackRichTextChannel> for SlackRichTextInlineElement {
    fn from(element: SlackRichTextChannel) -> Self {
        SlackRichTextInlineElement::Channel(element)
    }
}

impl From<SlackRichTextUserGroup> for SlackRichTextInlineElement {
    fn from(element: SlackRichTextUserGroup) -> Self {
        SlackRichTextInlineElement::UserGroup(element)
    }
}

impl From<SlackRichTextEmoji> for SlackRichTextInlineElement {
    fn from(element: SlackRichTextEmoji) -> Self {
        SlackRichTextInlineElement::Emoji(element)
    }
}

impl From<SlackRichTextDate> for SlackRichTextInlineElement {
    fn from(element: SlackRichTextDate) -> Self {
        SlackRichTextInlineElement::Date(element)
    }
}

impl From<SlackRichTextBroadcast> for SlackRichTextInlineElement {
    fn from(element: SlackRichTextBroadcast) -> Self {
        SlackRichTextInlineElement::Broadcast(element)
    }
}

impl From<SlackRichTextColor> for SlackRichTextInlineElement {
    fn from(element: SlackRichTextColor) -> Self {
        SlackRichTextInlineElement::Color(element)
    }
}

impl From<SlackRichTextMessageMention> for SlackRichTextInlineElement {
    fn from(element: SlackRichTextMessageMention) -> Self {
        SlackRichTextInlineElement::MessageMention(element)
    }
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

impl SlackRichTextText {
    /// Sets the bold flag, preserving any other flags already set on `style`.
    pub fn bold(mut self) -> Self {
        self.style.get_or_insert_with(SlackRichTextStyle::new).bold = Some(true);
        self
    }

    /// Sets the italic flag, preserving any other flags already set on `style`.
    pub fn italic(mut self) -> Self {
        self.style
            .get_or_insert_with(SlackRichTextStyle::new)
            .italic = Some(true);
        self
    }

    /// Sets the strike flag, preserving any other flags already set on `style`.
    pub fn strike(mut self) -> Self {
        self.style
            .get_or_insert_with(SlackRichTextStyle::new)
            .strike = Some(true);
        self
    }

    /// Sets the code flag, preserving any other flags already set on `style`.
    pub fn code(mut self) -> Self {
        self.style.get_or_insert_with(SlackRichTextStyle::new).code = Some(true);
        self
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackRichTextLink {
    pub url: SlackRelaxedUrl,
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
    pub timestamp: SlackDateTime,
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

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackRichTextMessageMention {
    pub url: SlackRelaxedUrl,
    pub text: Option<String>,
    pub channel_id: Option<SlackChannelId>,
    pub author_id: Option<SlackUserId>,
    pub message_ts: Option<SlackTs>,
    pub thread_ts: Option<SlackTs>,
    pub style: Option<SlackRichTextStyle>,
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

/**
 * https://docs.slack.dev/reference/block-kit/blocks/table-block
 */
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackTableBlock {
    pub block_id: Option<SlackBlockId>,
    pub rows: Vec<Vec<SlackTableCell>>,
    pub column_settings: Option<Vec<SlackTableColumnSetting>>,
}

impl From<SlackTableBlock> for SlackBlock {
    fn from(block: SlackTableBlock) -> Self {
        SlackBlock::Table(block)
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SlackTableCell {
    #[serde(rename = "raw_text")]
    RawText(SlackTableRawTextCell),
    #[serde(rename = "rich_text")]
    RichText(SlackTableRichTextCell),
}

/// A bare string becomes a raw-text cell.
impl From<&str> for SlackTableCell {
    fn from(value: &str) -> Self {
        SlackTableCell::RawText(SlackTableRawTextCell::new(value.to_string()))
    }
}

impl From<String> for SlackTableCell {
    fn from(value: String) -> Self {
        SlackTableCell::RawText(SlackTableRawTextCell::new(value))
    }
}

impl From<SlackTableRawTextCell> for SlackTableCell {
    fn from(cell: SlackTableRawTextCell) -> Self {
        SlackTableCell::RawText(cell)
    }
}

impl From<SlackTableRichTextCell> for SlackTableCell {
    fn from(cell: SlackTableRichTextCell) -> Self {
        SlackTableCell::RichText(cell)
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackTableRawTextCell {
    pub text: String,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackTableRichTextCell {
    pub elements: Vec<SlackRichTextElement>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackTableColumnSetting {
    pub align: Option<SlackTableColumnAlign>,
    pub is_wrapped: Option<bool>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SlackTableColumnAlign {
    Left,
    Center,
    Right,
}

/**
 * https://docs.slack.dev/reference/block-kit/blocks/task-card-block
 */
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackTaskCardBlock {
    pub task_id: SlackTaskId,
    pub title: String,
    pub block_id: Option<SlackBlockId>,
    pub status: Option<SlackTaskCardStatus>,
    pub icon: Option<SlackTaskCardIcon>,
    pub hide_title: Option<bool>,
    #[serde(rename = "details")]
    pub details: Option<SlackRichTextInlineContent>,
    #[serde(rename = "output")]
    pub output: Option<SlackRichTextInlineContent>,
    pub sources: Option<Vec<SlackTaskCardSource>>,
}

impl From<SlackTaskCardBlock> for SlackBlock {
    fn from(block: SlackTaskCardBlock) -> Self {
        SlackBlock::TaskCard(block)
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SlackTaskCardStatus {
    Pending,
    InProgress,
    Complete,
    Error,
}

/// Icon shown on a task card, serialised as `{"type":"icon","name":"<icon name>"}`.
/// `name` is an icon name, not a URL.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
#[serde(tag = "type", rename = "icon")]
pub struct SlackTaskCardIcon {
    pub name: String,
}

/**
 * https://docs.slack.dev/reference/block-kit/block-elements/url-source-element
 */
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackUrlSourceElement {
    pub url: Url,
    pub text: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SlackTaskCardSource {
    #[serde(rename = "url")]
    Url(SlackUrlSourceElement),
}

impl From<SlackUrlSourceElement> for SlackTaskCardSource {
    fn from(element: SlackUrlSourceElement) -> Self {
        SlackTaskCardSource::Url(element)
    }
}

/**
 * https://docs.slack.dev/reference/block-kit/block-elements/file-input-element
 */
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockFileInputElement {
    pub action_id: Option<SlackActionId>,
    pub filetypes: Option<Vec<String>>,
    pub max_files: Option<u64>,
}

impl From<SlackBlockFileInputElement> for SlackInputBlockElement {
    fn from(element: SlackBlockFileInputElement) -> Self {
        SlackInputBlockElement::FileInput(element)
    }
}

/**
 * https://docs.slack.dev/reference/block-kit/blocks/alert-block
 */
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SlackAlertLevel {
    Warning,
    Error,
    Info,
    Success,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackAlertBlock {
    pub block_id: Option<SlackBlockId>,
    pub text: SlackBlockText,
    pub level: Option<SlackAlertLevel>,
}

impl From<SlackAlertBlock> for SlackBlock {
    fn from(block: SlackAlertBlock) -> Self {
        SlackBlock::Alert(block)
    }
}

/**
 * https://docs.slack.dev/reference/block-kit/blocks/card-block
 */
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SlackCardImageElement {
    #[serde(rename = "image")]
    Image(SlackBlockImageElement),
}

impl From<SlackBlockImageElement> for SlackCardImageElement {
    fn from(element: SlackBlockImageElement) -> Self {
        SlackCardImageElement::Image(element)
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SlackCardActionBlockElement {
    #[serde(rename = "button")]
    Button(SlackBlockButtonElement),
}

impl From<SlackBlockButtonElement> for SlackCardActionBlockElement {
    fn from(element: SlackBlockButtonElement) -> Self {
        SlackCardActionBlockElement::Button(element)
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackCardBlock {
    pub block_id: Option<SlackBlockId>,
    pub title: Option<SlackBlockText>,
    pub subtitle: Option<SlackBlockText>,
    pub body: Option<SlackBlockText>,
    pub hero_image: Option<SlackCardImageElement>,
    pub icon: Option<SlackCardImageElement>,
    pub actions: Option<Vec<SlackCardActionBlockElement>>,
}

impl From<SlackCardBlock> for SlackBlock {
    fn from(block: SlackCardBlock) -> Self {
        SlackBlock::Card(block)
    }
}

/**
 * https://docs.slack.dev/reference/block-kit/blocks/carousel-block
 */
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackCarouselBlock {
    pub block_id: Option<SlackBlockId>,
    pub elements: Vec<SlackBlock>,
}

impl From<SlackCarouselBlock> for SlackBlock {
    fn from(block: SlackCarouselBlock) -> Self {
        SlackBlock::Carousel(block)
    }
}

/**
 * https://docs.slack.dev/reference/block-kit/blocks/context-actions-block
 * https://docs.slack.dev/reference/block-kit/block-elements/feedback-buttons-element
 * https://docs.slack.dev/reference/block-kit/block-elements/icon-button-element
 */
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackFeedbackButtonItem {
    pub action_id: SlackActionId,
    pub value: String,
    pub text: SlackBlockPlainTextOnly,
    pub confirm: Option<SlackBlockConfirmItem>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockFeedbackButtonsElement {
    pub action_id: Option<SlackActionId>,
    pub positive: SlackFeedbackButtonItem,
    pub negative: SlackFeedbackButtonItem,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockIconButtonElement {
    pub action_id: Option<SlackActionId>,
    pub icon: String,
    pub text: SlackBlockPlainTextOnly,
    pub value: Option<String>,
    pub confirm: Option<SlackBlockConfirmItem>,
    pub accessibility_label: Option<SlackAccessibilityLabel>,
    pub visible_to_user_ids: Option<Vec<SlackUserId>>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SlackContextActionBlockElement {
    #[serde(rename = "feedback_buttons")]
    FeedbackButtons(SlackBlockFeedbackButtonsElement),
    #[serde(rename = "icon_button")]
    IconButton(SlackBlockIconButtonElement),
}

impl From<SlackBlockFeedbackButtonsElement> for SlackContextActionBlockElement {
    fn from(element: SlackBlockFeedbackButtonsElement) -> Self {
        SlackContextActionBlockElement::FeedbackButtons(element)
    }
}

impl From<SlackBlockIconButtonElement> for SlackContextActionBlockElement {
    fn from(element: SlackBlockIconButtonElement) -> Self {
        SlackContextActionBlockElement::IconButton(element)
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackContextActionsBlock {
    pub block_id: Option<SlackBlockId>,
    pub elements: Vec<SlackContextActionBlockElement>,
}

impl From<SlackContextActionsBlock> for SlackBlock {
    fn from(block: SlackContextActionsBlock) -> Self {
        SlackBlock::ContextActions(block)
    }
}

/**
 * https://docs.slack.dev/reference/block-kit/blocks/container-block
 *
 * One of `title` / `rich_text_title` is required; `rich_text_title` wins when
 * both are set. `child_blocks` holds at most 10 blocks. The docs list
 * `actions, context, divider, file, header, image, input, rich_text, section,
 * table, video` as supported children while the live validator accepts
 * `divider, image, section, contact_card, callout, canvas_table, actions,
 * video, header, context, table, layout`; neither set is enforced here.
 */
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackContainerBlock {
    pub block_id: Option<SlackBlockId>,
    pub title: Option<SlackBlockPlainTextOnly>,
    pub rich_text_title: Option<SlackRichTextInlineContent>,
    pub subtitle: Option<SlackBlockText>,
    pub child_blocks: Vec<SlackBlock>,
    pub width: Option<SlackContainerWidth>,
    pub icon: Option<SlackCardImageElement>,
    pub is_collapsible: Option<bool>,
    pub default_collapsed: Option<bool>,
    pub has_header_divider: Option<bool>,
}

impl From<SlackContainerBlock> for SlackBlock {
    fn from(block: SlackContainerBlock) -> Self {
        SlackBlock::Container(block)
    }
}

/// `narrow`, `standard` (default) and `wide` are platform-constrained widths;
/// `full` fills the available space.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SlackContainerWidth {
    Narrow,
    Standard,
    Wide,
    Full,
}

/**
 * https://docs.slack.dev/reference/block-kit/blocks/plan-block
 *
 * `tasks` holds up to 50 task-card objects without a `type` tag
 * (`SlackTaskCardBlock` serialises without one); every `task_id` in a plan
 * must be unique.
 */
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackPlanBlock {
    pub title: String,
    pub tasks: Vec<SlackTaskCardBlock>,
    pub block_id: Option<SlackBlockId>,
}

impl From<SlackPlanBlock> for SlackBlock {
    fn from(block: SlackPlanBlock) -> Self {
        SlackBlock::Plan(block)
    }
}

/**
 * https://docs.slack.dev/reference/block-kit/blocks/data-table-block
 *
 * The first row is the header and may not hold `rich_text` cells. 2–201 rows,
 * 1–20 columns, every row the same length; `page_size` is 1–100 (default 5).
 */
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackDataTableBlock {
    pub caption: String,
    pub rows: Vec<Vec<SlackDataTableCell>>,
    pub block_id: Option<SlackBlockId>,
    pub page_size: Option<u32>,
    pub row_header_column_index: Option<u32>,
}

impl From<SlackDataTableBlock> for SlackBlock {
    fn from(block: SlackDataTableBlock) -> Self {
        SlackBlock::DataTable(block)
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SlackDataTableCell {
    #[serde(rename = "raw_text")]
    RawText(SlackTableRawTextCell),
    #[serde(rename = "raw_number")]
    RawNumber(SlackDataTableRawNumberCell),
    #[serde(rename = "rich_text")]
    RichText(SlackTableRichTextCell),
    #[serde(rename = "action_cell")]
    ActionCell(SlackDataTableActionCell),
    #[serde(untagged)]
    Unknown(serde_json::Value),
}

/// A bare string becomes a raw-text cell.
impl From<&str> for SlackDataTableCell {
    fn from(value: &str) -> Self {
        SlackDataTableCell::RawText(SlackTableRawTextCell::new(value.to_string()))
    }
}

impl From<String> for SlackDataTableCell {
    fn from(value: String) -> Self {
        SlackDataTableCell::RawText(SlackTableRawTextCell::new(value))
    }
}

impl From<SlackTableRawTextCell> for SlackDataTableCell {
    fn from(cell: SlackTableRawTextCell) -> Self {
        SlackDataTableCell::RawText(cell)
    }
}

impl From<SlackDataTableRawNumberCell> for SlackDataTableCell {
    fn from(cell: SlackDataTableRawNumberCell) -> Self {
        SlackDataTableCell::RawNumber(cell)
    }
}

impl From<SlackTableRichTextCell> for SlackDataTableCell {
    fn from(cell: SlackTableRichTextCell) -> Self {
        SlackDataTableCell::RichText(cell)
    }
}

impl From<SlackDataTableActionCell> for SlackDataTableCell {
    fn from(cell: SlackDataTableActionCell) -> Self {
        SlackDataTableCell::ActionCell(cell)
    }
}

/// `value` sorts the column numerically when every cell in it is a
/// `raw_number`; `text` is what is displayed.
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackDataTableRawNumberCell {
    pub value: serde_json::Number,
    pub text: String,
}

/// A button in a cell; `fallback` is a `raw_text` or `raw_number` cell shown
/// by clients that do not support action cells.
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackDataTableActionCell {
    pub element: SlackDataTableActionElement,
    pub fallback: Option<Box<SlackDataTableCell>>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SlackDataTableActionElement {
    #[serde(rename = "button")]
    Button(SlackBlockButtonElement),
}

impl From<SlackBlockButtonElement> for SlackDataTableActionElement {
    fn from(element: SlackBlockButtonElement) -> Self {
        SlackDataTableActionElement::Button(element)
    }
}

/**
 * https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block
 *
 * `title` is at most 50 characters; a message may hold at most two of these.
 */
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackDataVisualizationBlock {
    pub title: String,
    pub chart: SlackChart,
    pub block_id: Option<SlackBlockId>,
}

impl From<SlackDataVisualizationBlock> for SlackBlock {
    fn from(block: SlackDataVisualizationBlock) -> Self {
        SlackBlock::DataVisualization(block)
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SlackChart {
    Pie(SlackPieChart),
    Bar(SlackSeriesChart),
    Area(SlackSeriesChart),
    Line(SlackSeriesChart),
}

/// 1–12 segments; each renders as `value / sum(values)`.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackPieChart {
    pub segments: Vec<SlackChartSegment>,
}

/// Shared by `bar`, `area` and `line`: 1–12 series, each with exactly one data
/// point per entry of `axis_config.categories`, and unique series names.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackSeriesChart {
    pub series: Vec<SlackChartDataSeries>,
    pub axis_config: SlackChartAxisConfig,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackChartSegment {
    pub label: String,
    pub value: serde_json::Number,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackChartDataSeries {
    pub name: String,
    pub data: Vec<SlackChartDataPoint>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackChartDataPoint {
    pub label: String,
    pub value: serde_json::Number,
}

/// `categories` defines the valid data-point labels and their x-axis order.
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackChartAxisConfig {
    pub categories: Vec<String>,
    pub x_label: Option<String>,
    pub y_label: Option<String>,
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
        let elem = SlackBlockConversationsSelectElement::new()
            .with_action_id(SlackActionId("test_action".into()))
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
        let elem = SlackBlockMultiConversationsSelectElement::new()
            .with_action_id(SlackActionId("multi_action".into()))
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
        let elem = SlackBlockConversationsSelectElement::new()
            .with_action_id(SlackActionId("no_filter".into()));

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

        // list items are SlackRichTextElement::Section
        assert!(matches!(
            &list.elements[0],
            SlackRichTextListElement::Section(_)
        ));
        assert!(matches!(
            &list.elements[1],
            SlackRichTextListElement::Section(_)
        ));

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

    #[test]
    fn test_slack_table_block_deserialize() -> Result<(), Box<dyn std::error::Error>> {
        let payload = include_str!("./fixtures/slack_table_block.json");
        let block: SlackBlock = serde_json::from_str(payload)?;

        let table = match block {
            SlackBlock::Table(t) => t,
            _ => panic!("Expected a Table block"),
        };

        assert_eq!(table.block_id, Some(SlackBlockId("table_block_1".into())));
        assert_eq!(table.rows.len(), 2);
        assert_eq!(table.rows[0].len(), 2);

        // first row, first cell is raw_text
        match &table.rows[0][0] {
            SlackTableCell::RawText(c) => assert_eq!(c.text, "Header A"),
            _ => panic!("Expected RawText cell"),
        }

        // second row, second cell is rich_text
        match &table.rows[1][1] {
            SlackTableCell::RichText(c) => assert_eq!(c.elements.len(), 1),
            _ => panic!("Expected RichText cell"),
        }

        let settings = table
            .column_settings
            .expect("column_settings should be present");
        assert_eq!(settings.len(), 2);
        assert_eq!(settings[0].is_wrapped, Some(true));
        assert_eq!(settings[1].align, Some(SlackTableColumnAlign::Right));

        Ok(())
    }

    #[test]
    fn test_slack_table_block_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
        let payload = include_str!("./fixtures/slack_table_block.json");
        let block: SlackBlock = serde_json::from_str(payload)?;
        let serialized = serde_json::to_string(&block)?;
        let block2: SlackBlock = serde_json::from_str(&serialized)?;
        assert_eq!(block, block2);
        Ok(())
    }

    #[test]
    fn test_slack_task_card_block_deserialize() -> Result<(), Box<dyn std::error::Error>> {
        let payload = include_str!("./fixtures/slack_task_card_block.json");
        let block: SlackBlock = serde_json::from_str(payload)?;

        let task_card = match block {
            SlackBlock::TaskCard(t) => t,
            _ => panic!("Expected a TaskCard block"),
        };

        assert_eq!(task_card.task_id, SlackTaskId("task_1".into()));
        assert_eq!(task_card.title, "Fetching weather data");
        assert_eq!(
            task_card.block_id,
            Some(SlackBlockId("task_card_block_1".into()))
        );
        assert_eq!(task_card.status, Some(SlackTaskCardStatus::InProgress));

        let output = task_card.output.expect("output should be present");
        let SlackRichTextInlineContent::RichText(output_block) = output;
        assert_eq!(output_block.elements.len(), 1);

        let sources = task_card.sources.expect("sources should be present");
        assert_eq!(sources.len(), 2);
        match &sources[0] {
            SlackTaskCardSource::Url(u) => assert_eq!(u.text, "weather.com"),
        }

        Ok(())
    }

    #[test]
    fn test_slack_task_card_block_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
        let payload = include_str!("./fixtures/slack_task_card_block.json");
        let block: SlackBlock = serde_json::from_str(payload)?;
        let serialized = serde_json::to_string(&block)?;
        let block2: SlackBlock = serde_json::from_str(&serialized)?;
        assert_eq!(block, block2);
        Ok(())
    }

    #[test]
    fn test_slack_task_card_block_serializes_icon_and_hide_title(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let task_card = SlackTaskCardBlock::new(SlackTaskId("task_1".into()), "Title".into())
            .with_icon(SlackTaskCardIcon::new("check".into()))
            .with_hide_title(true);
        let json = serde_json::to_value(&task_card)?;
        assert_eq!(
            json["icon"],
            serde_json::json!({"type": "icon", "name": "check"})
        );
        assert_eq!(json["hide_title"], serde_json::json!(true));
        Ok(())
    }

    #[test]
    fn test_slack_alert_block_deserialize() -> Result<(), Box<dyn std::error::Error>> {
        let payload = include_str!("./fixtures/slack_alert_block.json");
        let block: SlackBlock = serde_json::from_str(payload)?;
        match block {
            SlackBlock::Alert(alert) => {
                assert_eq!(alert.level, Some(SlackAlertLevel::Warning));
            }
            _ => panic!("Expected Alert block"),
        }
        Ok(())
    }

    #[test]
    fn test_slack_alert_block_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
        let payload = include_str!("./fixtures/slack_alert_block.json");
        let block: SlackBlock = serde_json::from_str(payload)?;
        let serialized = serde_json::to_string(&block)?;
        let block2: SlackBlock = serde_json::from_str(&serialized)?;
        assert_eq!(block, block2);
        Ok(())
    }

    #[test]
    fn test_slack_card_block_deserialize() -> Result<(), Box<dyn std::error::Error>> {
        let payload = include_str!("./fixtures/slack_card_block.json");
        let block: SlackBlock = serde_json::from_str(payload)?;
        match block {
            SlackBlock::Card(card) => {
                assert!(card.hero_image.is_some());
                assert!(card.actions.is_some());
                let actions = card.actions.unwrap();
                assert_eq!(actions.len(), 1);
                match &actions[0] {
                    SlackCardActionBlockElement::Button(btn) => {
                        assert_eq!(btn.style, Some(SlackBlockButtonStyle::Primary));
                    }
                }
            }
            _ => panic!("Expected Card block"),
        }
        Ok(())
    }

    #[test]
    fn test_slack_card_block_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
        let payload = include_str!("./fixtures/slack_card_block.json");
        let block: SlackBlock = serde_json::from_str(payload)?;
        let serialized = serde_json::to_string(&block)?;
        let block2: SlackBlock = serde_json::from_str(&serialized)?;
        assert_eq!(block, block2);
        Ok(())
    }

    #[test]
    fn test_slack_context_actions_block_deserialize() -> Result<(), Box<dyn std::error::Error>> {
        let payload = include_str!("./fixtures/slack_context_actions_block.json");
        let block: SlackBlock = serde_json::from_str(payload)?;
        match block {
            SlackBlock::ContextActions(ctx) => {
                assert_eq!(ctx.elements.len(), 1);
                match &ctx.elements[0] {
                    SlackContextActionBlockElement::IconButton(btn) => {
                        assert_eq!(btn.icon, "trash");
                    }
                    _ => panic!("Expected IconButton element"),
                }
            }
            _ => panic!("Expected ContextActions block"),
        }
        Ok(())
    }

    #[test]
    fn test_slack_context_actions_block_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
        let payload = include_str!("./fixtures/slack_context_actions_block.json");
        let block: SlackBlock = serde_json::from_str(payload)?;
        let serialized = serde_json::to_string(&block)?;
        let block2: SlackBlock = serde_json::from_str(&serialized)?;
        assert_eq!(block, block2);
        Ok(())
    }

    #[test]
    fn test_slack_workflow_button_deserialize() -> Result<(), Box<dyn std::error::Error>> {
        let payload = include_str!("./fixtures/slack_workflow_button.json");
        let block: SlackBlock = serde_json::from_str(payload)?;
        match block {
            SlackBlock::Actions(actions) => {
                assert_eq!(actions.elements.len(), 1);
                match &actions.elements[0] {
                    SlackActionBlockElement::WorkflowButton(btn) => {
                        assert_eq!(btn.style, Some(SlackBlockButtonStyle::Primary));
                        let params = btn
                            .workflow
                            .trigger
                            .customizable_input_parameters
                            .as_ref()
                            .expect("params should be present");
                        assert_eq!(params.len(), 1);
                        assert_eq!(params[0].name, "user_input");
                    }
                    _ => panic!("Expected WorkflowButton element"),
                }
            }
            _ => panic!("Expected Actions block"),
        }
        Ok(())
    }

    #[test]
    fn test_slack_workflow_button_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
        let payload = include_str!("./fixtures/slack_workflow_button.json");
        let block: SlackBlock = serde_json::from_str(payload)?;
        let serialized = serde_json::to_string(&block)?;
        let block2: SlackBlock = serde_json::from_str(&serialized)?;
        assert_eq!(block, block2);
        Ok(())
    }

    #[test]
    fn test_rich_text_message_mention_deserialize() -> Result<(), Box<dyn std::error::Error>> {
        let payload = serde_json::json!({
            "type": "rich_text",
            "block_id": "msgm1",
            "elements": [
                {
                    "type": "rich_text_section",
                    "elements": [
                        {
                            "type": "message_mention",
                            "url": "https://acme.slack.com/archives/C12345678/p1784153496441789?thread_ts=1784153496.441789&cid=C12345678",
                            "text": "a message",
                            "channel_id": "C12345678",
                            "author_id": "U12345678",
                            "message_ts": "1784153496.441789",
                            "thread_ts": "1784153496.441789"
                        }
                    ]
                }
            ]
        })
        .to_string();
        let block: SlackBlock = serde_json::from_str(&payload)?;
        match block {
            SlackBlock::RichText(rich_text) => match &rich_text.elements[0] {
                SlackRichTextElement::Section(section) => match &section.elements[0] {
                    SlackRichTextInlineElement::MessageMention(mention) => {
                        assert_eq!(mention.channel_id, Some(SlackChannelId("C12345678".into())));
                        assert_eq!(mention.author_id, Some(SlackUserId("U12345678".into())));
                        assert_eq!(
                            mention.message_ts,
                            Some(SlackTs("1784153496.441789".into()))
                        );
                    }
                    other => panic!("Expected MessageMention element, got {other:?}"),
                },
                _ => panic!("Expected Section element"),
            },
            _ => panic!("Expected RichText block"),
        }
        Ok(())
    }

    #[test]
    fn test_rich_text_unknown_inline_element_deserialize() -> Result<(), Box<dyn std::error::Error>>
    {
        let payload = serde_json::json!({
            "type": "rich_text_section",
            "elements": [
                {
                    "type": "some_future_element",
                    "foo": "bar"
                }
            ]
        })
        .to_string();
        let section: SlackRichTextElement = serde_json::from_str(&payload)?;
        match section {
            SlackRichTextElement::Section(section) => match &section.elements[0] {
                SlackRichTextInlineElement::Unknown(value) => {
                    assert_eq!(value["type"], "some_future_element");
                }
                other => panic!("Expected Unknown element, got {other:?}"),
            },
            _ => panic!("Expected Section element"),
        }
        Ok(())
    }

    #[test]
    fn rich_text_str_converts_to_text_inline_element() -> Result<(), Box<dyn std::error::Error>> {
        let element: SlackRichTextInlineElement = "hi".into();
        assert_eq!(
            serde_json::to_value(&element)?,
            serde_json::json!({"type": "text", "text": "hi"})
        );

        let owned: SlackRichTextInlineElement = "hi".to_string().into();
        assert_eq!(
            serde_json::to_value(&owned)?,
            serde_json::to_value(&element)?
        );
        Ok(())
    }

    #[test]
    fn rich_text_leaf_elements_convert_to_inline_elements() -> Result<(), Box<dyn std::error::Error>>
    {
        let text: SlackRichTextInlineElement = SlackRichTextText::new("t".to_string()).into();
        assert_eq!(
            serde_json::to_value(&text)?,
            serde_json::json!({"type": "text", "text": "t"})
        );

        let link: SlackRichTextInlineElement =
            SlackRichTextLink::new(SlackRelaxedUrl("https://example.com".into())).into();
        assert_eq!(
            serde_json::to_value(&link)?,
            serde_json::json!({"type": "link", "url": "https://example.com"})
        );

        let user: SlackRichTextInlineElement =
            SlackRichTextUser::new(SlackUserId("U1".into())).into();
        assert_eq!(
            serde_json::to_value(&user)?,
            serde_json::json!({"type": "user", "user_id": "U1"})
        );

        let channel: SlackRichTextInlineElement =
            SlackRichTextChannel::new(SlackChannelId("C1".into())).into();
        assert_eq!(
            serde_json::to_value(&channel)?,
            serde_json::json!({"type": "channel", "channel_id": "C1"})
        );

        let usergroup: SlackRichTextInlineElement =
            SlackRichTextUserGroup::new(SlackUserGroupId("G1".into())).into();
        assert_eq!(
            serde_json::to_value(&usergroup)?,
            serde_json::json!({"type": "usergroup", "usergroup_id": "G1"})
        );

        let emoji: SlackRichTextInlineElement =
            SlackRichTextEmoji::new(SlackEmojiName("wave".into())).into();
        assert_eq!(
            serde_json::to_value(&emoji)?,
            serde_json::json!({"type": "emoji", "name": "wave"})
        );

        let date: SlackRichTextInlineElement = SlackRichTextDate::new(
            SlackDateTime("2020-01-01T00:42:42Z".parse::<SlackUtcDateTime>()?),
            "{date_short}".to_string(),
        )
        .into();
        assert_eq!(
            serde_json::to_value(&date)?,
            serde_json::json!({"type": "date", "timestamp": 1_577_839_362_i64, "format": "{date_short}"})
        );

        let broadcast: SlackRichTextInlineElement =
            SlackRichTextBroadcast::new(SlackRichTextBroadcastRange::Here).into();
        assert_eq!(
            serde_json::to_value(&broadcast)?,
            serde_json::json!({"type": "broadcast", "range": "here"})
        );

        let color: SlackRichTextInlineElement =
            SlackRichTextColor::new("#ff0000".to_string()).into();
        assert_eq!(
            serde_json::to_value(&color)?,
            serde_json::json!({"type": "color", "value": "#ff0000"})
        );

        let mention: SlackRichTextInlineElement = SlackRichTextMessageMention::new(
            SlackRelaxedUrl("https://example.com/archives/C1/p1".into()),
        )
        .into();
        assert_eq!(
            serde_json::to_value(&mention)?,
            serde_json::json!({"type": "message_mention", "url": "https://example.com/archives/C1/p1"})
        );

        Ok(())
    }

    #[test]
    fn rich_text_list_accepts_str_items() -> Result<(), Box<dyn std::error::Error>> {
        let payload = include_str!("./fixtures/slack_rich_text_block.json");
        let block: SlackBlock = serde_json::from_str(payload)?;
        let expected_list = match block {
            SlackBlock::RichText(r) => match &r.elements[1] {
                SlackRichTextElement::List(l) => l.clone(),
                other => panic!("Expected List element, got {other:?}"),
            },
            _ => panic!("Expected RichText block"),
        };

        let list = SlackRichTextList::new(
            SlackRichTextListStyle::Bullet,
            vec!["Item one".into(), "Item two".into()],
        )
        .with_indent(0);

        assert_eq!(list, expected_list);
        Ok(())
    }

    #[test]
    fn rich_text_style_helpers_merge_flags() -> Result<(), Box<dyn std::error::Error>> {
        let bold = SlackRichTextText::new("hi".to_string()).bold();
        let json = serde_json::to_value(&bold)?;
        assert_eq!(json["style"], serde_json::json!({"bold": true}));

        let both = SlackRichTextText::new("hi".to_string())
            .with_style(SlackRichTextStyle::new().with_italic(true))
            .bold();
        let json2 = serde_json::to_value(&both)?;
        assert_eq!(
            json2["style"],
            serde_json::json!({"bold": true, "italic": true})
        );

        let struck = SlackRichTextText::new("hi".to_string()).strike();
        assert_eq!(
            serde_json::to_value(&struck)?["style"],
            serde_json::json!({"strike": true})
        );

        let coded = SlackRichTextText::new("hi".to_string()).code();
        assert_eq!(
            serde_json::to_value(&coded)?["style"],
            serde_json::json!({"code": true})
        );

        Ok(())
    }

    #[test]
    fn table_block_builds_from_str_cells() -> Result<(), Box<dyn std::error::Error>> {
        let payload = include_str!("./fixtures/slack_table_block.json");
        let expected: serde_json::Value = serde_json::from_str(payload)?;

        let block: SlackBlock = SlackTableBlock::new(vec![
            vec!["Header A".into(), "Header B".into()],
            vec![
                "Data 1A".into(),
                SlackTableRichTextCell::new(vec![SlackRichTextSection::new(vec![
                    SlackRichTextLink::new(SlackRelaxedUrl("https://slack.com".into()))
                        .with_text("Data 1B".to_string())
                        .into(),
                ])
                .into()])
                .into(),
            ],
        ])
        .with_block_id(SlackBlockId("table_block_1".into()))
        .with_column_settings(vec![
            SlackTableColumnSetting::new().with_is_wrapped(true),
            SlackTableColumnSetting::new().with_align(SlackTableColumnAlign::Right),
        ])
        .into();

        assert_eq!(serde_json::to_value(&block)?, expected);
        Ok(())
    }

    #[test]
    fn rich_text_block_builds_fixture_with_conversions() -> Result<(), Box<dyn std::error::Error>> {
        let payload = include_str!("./fixtures/slack_rich_text_block.json");
        let expected: serde_json::Value = serde_json::from_str(payload)?;

        let section: SlackRichTextElement = SlackRichTextSection::new(vec![
            SlackRichTextText::new("Hello ".to_string()).bold().into(),
            SlackRichTextUser::new(SlackUserId("U123ABC456".into())).into(),
            "! Check out ".into(),
            SlackRichTextLink::new(SlackRelaxedUrl("https://example.com".into()))
                .with_text("this link".to_string())
                .with_style(SlackRichTextStyle::new().with_italic(true))
                .into(),
            SlackRichTextEmoji::new(SlackEmojiName("wave".into())).into(),
            SlackRichTextChannel::new(SlackChannelId("C123ABC456".into())).into(),
            SlackRichTextBroadcast::new(SlackRichTextBroadcastRange::Here).into(),
        ])
        .into();

        let list: SlackRichTextElement = SlackRichTextList::new(
            SlackRichTextListStyle::Bullet,
            vec!["Item one".into(), "Item two".into()],
        )
        .with_indent(0)
        .into();

        let preformatted: SlackRichTextElement =
            SlackRichTextPreformatted::new(vec![SlackRichTextText::new(
                "fn main() {}\n".to_string(),
            )
            .into()])
            .with_border(1)
            .into();

        let quote: SlackRichTextElement =
            SlackRichTextQuote::new(vec![SlackRichTextText::new("A wise quote".to_string())
                .italic()
                .into()])
            .into();

        let block: SlackBlock = SlackRichTextBlock::new(vec![section, list, preformatted, quote])
            .with_block_id(SlackBlockId("test_block".into()))
            .into();

        assert_eq!(serde_json::to_value(&block)?, expected);
        Ok(())
    }

    #[test]
    fn unknown_block_type_round_trips_verbatim() -> Result<(), Box<dyn std::error::Error>> {
        let payload = serde_json::json!([
            { "type": "some_future_block", "x": 1 },
            { "type": "divider", "block_id": "d1" }
        ]);
        let blocks: Vec<SlackBlock> = serde_json::from_value(payload.clone())?;
        match &blocks[0] {
            SlackBlock::Unknown(value) => assert_eq!(value, &payload[0]),
            other => panic!("Expected Unknown block, got {other:?}"),
        }
        assert_eq!(
            blocks[1],
            SlackBlock::Divider(SlackDividerBlock::new().with_block_id("d1".into()))
        );
        assert_eq!(serde_json::to_value(&blocks)?, payload);
        Ok(())
    }

    #[test]
    fn malformed_known_block_type_falls_back_to_unknown() -> Result<(), Box<dyn std::error::Error>>
    {
        // `header` requires `text`; serde tries the untagged fallback once the typed body fails.
        let payload = serde_json::json!({ "type": "header" });
        let block: SlackBlock = serde_json::from_value(payload.clone())?;
        assert_eq!(block, SlackBlock::Unknown(payload));
        Ok(())
    }

    /// Deserialises a fixture to `SlackBlock`, re-serialises it and returns the
    /// typed block alongside the equality of both JSON values.
    fn round_trip(payload: &str) -> Result<SlackBlock, Box<dyn std::error::Error>> {
        let expected: serde_json::Value = serde_json::from_str(payload)?;
        let block: SlackBlock = serde_json::from_str(payload)?;
        assert_eq!(serde_json::to_value(&block)?, expected);
        Ok(block)
    }

    #[test]
    fn buttons_without_action_id_stay_a_typed_actions_block(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let block = round_trip(include_str!(
            "./fixtures/slack_actions_block_without_action_id.json"
        ))?;
        let SlackBlock::Actions(actions) = block else {
            panic!("Expected an Actions block, got {block:?}");
        };
        assert_eq!(actions.elements.len(), 2);
        assert!(actions
            .elements
            .iter()
            .all(|element| matches!(element, SlackActionBlockElement::Button(_))));
        Ok(())
    }

    #[test]
    fn container_block_fixture_round_trips() -> Result<(), Box<dyn std::error::Error>> {
        let block = round_trip(include_str!("./fixtures/slack_container_block.json"))?;
        let SlackBlock::Container(container) = block else {
            panic!("Expected a Container block, got {block:?}");
        };
        assert_eq!(container.is_collapsible, Some(true));
        assert_eq!(container.child_blocks.len(), 6);
        assert!(matches!(container.child_blocks[0], SlackBlock::Section(_)));
        assert!(matches!(container.child_blocks[5], SlackBlock::Actions(_)));
        Ok(())
    }

    #[test]
    fn plan_block_fixture_round_trips() -> Result<(), Box<dyn std::error::Error>> {
        let block = round_trip(include_str!("./fixtures/slack_plan_block.json"))?;
        let SlackBlock::Plan(plan) = block else {
            panic!("Expected a Plan block, got {block:?}");
        };
        assert_eq!(plan.title, "Thinking completed");
        assert_eq!(plan.tasks.len(), 3);
        assert_eq!(plan.tasks[1].status, Some(SlackTaskCardStatus::Pending));
        assert!(plan.tasks[2].output.is_some());
        Ok(())
    }

    #[test]
    fn data_table_block_fixture_round_trips() -> Result<(), Box<dyn std::error::Error>> {
        let block = round_trip(include_str!("./fixtures/slack_data_table_block.json"))?;
        let SlackBlock::DataTable(table) = block else {
            panic!("Expected a DataTable block, got {block:?}");
        };
        assert_eq!(table.caption, "A Fabulous Table");
        assert_eq!(table.rows.len(), 4);
        assert_eq!(table.rows[0][0], "Name".into());
        assert!(matches!(table.rows[1][2], SlackDataTableCell::RichText(_)));
        Ok(())
    }

    #[test]
    fn data_table_number_and_action_cells_round_trip() -> Result<(), Box<dyn std::error::Error>> {
        // `action_cell` example from the docs page plus a `raw_number` and a future cell kind.
        let payload = serde_json::json!([
            { "type": "raw_number", "value": 1.5, "text": "1.5" },
            {
                "type": "action_cell",
                "element": {
                    "type": "button",
                    "text": { "type": "plain_text", "text": "Mark done" },
                    "action_id": "mark_done",
                    "value": "task_123"
                },
                "fallback": { "type": "raw_text", "text": "Open" }
            },
            { "type": "sparkline", "points": [1, 2] }
        ]);
        let cells: Vec<SlackDataTableCell> = serde_json::from_value(payload.clone())?;
        assert!(matches!(cells[0], SlackDataTableCell::RawNumber(_)));
        let SlackDataTableCell::ActionCell(action) = &cells[1] else {
            panic!("Expected an ActionCell, got {:?}", cells[1]);
        };
        assert_eq!(action.fallback.as_deref(), Some(&"Open".into()));
        assert!(matches!(cells[2], SlackDataTableCell::Unknown(_)));
        assert_eq!(serde_json::to_value(&cells)?, payload);
        Ok(())
    }

    #[test]
    fn data_visualization_pie_fixture_round_trips() -> Result<(), Box<dyn std::error::Error>> {
        let block = round_trip(include_str!(
            "./fixtures/slack_data_visualization_pie_block.json"
        ))?;
        let SlackBlock::DataVisualization(viz) = block else {
            panic!("Expected a DataVisualization block, got {block:?}");
        };
        let SlackChart::Pie(pie) = viz.chart else {
            panic!("Expected a pie chart, got {:?}", viz.chart);
        };
        assert_eq!(pie.segments.len(), 4);
        assert_eq!(pie.segments[0].value, 45.into());
        Ok(())
    }

    #[test]
    fn data_visualization_bar_fixture_round_trips() -> Result<(), Box<dyn std::error::Error>> {
        let block = round_trip(include_str!(
            "./fixtures/slack_data_visualization_bar_block.json"
        ))?;
        let SlackBlock::DataVisualization(viz) = block else {
            panic!("Expected a DataVisualization block, got {block:?}");
        };
        let SlackChart::Bar(bar) = viz.chart else {
            panic!("Expected a bar chart, got {:?}", viz.chart);
        };
        assert_eq!(bar.series[0].data.len(), 5);
        assert_eq!(bar.axis_config.categories.len(), 5);
        assert_eq!(
            bar.axis_config.y_label.as_deref(),
            Some("Percentage of Tastiness")
        );
        Ok(())
    }

    #[test]
    fn stream_chunk_blocks_carries_new_block_types() -> Result<(), Box<dyn std::error::Error>> {
        let payload = serde_json::json!({
            "type": "blocks",
            "blocks": [
                { "type": "plan", "title": "Plan", "tasks": [] },
                { "type": "some_future_block", "x": 1 }
            ]
        });
        let chunk: crate::api::SlackStreamChunk = serde_json::from_value(payload.clone())?;
        let crate::api::SlackStreamChunk::Blocks { blocks } = &chunk else {
            panic!("Expected a Blocks chunk, got {chunk:?}");
        };
        assert!(matches!(blocks[0], SlackBlock::Plan(_)));
        assert!(matches!(blocks[1], SlackBlock::Unknown(_)));
        assert_eq!(serde_json::to_value(&chunk)?, payload);
        Ok(())
    }

    #[test]
    fn header_block_keeps_its_level() -> Result<(), Box<dyn std::error::Error>> {
        let payload = serde_json::json!({
            "type": "header",
            "text": { "type": "plain_text", "text": "Budget", "emoji": true },
            "level": 2
        });
        let block: SlackBlock = serde_json::from_value(payload.clone())?;
        let SlackBlock::Header(header) = &block else {
            panic!("Expected a header block, got {block:?}");
        };
        assert_eq!(header.level, Some(SlackHeaderLevel(2)));
        assert_eq!(serde_json::to_value(&block)?, payload);
        let plain: SlackBlock = SlackHeaderBlock::new(crate::pt!("Budget")).into();
        assert!(serde_json::to_value(&plain)?.get("level").is_none());
        Ok(())
    }
}
