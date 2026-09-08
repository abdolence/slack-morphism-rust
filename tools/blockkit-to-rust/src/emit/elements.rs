//! The four block element enums and the section/action/input/context element
//! structs.

use slack_morphism::prelude::*;

use crate::emit::{rich_text, workflow};
use crate::leaf;
use crate::writer::{Call, Expr, ListKind};
use crate::Ctx;

/// `SlackSectionBlockElement` is only ever read back through
/// `SlackSectionBlock.accessory`, a single-value position: the caller in
/// `blocks.rs` appends the one `.into()` that promotes the arm's own return
/// type into the enum, so no arm here adds its own.
pub fn emit_slack_section_block_element(v: &SlackSectionBlockElement, ctx: &mut Ctx) -> Expr {
    match v {
        SlackSectionBlockElement::Image(e) => emit_slack_block_image_element(e, ctx),
        SlackSectionBlockElement::Button(e) => emit_slack_block_button_element(e, ctx),
        SlackSectionBlockElement::StaticSelect(e) => emit_slack_block_static_select_element(e, ctx),
        SlackSectionBlockElement::MultiStaticSelect(e) => {
            emit_slack_block_multi_static_select_element(e, ctx)
        }
        SlackSectionBlockElement::ExternalSelect(e) => {
            emit_slack_block_external_select_element(e, ctx)
        }
        SlackSectionBlockElement::MultiExternalSelect(e) => {
            emit_slack_block_multi_external_select_element(e, ctx)
        }
        SlackSectionBlockElement::UsersSelect(e) => emit_slack_block_users_select_element(e, ctx),
        SlackSectionBlockElement::MultiUsersSelect(e) => {
            emit_slack_block_multi_users_select_element(e, ctx)
        }
        SlackSectionBlockElement::ConversationsSelect(e) => {
            emit_slack_block_conversations_select_element(e, ctx)
        }
        SlackSectionBlockElement::MultiConversationsSelect(e) => {
            emit_slack_block_multi_conversations_select_element(e, ctx)
        }
        SlackSectionBlockElement::ChannelsSelect(e) => {
            emit_slack_block_channels_select_element(e, ctx)
        }
        SlackSectionBlockElement::MultiChannelsSelect(e) => {
            emit_slack_block_multi_channels_select_element(e, ctx)
        }
        SlackSectionBlockElement::Overflow(e) => emit_slack_block_overflow_element(e, ctx),
        SlackSectionBlockElement::DatePicker(e) => emit_slack_block_date_picker_element(e, ctx),
        SlackSectionBlockElement::TimePicker(e) => emit_slack_block_time_picker_element(e, ctx),
        SlackSectionBlockElement::PlainTextInput(e) => {
            emit_slack_block_plain_text_input_element(e, ctx)
        }
        SlackSectionBlockElement::NumberInput(e) => emit_slack_block_number_input_element(e, ctx),
        SlackSectionBlockElement::UrlInput(e) => emit_slack_block_url_input_element(e, ctx),
        SlackSectionBlockElement::RadioButtons(e) => emit_slack_block_radio_buttons_element(e, ctx),
        SlackSectionBlockElement::Checkboxes(e) => emit_slack_block_checkboxes_element(e, ctx),
        // `SlackBlockWorkflowButtonElement` has no `From` impl into this enum
        // (unlike every other element struct), so the variant is built by
        // name rather than left to the caller's `.into()`.
        SlackSectionBlockElement::WorkflowButton(e) => {
            Call::new("SlackSectionBlockElement::WorkflowButton")
                .arg(workflow::emit_slack_block_workflow_button_element(e, ctx))
                .into()
        }
    }
}

pub fn emit_slack_action_block_element(v: &SlackActionBlockElement, ctx: &mut Ctx) -> Expr {
    match v {
        SlackActionBlockElement::Button(e) => emit_slack_block_button_element(e, ctx),
        SlackActionBlockElement::Overflow(e) => emit_slack_block_overflow_element(e, ctx),
        SlackActionBlockElement::DatePicker(e) => emit_slack_block_date_picker_element(e, ctx),
        SlackActionBlockElement::TimePicker(e) => emit_slack_block_time_picker_element(e, ctx),
        SlackActionBlockElement::DateTimePicker(e) => {
            emit_slack_block_date_time_picker_element(e, ctx)
        }
        SlackActionBlockElement::PlainTextInput(e) => {
            emit_slack_block_plain_text_input_element(e, ctx)
        }
        SlackActionBlockElement::NumberInput(e) => emit_slack_block_number_input_element(e, ctx),
        SlackActionBlockElement::UrlInput(e) => emit_slack_block_url_input_element(e, ctx),
        SlackActionBlockElement::RadioButtons(e) => emit_slack_block_radio_buttons_element(e, ctx),
        SlackActionBlockElement::Checkboxes(e) => emit_slack_block_checkboxes_element(e, ctx),
        SlackActionBlockElement::StaticSelect(e) => emit_slack_block_static_select_element(e, ctx),
        SlackActionBlockElement::ExternalSelect(e) => {
            emit_slack_block_external_select_element(e, ctx)
        }
        SlackActionBlockElement::UsersSelect(e) => emit_slack_block_users_select_element(e, ctx),
        SlackActionBlockElement::ConversationsSelect(e) => {
            emit_slack_block_conversations_select_element(e, ctx)
        }
        SlackActionBlockElement::ChannelsSelect(e) => {
            emit_slack_block_channels_select_element(e, ctx)
        }
        // Same missing-`From`-impl caveat as the section variant above.
        SlackActionBlockElement::WorkflowButton(e) => {
            Call::new("SlackActionBlockElement::WorkflowButton")
                .arg(workflow::emit_slack_block_workflow_button_element(e, ctx))
                .into()
        }
    }
}

