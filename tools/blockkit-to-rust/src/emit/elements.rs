//! The four block element enums and the section/action/input/context element
//! structs.

use slack_morphism::prelude::*;

use crate::emit::{stub, workflow};
use crate::leaf;
use crate::writer::{Call, Expr, ListKind};
use crate::Ctx;

/// Every arm that is not an image or a text object ends in `.into()`, because
/// `SlackSectionBlockElement` is what a `slack_blocks![]` list or a single
/// `with_accessory` argument is typed as.
pub fn emit_slack_section_block_element(v: &SlackSectionBlockElement, ctx: &mut Ctx) -> Expr {
    match v {
        SlackSectionBlockElement::Image(e) => {
            Expr::suffixed(emit_slack_block_image_element(e, ctx), ".into()")
        }
        SlackSectionBlockElement::Button(e) => {
            Expr::suffixed(emit_slack_block_button_element(e, ctx), ".into()")
        }
        SlackSectionBlockElement::StaticSelect(e) => {
            Expr::suffixed(emit_slack_block_static_select_element(e, ctx), ".into()")
        }
        SlackSectionBlockElement::MultiStaticSelect(e) => {
            Expr::suffixed(stub(e, "multi static select element", ctx), ".into()")
        }
        SlackSectionBlockElement::ExternalSelect(e) => {
            Expr::suffixed(stub(e, "external select element", ctx), ".into()")
        }
        SlackSectionBlockElement::MultiExternalSelect(e) => {
            Expr::suffixed(stub(e, "multi external select element", ctx), ".into()")
        }
        SlackSectionBlockElement::UsersSelect(e) => {
            Expr::suffixed(stub(e, "users select element", ctx), ".into()")
        }
        SlackSectionBlockElement::MultiUsersSelect(e) => {
            Expr::suffixed(stub(e, "multi users select element", ctx), ".into()")
        }
        SlackSectionBlockElement::ConversationsSelect(e) => {
            Expr::suffixed(stub(e, "conversations select element", ctx), ".into()")
        }
        SlackSectionBlockElement::MultiConversationsSelect(e) => Expr::suffixed(
            stub(e, "multi conversations select element", ctx),
            ".into()",
        ),
        SlackSectionBlockElement::ChannelsSelect(e) => {
            Expr::suffixed(stub(e, "channels select element", ctx), ".into()")
        }
        SlackSectionBlockElement::MultiChannelsSelect(e) => {
            Expr::suffixed(stub(e, "multi channels select element", ctx), ".into()")
        }
        SlackSectionBlockElement::Overflow(e) => {
            Expr::suffixed(stub(e, "overflow element", ctx), ".into()")
        }
        SlackSectionBlockElement::DatePicker(e) => {
            Expr::suffixed(stub(e, "date picker element", ctx), ".into()")
        }
        SlackSectionBlockElement::TimePicker(e) => {
            Expr::suffixed(stub(e, "time picker element", ctx), ".into()")
        }
        SlackSectionBlockElement::PlainTextInput(e) => {
            Expr::suffixed(stub(e, "plain text input element", ctx), ".into()")
        }
        SlackSectionBlockElement::NumberInput(e) => {
            Expr::suffixed(stub(e, "number input element", ctx), ".into()")
        }
        SlackSectionBlockElement::UrlInput(e) => {
            Expr::suffixed(stub(e, "url input element", ctx), ".into()")
        }
        SlackSectionBlockElement::RadioButtons(e) => {
            Expr::suffixed(stub(e, "radio buttons element", ctx), ".into()")
        }
        SlackSectionBlockElement::Checkboxes(e) => {
            Expr::suffixed(stub(e, "checkboxes element", ctx), ".into()")
        }
        SlackSectionBlockElement::WorkflowButton(e) => Expr::suffixed(
            workflow::emit_slack_block_workflow_button_element(e, ctx),
            ".into()",
        ),
    }
}