pub fn emit_slack_context_block_element(v: &SlackContextBlockElement, ctx: &mut Ctx) -> Expr {
    match v {
        SlackContextBlockElement::Image(e) => {
            Expr::suffixed(emit_slack_block_image_element(e, ctx), ".into()")
        }
        // `md!`/`pt!` already end in an untyped `.into()`; a second one would
        // not compile.
        SlackContextBlockElement::Plain(t) => leaf::plain_text(t, ctx),
        SlackContextBlockElement::MarkDown(t) => leaf::markdown_text(t, ctx),
    }
}

pub fn emit_slack_input_block_element(v: &SlackInputBlockElement, ctx: &mut Ctx) -> Expr {
    match v {
        SlackInputBlockElement::StaticSelect(e) => emit_slack_block_static_select_element(e, ctx),
        SlackInputBlockElement::MultiStaticSelect(e) => {
            emit_slack_block_multi_static_select_element(e, ctx)
        }
        SlackInputBlockElement::ExternalSelect(e) => {
            emit_slack_block_external_select_element(e, ctx)
        }
        SlackInputBlockElement::MultiExternalSelect(e) => {
            emit_slack_block_multi_external_select_element(e, ctx)
        }
        SlackInputBlockElement::UsersSelect(e) => emit_slack_block_users_select_element(e, ctx),
        SlackInputBlockElement::MultiUsersSelect(e) => {
            emit_slack_block_multi_users_select_element(e, ctx)
        }
        SlackInputBlockElement::ConversationsSelect(e) => {
            emit_slack_block_conversations_select_element(e, ctx)
        }
        SlackInputBlockElement::MultiConversationsSelect(e) => {
            emit_slack_block_multi_conversations_select_element(e, ctx)
        }
        SlackInputBlockElement::ChannelsSelect(e) => {
            emit_slack_block_channels_select_element(e, ctx)
        }
        SlackInputBlockElement::MultiChannelsSelect(e) => {
            emit_slack_block_multi_channels_select_element(e, ctx)
        }
        SlackInputBlockElement::DatePicker(e) => emit_slack_block_date_picker_element(e, ctx),
        SlackInputBlockElement::TimePicker(e) => emit_slack_block_time_picker_element(e, ctx),
        SlackInputBlockElement::DateTimePicker(e) => {
            emit_slack_block_date_time_picker_element(e, ctx)
        }
        SlackInputBlockElement::PlainTextInput(e) => {
            emit_slack_block_plain_text_input_element(e, ctx)
        }
        SlackInputBlockElement::NumberInput(e) => emit_slack_block_number_input_element(e, ctx),
        SlackInputBlockElement::UrlInput(e) => emit_slack_block_url_input_element(e, ctx),
        SlackInputBlockElement::RadioButtons(e) => emit_slack_block_radio_buttons_element(e, ctx),
        SlackInputBlockElement::Checkboxes(e) => emit_slack_block_checkboxes_element(e, ctx),
        SlackInputBlockElement::EmailInput(e) => emit_slack_block_email_input_element(e, ctx),
        SlackInputBlockElement::RichTextInput(e) => {
            emit_slack_block_rich_text_input_element(e, ctx)
        }
        SlackInputBlockElement::FileInput(e) => emit_slack_block_file_input_element(e, ctx),
    }
}

pub fn emit_slack_block_image_element(v: &SlackBlockImageElement, ctx: &mut Ctx) -> Expr {
    let SlackBlockImageElement {
        image_url_or_file,
        alt_text,
    } = v;
    Call::new("SlackBlockImageElement::new")
        .arg(crate::emit::blocks::emit_slack_image_url_or_file(
            image_url_or_file,
            ctx,
        ))
        .arg(leaf::value_str(alt_text))
        .into()
}

pub fn emit_slack_block_button_style(v: &SlackBlockButtonStyle) -> Expr {
    match v {
        SlackBlockButtonStyle::Primary => leaf::unit_variant("SlackBlockButtonStyle", "Primary"),
        SlackBlockButtonStyle::Danger => leaf::unit_variant("SlackBlockButtonStyle", "Danger"),
    }
}

pub fn emit_slack_block_button_element(v: &SlackBlockButtonElement, ctx: &mut Ctx) -> Expr {
    let SlackBlockButtonElement {
        action_id,
        text,
        url,
        value,
        style,
        confirm,
        accessibility_label,
    } = v;
    let mut call = Call::new("SlackBlockButtonElement::new")
        .arg(leaf::value_str(action_id.value()))
        .arg(leaf::plain_text_only(text, ctx));
    if let Some(x) = url {
        call = call.set("with_url", leaf::url_expr(x, ctx));
    }
    if let Some(x) = value {
        call = call.set("with_value", leaf::value_str(x));
    }
    if let Some(x) = style {
        call = call.set("with_style", emit_slack_block_button_style(x));
    }
    if let Some(x) = confirm {
        call = call.set("with_confirm", emit_slack_block_confirm_item(x, ctx));
    }
    if let Some(x) = accessibility_label {
        call = call.set("with_accessibility_label", leaf::value_str(x.value()));
    }
    call.into()
}

pub fn emit_slack_block_confirm_item(v: &SlackBlockConfirmItem, ctx: &mut Ctx) -> Expr {
    let SlackBlockConfirmItem {
        title,
        text,
        confirm,
        deny,
        style,
    } = v;
    let mut call = Call::new("SlackBlockConfirmItem::new")
        .arg(leaf::plain_text_only(title, ctx))
        .arg(leaf::block_text(text, ctx))
        .arg(leaf::plain_text_only(confirm, ctx))
        .arg(leaf::plain_text_only(deny, ctx));
    if let Some(x) = style {
        call = call.set("with_style", leaf::value_str(x));
    }
    call.into()
}