pub fn emit_slack_action_block_element(v: &SlackActionBlockElement, ctx: &mut Ctx) -> Expr {
    match v {
        SlackActionBlockElement::Button(e) => emit_slack_block_button_element(e, ctx),
        SlackActionBlockElement::Overflow(e) => stub(e, "overflow element", ctx),
        SlackActionBlockElement::DatePicker(e) => stub(e, "date picker element", ctx),
        SlackActionBlockElement::TimePicker(e) => stub(e, "time picker element", ctx),
        SlackActionBlockElement::DateTimePicker(e) => stub(e, "date time picker element", ctx),
        SlackActionBlockElement::PlainTextInput(e) => stub(e, "plain text input element", ctx),
        SlackActionBlockElement::NumberInput(e) => stub(e, "number input element", ctx),
        SlackActionBlockElement::UrlInput(e) => stub(e, "url input element", ctx),
        SlackActionBlockElement::RadioButtons(e) => stub(e, "radio buttons element", ctx),
        SlackActionBlockElement::Checkboxes(e) => stub(e, "checkboxes element", ctx),
        SlackActionBlockElement::StaticSelect(e) => emit_slack_block_static_select_element(e, ctx),
        SlackActionBlockElement::ExternalSelect(e) => stub(e, "external select element", ctx),
        SlackActionBlockElement::UsersSelect(e) => stub(e, "users select element", ctx),
        SlackActionBlockElement::ConversationsSelect(e) => {
            stub(e, "conversations select element", ctx)
        }
        SlackActionBlockElement::ChannelsSelect(e) => stub(e, "channels select element", ctx),
        SlackActionBlockElement::WorkflowButton(e) => {
            workflow::emit_slack_block_workflow_button_element(e, ctx)
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
        SlackInputBlockElement::MultiStaticSelect(e) => stub(e, "multi static select element", ctx),
        SlackInputBlockElement::ExternalSelect(e) => stub(e, "external select element", ctx),
        SlackInputBlockElement::MultiExternalSelect(e) => {
            stub(e, "multi external select element", ctx)
        }
        SlackInputBlockElement::UsersSelect(e) => stub(e, "users select element", ctx),
        SlackInputBlockElement::MultiUsersSelect(e) => stub(e, "multi users select element", ctx),
        SlackInputBlockElement::ConversationsSelect(e) => {
            stub(e, "conversations select element", ctx)
        }
        SlackInputBlockElement::MultiConversationsSelect(e) => {
            stub(e, "multi conversations select element", ctx)
        }
        SlackInputBlockElement::ChannelsSelect(e) => stub(e, "channels select element", ctx),
        SlackInputBlockElement::MultiChannelsSelect(e) => {
            stub(e, "multi channels select element", ctx)
        }
        SlackInputBlockElement::DatePicker(e) => stub(e, "date picker element", ctx),
        SlackInputBlockElement::TimePicker(e) => stub(e, "time picker element", ctx),
        SlackInputBlockElement::DateTimePicker(e) => stub(e, "date time picker element", ctx),
        SlackInputBlockElement::PlainTextInput(e) => stub(e, "plain text input element", ctx),
        SlackInputBlockElement::NumberInput(e) => stub(e, "number input element", ctx),
        SlackInputBlockElement::UrlInput(e) => stub(e, "url input element", ctx),
        SlackInputBlockElement::RadioButtons(e) => stub(e, "radio buttons element", ctx),
        SlackInputBlockElement::Checkboxes(e) => stub(e, "checkboxes element", ctx),
        SlackInputBlockElement::EmailInput(e) => stub(e, "email input element", ctx),
        SlackInputBlockElement::RichTextInput(e) => stub(e, "rich text input element", ctx),
        SlackInputBlockElement::FileInput(e) => stub(e, "file input element", ctx),
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
    stub(v, "multi static select element", ctx)
}

pub fn emit_slack_block_external_select_element(
    v: &SlackBlockExternalSelectElement,
    ctx: &mut Ctx,
) -> Expr {
    stub(v, "external select element", ctx)
}

pub fn emit_slack_block_multi_external_select_element(
    v: &SlackBlockMultiExternalSelectElement,
    ctx: &mut Ctx,
) -> Expr {
    stub(v, "multi external select element", ctx)
}

pub fn emit_slack_block_users_select_element(
    v: &SlackBlockUsersSelectElement,
    ctx: &mut Ctx,
) -> Expr {
    stub(v, "users select element", ctx)
}

pub fn emit_slack_block_multi_users_select_element(
    v: &SlackBlockMultiUsersSelectElement,
    ctx: &mut Ctx,
) -> Expr {
    stub(v, "multi users select element", ctx)
}

pub fn emit_slack_block_conversation_filter(
    v: &SlackBlockConversationFilter,
    ctx: &mut Ctx,
) -> Expr {
    stub(v, "conversation filter", ctx)
}

pub fn emit_slack_block_conversations_select_element(
    v: &SlackBlockConversationsSelectElement,
    ctx: &mut Ctx,
) -> Expr {
    stub(v, "conversations select element", ctx)
}

pub fn emit_slack_block_multi_conversations_select_element(
    v: &SlackBlockMultiConversationsSelectElement,
    ctx: &mut Ctx,
) -> Expr {
    stub(v, "multi conversations select element", ctx)
}

pub fn emit_slack_block_channels_select_element(
    v: &SlackBlockChannelsSelectElement,
    ctx: &mut Ctx,
) -> Expr {
    stub(v, "channels select element", ctx)
}

pub fn emit_slack_block_multi_channels_select_element(
    v: &SlackBlockMultiChannelsSelectElement,
    ctx: &mut Ctx,
) -> Expr {
    stub(v, "multi channels select element", ctx)
}

pub fn emit_slack_block_overflow_element(v: &SlackBlockOverflowElement, ctx: &mut Ctx) -> Expr {
    stub(v, "overflow element", ctx)
}

pub fn emit_slack_block_date_picker_element(
    v: &SlackBlockDatePickerElement,
    ctx: &mut Ctx,
) -> Expr {
    stub(v, "date picker element", ctx)
}

pub fn emit_slack_block_time_picker_element(
    v: &SlackBlockTimePickerElement,
    ctx: &mut Ctx,
) -> Expr {
    stub(v, "time picker element", ctx)
}

pub fn emit_slack_block_date_time_picker_element(
    v: &SlackBlockDateTimePickerElement,
    ctx: &mut Ctx,
) -> Expr {
    stub(v, "date time picker element", ctx)
}

pub fn emit_slack_dispatch_action_config(v: &SlackDispatchActionConfig, ctx: &mut Ctx) -> Expr {
    stub(v, "dispatch action config", ctx)
}

pub fn emit_slack_block_plain_text_input_element(
    v: &SlackBlockPlainTextInputElement,
    ctx: &mut Ctx,
) -> Expr {
    stub(v, "plain text input element", ctx)
}

pub fn emit_slack_block_number_input_element(
    v: &SlackBlockNumberInputElement,
    ctx: &mut Ctx,
) -> Expr {
    stub(v, "number input element", ctx)
}

pub fn emit_slack_block_url_input_element(v: &SlackBlockUrlInputElement, ctx: &mut Ctx) -> Expr {
    stub(v, "url input element", ctx)
}

pub fn emit_slack_block_email_input_element(
    v: &SlackBlockEmailInputElement,
    ctx: &mut Ctx,
) -> Expr {
    stub(v, "email input element", ctx)
}

pub fn emit_slack_block_radio_buttons_element(
    v: &SlackBlockRadioButtonsElement,
    ctx: &mut Ctx,
) -> Expr {
    stub(v, "radio buttons element", ctx)
}

pub fn emit_slack_block_checkboxes_element(v: &SlackBlockCheckboxesElement, ctx: &mut Ctx) -> Expr {
    stub(v, "checkboxes element", ctx)
}

pub fn emit_slack_block_rich_text_input_element(
    v: &SlackBlockRichTextInputElement,
    ctx: &mut Ctx,
) -> Expr {
    stub(v, "rich text input element", ctx)
}

pub fn emit_slack_block_file_input_element(v: &SlackBlockFileInputElement, ctx: &mut Ctx) -> Expr {
    stub(v, "file input element", ctx)
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
}