/// `text` is rendered by the caller's choice of renderer because `T` is
/// instantiated at two different text types across the model.
pub fn emit_slack_block_choice_item<T>(
    v: &SlackBlockChoiceItem<T>,
    ctx: &mut Ctx,
    text_of: fn(&T, &mut Ctx) -> Expr,
) -> Expr
where
    T: Into<SlackBlockText> + Clone,
{
    let SlackBlockChoiceItem {
        text,
        value,
        description,
        url,
    } = v;
    let mut call = Call::new("SlackBlockChoiceItem::new")
        .arg(text_of(text, ctx))
        .arg(leaf::value_str(value));
    if let Some(x) = description {
        call = call.set("with_description", leaf::plain_text_only(x, ctx));
    }
    if let Some(x) = url {
        call = call.set("with_url", leaf::url_expr(x, ctx));
    }
    call.into()
}

pub fn emit_slack_block_option_group<T>(
    v: &SlackBlockOptionGroup<T>,
    ctx: &mut Ctx,
    text_of: fn(&T, &mut Ctx) -> Expr,
) -> Expr
where
    T: Into<SlackBlockText> + Clone,
{
    let SlackBlockOptionGroup { label, options } = v;
    Call::new("SlackBlockOptionGroup::new")
        .arg(leaf::plain_text_only(label, ctx))
        .arg(Expr::List {
            kind: ListKind::Vec,
            items: options
                .iter()
                .map(|o| emit_slack_block_choice_item(o, ctx, text_of))
                .collect(),
        })
        .into()
}

pub fn emit_slack_block_static_select_element(
    v: &SlackBlockStaticSelectElement,
    ctx: &mut Ctx,
) -> Expr {
    let SlackBlockStaticSelectElement {
        action_id,
        placeholder,
        options,
        option_groups,
        initial_option,
        confirm,
        focus_on_load,
    } = v;
    let mut call =
        Call::new("SlackBlockStaticSelectElement::new").arg(leaf::value_str(action_id.value()));
    if let Some(x) = placeholder {
        call = call.set("with_placeholder", leaf::plain_text_only(x, ctx));
    }
    if let Some(x) = options {
        call = call.set(
            "with_options",
            Expr::List {
                kind: ListKind::Vec,
                items: x
                    .iter()
                    .map(|o| emit_slack_block_choice_item(o, ctx, leaf::plain_text_only))
                    .collect(),
            },
        );
    }
    if let Some(x) = option_groups {
        call = call.set(
            "with_option_groups",
            Expr::List {
                kind: ListKind::Vec,
                items: x
                    .iter()
                    .map(|g| emit_slack_block_option_group(g, ctx, leaf::plain_text_only))
                    .collect(),
            },
        );
    }
    if let Some(x) = initial_option {
        call = call.set(
            "with_initial_option",
            emit_slack_block_choice_item(x, ctx, leaf::plain_text_only),
        );
    }
    if let Some(x) = confirm {
        call = call.set("with_confirm", emit_slack_block_confirm_item(x, ctx));
    }
    if let Some(x) = focus_on_load {
        call = call.set("with_focus_on_load", leaf::bool_lit(*x));
    }
    call.into()
}

pub fn emit_slack_conversation_filter_include(v: &SlackConversationFilterInclude) -> Expr {
    match v {
        SlackConversationFilterInclude::Im => {
            leaf::unit_variant("SlackConversationFilterInclude", "Im")
        }
        SlackConversationFilterInclude::Mpim => {
            leaf::unit_variant("SlackConversationFilterInclude", "Mpim")
        }
        SlackConversationFilterInclude::Private => {
            leaf::unit_variant("SlackConversationFilterInclude", "Private")
        }
        SlackConversationFilterInclude::Public => {
            leaf::unit_variant("SlackConversationFilterInclude", "Public")
        }
    }
}

pub fn emit_slack_dispatch_action_trigger(v: &SlackDispatchActionTrigger) -> Expr {
    match v {
        SlackDispatchActionTrigger::OnEnterPressed => {
            leaf::unit_variant("SlackDispatchActionTrigger", "OnEnterPressed")
        }
        SlackDispatchActionTrigger::OnCharacterEntered => {
            leaf::unit_variant("SlackDispatchActionTrigger", "OnCharacterEntered")
        }
    }
}

pub fn emit_slack_block_multi_static_select_element(
    v: &SlackBlockMultiStaticSelectElement,
    ctx: &mut Ctx,
) -> Expr {
    let SlackBlockMultiStaticSelectElement {
        action_id,
        placeholder,
        options,
        option_groups,
        initial_options,
        confirm,
        max_selected_items,
        focus_on_load,
    } = v;
    let mut call = Call::new("SlackBlockMultiStaticSelectElement::new")
        .arg(leaf::value_str(action_id.value()));
    if let Some(x) = placeholder {
        call = call.set("with_placeholder", leaf::plain_text_only(x, ctx));
    }
    if let Some(x) = options {
        call = call.set(
            "with_options",
            Expr::List {
                kind: ListKind::Vec,
                items: x
                    .iter()
                    .map(|o| emit_slack_block_choice_item(o, ctx, leaf::plain_text_only))
                    .collect(),
            },
        );
    }
    if let Some(x) = option_groups {
        call = call.set(
            "with_option_groups",
            Expr::List {
                kind: ListKind::Vec,
                items: x
                    .iter()
                    .map(|g| emit_slack_block_option_group(g, ctx, leaf::plain_text_only))
                    .collect(),
            },
        );
    }
    if let Some(x) = initial_options {
        call = call.set(
            "with_initial_options",
            Expr::List {
                kind: ListKind::Vec,
                items: x
                    .iter()
                    .map(|o| emit_slack_block_choice_item(o, ctx, leaf::plain_text_only))
                    .collect(),
            },
        );
    }
    if let Some(x) = confirm {
        call = call.set("with_confirm", emit_slack_block_confirm_item(x, ctx));
    }
    if let Some(x) = max_selected_items {
        call = call.set("with_max_selected_items", leaf::u64_lit(*x));
    }
    if let Some(x) = focus_on_load {
        call = call.set("with_focus_on_load", leaf::bool_lit(*x));
    }
    call.into()
}

pub fn emit_slack_block_external_select_element(
    v: &SlackBlockExternalSelectElement,
    ctx: &mut Ctx,
) -> Expr {
    let SlackBlockExternalSelectElement {
        action_id,
        placeholder,
        initial_option,
        confirm,
        focus_on_load,
        min_query_length,
    } = v;
    let mut call =
        Call::new("SlackBlockExternalSelectElement::new").arg(leaf::value_str(action_id.value()));
    if let Some(x) = placeholder {
        call = call.set("with_placeholder", leaf::plain_text_only(x, ctx));
    }
    if let Some(x) = initial_option {
        call = call.set(
            "with_initial_option",
            emit_slack_block_choice_item(x, ctx, leaf::plain_text_only),
        );
    }
    if let Some(x) = confirm {
        call = call.set("with_confirm", emit_slack_block_confirm_item(x, ctx));
    }
    if let Some(x) = focus_on_load {
        call = call.set("with_focus_on_load", leaf::bool_lit(*x));
    }
    if let Some(x) = min_query_length {
        call = call.set("with_min_query_length", leaf::u64_lit(*x));
    }
    call.into()
}

pub fn emit_slack_block_multi_external_select_element(
    v: &SlackBlockMultiExternalSelectElement,
    ctx: &mut Ctx,
) -> Expr {
    let SlackBlockMultiExternalSelectElement {
        action_id,
        placeholder,
        initial_options,
        confirm,
        max_selected_items,
        focus_on_load,
        min_query_length,
    } = v;
    let mut call = Call::new("SlackBlockMultiExternalSelectElement::new")
        .arg(leaf::value_str(action_id.value()));
    if let Some(x) = placeholder {
        call = call.set("with_placeholder", leaf::plain_text_only(x, ctx));
    }
    if let Some(x) = initial_options {
        call = call.set(
            "with_initial_options",
            Expr::List {
                kind: ListKind::Vec,
                items: x
                    .iter()
                    .map(|o| emit_slack_block_choice_item(o, ctx, leaf::plain_text_only))
                    .collect(),
            },
        );
    }
    if let Some(x) = confirm {
        call = call.set("with_confirm", emit_slack_block_confirm_item(x, ctx));
    }
    if let Some(x) = max_selected_items {
        call = call.set("with_max_selected_items", leaf::u64_lit(*x));
    }
    if let Some(x) = focus_on_load {
        call = call.set("with_focus_on_load", leaf::bool_lit(*x));
    }
    if let Some(x) = min_query_length {
        call = call.set("with_min_query_length", leaf::u64_lit(*x));
    }
    call.into()
}

pub fn emit_slack_block_users_select_element(
    v: &SlackBlockUsersSelectElement,
    ctx: &mut Ctx,
) -> Expr {
    let SlackBlockUsersSelectElement {
        action_id,
        placeholder,
        initial_user,
        confirm,
        focus_on_load,
    } = v;
    let mut call =
        Call::new("SlackBlockUsersSelectElement::new").arg(leaf::value_str(action_id.value()));
    if let Some(x) = placeholder {
        call = call.set("with_placeholder", leaf::plain_text_only(x, ctx));
    }
    if let Some(x) = initial_user {
        call = call.set("with_initial_user", leaf::value_str(x));
    }
    if let Some(x) = confirm {
        call = call.set("with_confirm", emit_slack_block_confirm_item(x, ctx));
    }
    if let Some(x) = focus_on_load {
        call = call.set("with_focus_on_load", leaf::bool_lit(*x));
    }
    call.into()
}

pub fn emit_slack_block_multi_users_select_element(
    v: &SlackBlockMultiUsersSelectElement,
    ctx: &mut Ctx,
) -> Expr {
    let SlackBlockMultiUsersSelectElement {
        action_id,
        placeholder,
        initial_users,
        confirm,
        max_selected_items,
        focus_on_load,
    } = v;
    let mut call =
        Call::new("SlackBlockMultiUsersSelectElement::new").arg(leaf::value_str(action_id.value()));
    if let Some(x) = placeholder {
        call = call.set("with_placeholder", leaf::plain_text_only(x, ctx));
    }
    if let Some(x) = initial_users {
        call = call.set(
            "with_initial_users",
            Expr::List {
                kind: ListKind::Vec,
                items: x.iter().map(|u| leaf::value_str(u)).collect(),
            },
        );
    }
    if let Some(x) = confirm {
        call = call.set("with_confirm", emit_slack_block_confirm_item(x, ctx));
    }
    if let Some(x) = max_selected_items {
        call = call.set("with_max_selected_items", leaf::u64_lit(*x));
    }
    if let Some(x) = focus_on_load {
        call = call.set("with_focus_on_load", leaf::bool_lit(*x));
    }
    call.into()
}

pub fn emit_slack_block_conversation_filter(
    v: &SlackBlockConversationFilter,
    _ctx: &mut Ctx,
) -> Expr {
    let SlackBlockConversationFilter {
        include,
        exclude_external_shared_channels,
        exclude_bot_users,
    } = v;
    let mut call = Call::new("SlackBlockConversationFilter::new");
    if let Some(x) = include {
        call = call.set(
            "with_include",
            Expr::List {
                kind: ListKind::Vec,
                items: x
                    .iter()
                    .map(emit_slack_conversation_filter_include)
                    .collect(),
            },
        );
    }
    if let Some(x) = exclude_external_shared_channels {
        call = call.set("with_exclude_external_shared_channels", leaf::bool_lit(*x));
    }
    if let Some(x) = exclude_bot_users {
        call = call.set("with_exclude_bot_users", leaf::bool_lit(*x));
    }
    call.into()
}

pub fn emit_slack_block_conversations_select_element(
    v: &SlackBlockConversationsSelectElement,
    ctx: &mut Ctx,
) -> Expr {
    let SlackBlockConversationsSelectElement {
        action_id,
        placeholder,
        initial_conversation,
        default_to_current_conversation,
        confirm,
        response_url_enabled,
        focus_on_load,
        filter,
    } = v;
    let mut call = Call::new("SlackBlockConversationsSelectElement::new")
        .arg(leaf::value_str(action_id.value()));
    if let Some(x) = placeholder {
        call = call.set("with_placeholder", leaf::plain_text_only(x, ctx));
    }
    if let Some(x) = initial_conversation {
        call = call.set("with_initial_conversation", leaf::value_str(x.value()));
    }
    if let Some(x) = default_to_current_conversation {
        call = call.set("with_default_to_current_conversation", leaf::bool_lit(*x));
    }
    if let Some(x) = confirm {
        call = call.set("with_confirm", emit_slack_block_confirm_item(x, ctx));
    }
    if let Some(x) = response_url_enabled {
        call = call.set("with_response_url_enabled", leaf::bool_lit(*x));
    }
    if let Some(x) = focus_on_load {
        call = call.set("with_focus_on_load", leaf::bool_lit(*x));
    }
    if let Some(x) = filter {
        call = call.set("with_filter", emit_slack_block_conversation_filter(x, ctx));
    }
    call.into()
}

pub fn emit_slack_block_multi_conversations_select_element(
    v: &SlackBlockMultiConversationsSelectElement,
    ctx: &mut Ctx,
) -> Expr {
    let SlackBlockMultiConversationsSelectElement {
        action_id,
        placeholder,
        initial_conversations,
        default_to_current_conversation,
        confirm,
        max_selected_items,
        focus_on_load,
        filter,
    } = v;
    let mut call = Call::new("SlackBlockMultiConversationsSelectElement::new")
        .arg(leaf::value_str(action_id.value()));
    if let Some(x) = placeholder {
        call = call.set("with_placeholder", leaf::plain_text_only(x, ctx));
    }
    if let Some(x) = initial_conversations {
        call = call.set(
            "with_initial_conversations",
            Expr::List {
                kind: ListKind::Vec,
                items: x.iter().map(|c| leaf::value_str(c.value())).collect(),
            },
        );
    }
    if let Some(x) = default_to_current_conversation {
        call = call.set("with_default_to_current_conversation", leaf::bool_lit(*x));
    }
    if let Some(x) = confirm {
        call = call.set("with_confirm", emit_slack_block_confirm_item(x, ctx));
    }
    if let Some(x) = max_selected_items {
        call = call.set("with_max_selected_items", leaf::u64_lit(*x));
    }
    if let Some(x) = focus_on_load {
        call = call.set("with_focus_on_load", leaf::bool_lit(*x));
    }
    if let Some(x) = filter {
        call = call.set("with_filter", emit_slack_block_conversation_filter(x, ctx));
    }
    call.into()
}

pub fn emit_slack_block_channels_select_element(
    v: &SlackBlockChannelsSelectElement,
    ctx: &mut Ctx,
) -> Expr {
    let SlackBlockChannelsSelectElement {
        action_id,
        placeholder,
        initial_channel,
        confirm,
        response_url_enabled,
        focus_on_load,
    } = v;
    let mut call =
        Call::new("SlackBlockChannelsSelectElement::new").arg(leaf::value_str(action_id.value()));
    if let Some(x) = placeholder {
        call = call.set("with_placeholder", leaf::plain_text_only(x, ctx));
    }
    if let Some(x) = initial_channel {
        call = call.set("with_initial_channel", leaf::value_str(x.value()));
    }
    if let Some(x) = confirm {
        call = call.set("with_confirm", emit_slack_block_confirm_item(x, ctx));
    }
    if let Some(x) = response_url_enabled {
        call = call.set("with_response_url_enabled", leaf::bool_lit(*x));
    }
    if let Some(x) = focus_on_load {
        call = call.set("with_focus_on_load", leaf::bool_lit(*x));
    }
    call.into()
}

pub fn emit_slack_block_multi_channels_select_element(
    v: &SlackBlockMultiChannelsSelectElement,
    ctx: &mut Ctx,
) -> Expr {
    let SlackBlockMultiChannelsSelectElement {
        action_id,
        placeholder,
        initial_channels,
        confirm,
        max_selected_items,
        focus_on_load,
    } = v;
    let mut call = Call::new("SlackBlockMultiChannelsSelectElement::new")
        .arg(leaf::value_str(action_id.value()));
    if let Some(x) = placeholder {
        call = call.set("with_placeholder", leaf::plain_text_only(x, ctx));
    }
    if let Some(x) = initial_channels {
        call = call.set(
            "with_initial_channels",
            Expr::List {
                kind: ListKind::Vec,
                items: x.iter().map(|c| leaf::value_str(c.value())).collect(),
            },
        );
    }
    if let Some(x) = confirm {
        call = call.set("with_confirm", emit_slack_block_confirm_item(x, ctx));
    }
    if let Some(x) = max_selected_items {
        call = call.set("with_max_selected_items", leaf::u64_lit(*x));
    }
    if let Some(x) = focus_on_load {
        call = call.set("with_focus_on_load", leaf::bool_lit(*x));
    }
    call.into()
}

pub fn emit_slack_block_overflow_element(v: &SlackBlockOverflowElement, ctx: &mut Ctx) -> Expr {
    let SlackBlockOverflowElement {
        action_id,
        options,
        confirm,
    } = v;
    let mut call = Call::new("SlackBlockOverflowElement::new")
        .arg(leaf::value_str(action_id.value()))
        .arg(Expr::List {
            kind: ListKind::Vec,
            items: options
                .iter()
                .map(|o| emit_slack_block_choice_item(o, ctx, leaf::plain_text_only))
                .collect(),
        });
    if let Some(x) = confirm {
        call = call.set("with_confirm", emit_slack_block_confirm_item(x, ctx));
    }
    call.into()
}

pub fn emit_slack_block_date_picker_element(
    v: &SlackBlockDatePickerElement,
    ctx: &mut Ctx,
) -> Expr {
    let SlackBlockDatePickerElement {
        action_id,
        placeholder,
        initial_date,
        confirm,
        focus_on_load,
    } = v;
    let mut call =
        Call::new("SlackBlockDatePickerElement::new").arg(leaf::value_str(action_id.value()));
    if let Some(x) = placeholder {
        call = call.set("with_placeholder", leaf::plain_text_only(x, ctx));
    }
    if let Some(x) = initial_date {
        call = call.set("with_initial_date", leaf::value_str(x));
    }
    if let Some(x) = confirm {
        call = call.set("with_confirm", emit_slack_block_confirm_item(x, ctx));
    }
    if let Some(x) = focus_on_load {
        call = call.set("with_focus_on_load", leaf::bool_lit(*x));
    }
    call.into()
}

pub fn emit_slack_block_time_picker_element(
    v: &SlackBlockTimePickerElement,
    ctx: &mut Ctx,
) -> Expr {
    let SlackBlockTimePickerElement {
        action_id,
        initial_time,
        confirm,
        focus_on_load,
        placeholder,
        timezone,
    } = v;
    let mut call =
        Call::new("SlackBlockTimePickerElement::new").arg(leaf::value_str(action_id.value()));
    if let Some(x) = initial_time {
        call = call.set("with_initial_time", leaf::value_str(x));
    }
    if let Some(x) = confirm {
        call = call.set("with_confirm", emit_slack_block_confirm_item(x, ctx));
    }
    if let Some(x) = focus_on_load {
        call = call.set("with_focus_on_load", leaf::bool_lit(*x));
    }
    if let Some(x) = placeholder {
        call = call.set("with_placeholder", leaf::plain_text_only(x, ctx));
    }
    if let Some(x) = timezone {
        call = call.set("with_timezone", leaf::value_str(x));
    }
    call.into()
}

pub fn emit_slack_block_date_time_picker_element(
    v: &SlackBlockDateTimePickerElement,
    ctx: &mut Ctx,
) -> Expr {
    let SlackBlockDateTimePickerElement {
        action_id,
        initial_date_time,
        confirm,
        focus_on_load,
    } = v;
    let mut call =
        Call::new("SlackBlockDateTimePickerElement::new").arg(leaf::value_str(action_id.value()));
    if let Some(x) = initial_date_time {
        call = call.set("with_initial_date_time", leaf::datetime_expr(x, ctx));
    }
    if let Some(x) = confirm {
        call = call.set("with_confirm", emit_slack_block_confirm_item(x, ctx));
    }
    if let Some(x) = focus_on_load {
        call = call.set("with_focus_on_load", leaf::bool_lit(*x));
    }
    call.into()
}

pub fn emit_slack_dispatch_action_config(v: &SlackDispatchActionConfig, _ctx: &mut Ctx) -> Expr {
    let SlackDispatchActionConfig { trigger_actions_on } = v;
    let mut call = Call::new("SlackDispatchActionConfig::new");
    if let Some(x) = trigger_actions_on {
        call = call.set(
            "with_trigger_actions_on",
            Expr::List {
                kind: ListKind::Vec,
                items: x.iter().map(emit_slack_dispatch_action_trigger).collect(),
            },
        );
    }
    call.into()
}

pub fn emit_slack_block_plain_text_input_element(
    v: &SlackBlockPlainTextInputElement,
    ctx: &mut Ctx,
) -> Expr {
    let SlackBlockPlainTextInputElement {
        action_id,
        placeholder,
        initial_value,
        multiline,
        min_length,
        max_length,
        focus_on_load,
        dispatch_action_config,
    } = v;
    let mut call =
        Call::new("SlackBlockPlainTextInputElement::new").arg(leaf::value_str(action_id.value()));
    if let Some(x) = placeholder {
        call = call.set("with_placeholder", leaf::plain_text_only(x, ctx));
    }
    if let Some(x) = initial_value {
        call = call.set("with_initial_value", leaf::value_str(x));
    }
    if let Some(x) = multiline {
        call = call.set("with_multiline", leaf::bool_lit(*x));
    }
    if let Some(x) = min_length {
        call = call.set("with_min_length", leaf::u64_lit(*x));
    }
    if let Some(x) = max_length {
        call = call.set("with_max_length", leaf::u64_lit(*x));
    }
    if let Some(x) = focus_on_load {
        call = call.set("with_focus_on_load", leaf::bool_lit(*x));
    }
    if let Some(x) = dispatch_action_config {
        call = call.set(
            "with_dispatch_action_config",
            emit_slack_dispatch_action_config(x, ctx),
        );
    }
    call.into()
}

pub fn emit_slack_block_number_input_element(
    v: &SlackBlockNumberInputElement,
    ctx: &mut Ctx,
) -> Expr {
    let SlackBlockNumberInputElement {
        action_id,
        is_decimal_allowed,
        focus_on_load,
        placeholder,
        initial_value,
        min_value,
        max_value,
    } = v;
    let mut call = Call::new("SlackBlockNumberInputElement::new")
        .arg(leaf::value_str(action_id.value()))
        .arg(leaf::bool_lit(*is_decimal_allowed));
    if let Some(x) = focus_on_load {
        call = call.set("with_focus_on_load", leaf::bool_lit(*x));
    }
    if let Some(x) = placeholder {
        call = call.set("with_placeholder", leaf::plain_text_only(x, ctx));
    }
    if let Some(x) = initial_value {
        call = call.set("with_initial_value", leaf::value_str(x));
    }
    if let Some(x) = min_value {
        call = call.set("with_min_value", leaf::value_str(x));
    }
    if let Some(x) = max_value {
        call = call.set("with_max_value", leaf::value_str(x));
    }
    call.into()
}

pub fn emit_slack_block_url_input_element(v: &SlackBlockUrlInputElement, ctx: &mut Ctx) -> Expr {
    let SlackBlockUrlInputElement {
        action_id,
        placeholder,
        initial_value,
    } = v;
    let mut call =
        Call::new("SlackBlockUrlInputElement::new").arg(leaf::value_str(action_id.value()));
    if let Some(x) = placeholder {
        call = call.set("with_placeholder", leaf::plain_text_only(x, ctx));
    }
    if let Some(x) = initial_value {
        call = call.set("with_initial_value", leaf::value_str(x));
    }
    call.into()
}

pub fn emit_slack_block_email_input_element(
    v: &SlackBlockEmailInputElement,
    ctx: &mut Ctx,
) -> Expr {
    let SlackBlockEmailInputElement {
        action_id,
        focus_on_load,
        placeholder,
        initial_value,
    } = v;
    let mut call =
        Call::new("SlackBlockEmailInputElement::new").arg(leaf::value_str(action_id.value()));
    if let Some(x) = focus_on_load {
        call = call.set("with_focus_on_load", leaf::bool_lit(*x));
    }
    if let Some(x) = placeholder {
        call = call.set("with_placeholder", leaf::plain_text_only(x, ctx));
    }
    if let Some(x) = initial_value {
        call = call.set("with_initial_value", leaf::value_str(x.value()));
    }
    call.into()
}

pub fn emit_slack_block_radio_buttons_element(
    v: &SlackBlockRadioButtonsElement,
    ctx: &mut Ctx,
) -> Expr {
    let SlackBlockRadioButtonsElement {
        action_id,
        options,
        initial_option,
        confirm,
        focus_on_load,
    } = v;
    let mut call = Call::new("SlackBlockRadioButtonsElement::new")
        .arg(leaf::value_str(action_id.value()))
        .arg(Expr::List {
            kind: ListKind::Vec,
            items: options
                .iter()
                .map(|o| emit_slack_block_choice_item(o, ctx, leaf::block_text))
                .collect(),
        });
    if let Some(x) = initial_option {
        call = call.set(
            "with_initial_option",
            emit_slack_block_choice_item(x, ctx, leaf::block_text),
        );
    }
    if let Some(x) = confirm {
        call = call.set("with_confirm", emit_slack_block_confirm_item(x, ctx));
    }
    if let Some(x) = focus_on_load {
        call = call.set("with_focus_on_load", leaf::bool_lit(*x));
    }
    call.into()
}

pub fn emit_slack_block_checkboxes_element(v: &SlackBlockCheckboxesElement, ctx: &mut Ctx) -> Expr {
    let SlackBlockCheckboxesElement {
        action_id,
        options,
        initial_options,
        confirm,
        focus_on_load,
    } = v;
    let mut call = Call::new("SlackBlockCheckboxesElement::new")
        .arg(leaf::value_str(action_id.value()))
        .arg(Expr::List {
            kind: ListKind::Vec,
            items: options
                .iter()
                .map(|o| emit_slack_block_choice_item(o, ctx, leaf::block_text))
                .collect(),
        });
    if let Some(x) = initial_options {
        call = call.set(
            "with_initial_options",
            Expr::List {
                kind: ListKind::Vec,
                items: x
                    .iter()
                    .map(|o| emit_slack_block_choice_item(o, ctx, leaf::block_text))
                    .collect(),
            },
        );
    }
    if let Some(x) = confirm {
        call = call.set("with_confirm", emit_slack_block_confirm_item(x, ctx));
    }
    if let Some(x) = focus_on_load {
        call = call.set("with_focus_on_load", leaf::bool_lit(*x));
    }
    call.into()
}

pub fn emit_slack_block_rich_text_input_element(
    v: &SlackBlockRichTextInputElement,
    ctx: &mut Ctx,
) -> Expr {
    let SlackBlockRichTextInputElement {
        action_id,
        initial_value,
        focus_on_load,
        placeholder,
    } = v;
    let mut call =
        Call::new("SlackBlockRichTextInputElement::new").arg(leaf::value_str(action_id.value()));
    if let Some(x) = initial_value {
        call = call.set(
            "with_initial_value",
            rich_text::emit_slack_rich_text_block(x, ctx),
        );
    }
    if let Some(x) = focus_on_load {
        call = call.set("with_focus_on_load", leaf::bool_lit(*x));
    }
    if let Some(x) = placeholder {
        call = call.set("with_placeholder", leaf::plain_text_only(x, ctx));
    }
    call.into()
}

pub fn emit_slack_block_file_input_element(v: &SlackBlockFileInputElement, _ctx: &mut Ctx) -> Expr {
    let SlackBlockFileInputElement {
        action_id,
        filetypes,
        max_files,
    } = v;
    let mut call =
        Call::new("SlackBlockFileInputElement::new").arg(leaf::value_str(action_id.value()));
    if let Some(x) = filetypes {
        call = call.set(
            "with_filetypes",
            Expr::List {
                kind: ListKind::Vec,
                items: x.iter().map(|f| leaf::value_str(f)).collect(),
            },
        );
    }
    if let Some(x) = max_files {
        call = call.set("with_max_files", leaf::u64_lit(*x));
    }
    call.into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Options;
    use serde_json::json;

    #[test]
    fn spec_example_two_actions_block_renders_verbatim() {
        let options = Options::default();
        let mut ctx = Ctx::new(&options);
        let block: SlackBlock = serde_json::from_value(json!({
            "type": "actions", "block_id": "deploy-actions",
            "elements": [
                { "type": "button", "text": { "type": "plain_text", "text": "Approve", "emoji": true },
                  "style": "primary", "value": "approve", "action_id": "approve" },
                { "type": "static_select",
                  "placeholder": { "type": "plain_text", "text": "Choose a region", "emoji": true },
                  "options": [
                    { "text": { "type": "plain_text", "text": "US East", "emoji": true }, "value": "us-east-1" },
                    { "text": { "type": "plain_text", "text": "EU West", "emoji": true }, "value": "eu-west-1" } ],
                  "action_id": "region" } ]
        }))
        .expect("fixture parses");
        let mut w = crate::writer::Writer::new();
        w.open("x");
        let out = crate::emit::blocks::emit_slack_block(&block, &mut ctx).render(&mut w);
        let expected = "\
SlackActionsBlock::new(slack_blocks![
        SlackBlockButtonElement::new(\"approve\".into(), pt!(\"Approve\"))
            .with_value(\"approve\".into())
            .with_style(SlackBlockButtonStyle::Primary),
        SlackBlockStaticSelectElement::new(\"region\".into())
            .with_placeholder(pt!(\"Choose a region\"))
            .with_options(vec![
                SlackBlockChoiceItem::new(pt!(\"US East\"), \"us-east-1\".into()),
                SlackBlockChoiceItem::new(pt!(\"EU West\"), \"eu-west-1\".into()),
            ]),
    ])
    .with_block_id(\"deploy-actions\".into())";
        assert_eq!(out, expected);
    }

    #[test]
    fn setter_order_follows_declaration_not_input() {
        let options = Options::default();
        let mut ctx = Ctx::new(&options);
        // `style` appears before `value` in the JSON; the struct declares
        // `action_id, text, url, value, style, ...` (kit.rs:351-359).
        let element: SlackActionBlockElement = serde_json::from_value(json!({
            "type": "button", "action_id": "a", "style": "danger", "value": "v",
            "text": { "type": "plain_text", "text": "T" }
        }))
        .expect("parses");
        let out = emit_slack_action_block_element(&element, &mut ctx).flat();
        assert_eq!(
            out,
            "SlackBlockButtonElement::new(\"a\".into(), pt!(\"T\")).with_value(\"v\".into())\
             .with_style(SlackBlockButtonStyle::Danger)"
        );
    }

    #[test]
    fn a_multi_static_select_emits_option_groups_as_a_vec() {
        let options = Options::default();
        let mut ctx = Ctx::new(&options);
        let element: SlackInputBlockElement = serde_json::from_value(json!({
            "type": "multi_static_select", "action_id": "a", "max_selected_items": 3,
            "option_groups": [{
                "label": { "type": "plain_text", "text": "G" },
                "options": [{ "text": { "type": "plain_text", "text": "O" }, "value": "o" }]
            }]
        }))
        .expect("parses");
        assert_eq!(
            emit_slack_input_block_element(&element, &mut ctx).flat(),
            "SlackBlockMultiStaticSelectElement::new(\"a\".into())\
             .with_option_groups(vec![SlackBlockOptionGroup::new(pt!(\"G\"), \
             vec![SlackBlockChoiceItem::new(pt!(\"O\"), \"o\".into())])])\
             .with_max_selected_items(3)"
        );
    }

    #[test]
    fn a_context_image_element_ends_in_into() {
        let options = Options::default();
        let mut ctx = Ctx::new(&options);
        let element: SlackContextBlockElement = serde_json::from_value(json!({
            "type": "image", "image_url": "https://example.com/a.png", "alt_text": "a"
        }))
        .expect("parses");
        assert_eq!(
            emit_slack_context_block_element(&element, &mut ctx).flat(),
            "SlackBlockImageElement::new(Url::parse(\"https://example.com/a.png\")?.into(), \
             \"a\".into()).into()"
        );
    }

    #[test]
    fn the_conversation_filter_fixture_emits_builders() {
        let options = Options::default();
        let mut ctx = Ctx::new(&options);
        let payload = include_str!(
            "../../../../src/models/blocks/fixtures/slack_conversations_select_with_filter.json"
        );
        let block: SlackBlock = serde_json::from_str(payload).expect("fixture parses");
        let out = crate::emit::blocks::emit_slack_block(&block, &mut ctx).flat();
        assert!(
            out.contains(
                "SlackBlockConversationFilter::new().with_include(vec![\
                 SlackConversationFilterInclude::Public, SlackConversationFilterInclude::Private])"
            ),
            "{out}"
        );
        assert!(!out.contains("serde_json::from_value"), "{out}");
    }

    #[test]
    fn a_datetime_picker_renders_its_timestamp_as_a_parse_call() {
        let options = Options::default();
        let mut ctx = Ctx::new(&options);
        let element: SlackActionBlockElement = serde_json::from_value(json!({
            "type": "datetimepicker", "action_id": "a", "initial_date_time": 1577839362
        }))
        .expect("parses");
        assert_eq!(
            emit_slack_action_block_element(&element, &mut ctx).flat(),
            "SlackBlockDateTimePickerElement::new(\"a\".into())\
             .with_initial_date_time(SlackDateTime(\"2020-01-01T00:42:42Z\".parse()?))"
        );
        assert!(ctx.needs_result);
    }

    #[test]
    fn a_number_input_takes_is_decimal_allowed_as_a_constructor_argument() {
        let options = Options::default();
        let mut ctx = Ctx::new(&options);
        let element: SlackInputBlockElement = serde_json::from_value(json!({
            "type": "number_input", "action_id": "a", "is_decimal_allowed": false,
            "max_value": "10"
        }))
        .expect("parses");
        assert_eq!(
            emit_slack_input_block_element(&element, &mut ctx).flat(),
            "SlackBlockNumberInputElement::new(\"a\".into(), false).with_max_value(\"10\".into())"
        );
    }
}
