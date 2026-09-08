//! Table, task card, alert, card, carousel and context-actions blocks.

use slack_morphism::prelude::*;

use crate::emit::{blocks, elements, rich_text};
use crate::leaf;
use crate::writer::{Call, Expr, ListKind};
use crate::Ctx;

pub fn emit_slack_table_block(v: &SlackTableBlock, ctx: &mut Ctx) -> Expr {
    let SlackTableBlock {
        block_id,
        rows,
        column_settings,
    } = v;
    let mut call = Call::new("SlackTableBlock::new").arg(Expr::List {
        kind: ListKind::Vec,
        items: rows
            .iter()
            .map(|row| Expr::List {
                kind: ListKind::SlackBlocks,
                items: row
                    .iter()
                    .map(|cell| emit_slack_table_cell(cell, ctx))
                    .collect(),
            })
            .collect(),
    });
    if let Some(x) = block_id {
        call = call.set("with_block_id", leaf::value_str(x.value()));
    }
    if let Some(x) = column_settings {
        call = call.set(
            "with_column_settings",
            Expr::List {
                kind: ListKind::Vec,
                items: x
                    .iter()
                    .map(|c| emit_slack_table_column_setting(c, ctx))
                    .collect(),
            },
        );
    }
    call.into()
}

pub fn emit_slack_table_raw_text_cell(v: &SlackTableRawTextCell) -> Expr {
    let SlackTableRawTextCell { text } = v;
    Call::new("SlackTableRawTextCell::new")
        .arg(leaf::value_str(text))
        .into()
}

pub fn emit_slack_table_rich_text_cell(v: &SlackTableRichTextCell, ctx: &mut Ctx) -> Expr {
    let SlackTableRichTextCell { elements } = v;
    Call::new("SlackTableRichTextCell::new")
        .arg(Expr::List {
            kind: ListKind::SlackBlocks,
            items: elements
                .iter()
                .map(|e| rich_text::emit_slack_rich_text_element(e, ctx))
                .collect(),
        })
        .into()
}

/// A raw-text cell always collapses: its only field is the text itself, so
/// the bare literal and the builder form carry identical information. A
/// rich-text cell collapses under the same rule as a rich-text list item
/// (`rich_text::collapsible_text`): a single section holding one unstyled
/// run.
pub fn emit_slack_table_cell(v: &SlackTableCell, ctx: &mut Ctx) -> Expr {
    match v {
        SlackTableCell::RawText(SlackTableRawTextCell { text }) => leaf::str_lit(text),
        SlackTableCell::RichText(cell) => {
            let SlackTableRichTextCell { elements } = cell;
            match elements.as_slice() {
                [SlackRichTextElement::Section(section)] => {
                    match rich_text::collapsible_text(section) {
                        Some(text) => leaf::str_lit(text),
                        None => emit_slack_table_rich_text_cell(cell, ctx),
                    }
                }
                _ => emit_slack_table_rich_text_cell(cell, ctx),
            }
        }
    }
}

pub fn emit_slack_table_column_align(v: &SlackTableColumnAlign) -> Expr {
    match v {
        SlackTableColumnAlign::Left => leaf::unit_variant("SlackTableColumnAlign", "Left"),
        SlackTableColumnAlign::Center => leaf::unit_variant("SlackTableColumnAlign", "Center"),
        SlackTableColumnAlign::Right => leaf::unit_variant("SlackTableColumnAlign", "Right"),
    }
}

pub fn emit_slack_table_column_setting(v: &SlackTableColumnSetting, _ctx: &mut Ctx) -> Expr {
    let SlackTableColumnSetting { align, is_wrapped } = v;
    let mut call = Call::new("SlackTableColumnSetting::new");
    if let Some(x) = align {
        call = call.set("with_align", emit_slack_table_column_align(x));
    }
    if let Some(x) = is_wrapped {
        call = call.set("with_is_wrapped", leaf::bool_lit(*x));
    }
    call.into()
}

pub fn emit_slack_task_card_status(v: &SlackTaskCardStatus) -> Expr {
    match v {
        SlackTaskCardStatus::Pending => leaf::unit_variant("SlackTaskCardStatus", "Pending"),
        SlackTaskCardStatus::InProgress => leaf::unit_variant("SlackTaskCardStatus", "InProgress"),
        SlackTaskCardStatus::Complete => leaf::unit_variant("SlackTaskCardStatus", "Complete"),
        SlackTaskCardStatus::Error => leaf::unit_variant("SlackTaskCardStatus", "Error"),
    }
}

pub fn emit_slack_url_source_element(v: &SlackUrlSourceElement, ctx: &mut Ctx) -> Expr {
    let SlackUrlSourceElement { url, text } = v;
    Call::new("SlackUrlSourceElement::new")
        .arg(leaf::url_expr(url, ctx))
        .arg(leaf::value_str(text))
        .into()
}

pub fn emit_slack_task_card_source(v: &SlackTaskCardSource, ctx: &mut Ctx) -> Expr {
    match v {
        SlackTaskCardSource::Url(e) => emit_slack_url_source_element(e, ctx),
    }
}

pub fn emit_slack_task_card_block(v: &SlackTaskCardBlock, ctx: &mut Ctx) -> Expr {
    let SlackTaskCardBlock {
        task_id,
        title,
        block_id,
        status,
        details,
        output,
        sources,
    } = v;
    let mut call = Call::new("SlackTaskCardBlock::new")
        .arg(leaf::value_str(task_id.value()))
        .arg(leaf::value_str(title));
    if let Some(x) = block_id {
        call = call.set("with_block_id", leaf::value_str(x.value()));
    }
    if let Some(x) = status {
        call = call.set("with_status", emit_slack_task_card_status(x));
    }
    if let Some(x) = details {
        call = call.set(
            "with_details",
            rich_text::emit_slack_rich_text_inline_content(x, ctx),
        );
    }
    if let Some(x) = output {
        call = call.set(
            "with_output",
            rich_text::emit_slack_rich_text_inline_content(x, ctx),
        );
    }
    if let Some(x) = sources {
        call = call.set(
            "with_sources",
            Expr::List {
                kind: ListKind::Vec,
                items: x
                    .iter()
                    .map(|s| emit_slack_task_card_source(s, ctx))
                    .collect(),
            },
        );
    }
    call.into()
}

pub fn emit_slack_alert_level(v: &SlackAlertLevel) -> Expr {
    match v {
        SlackAlertLevel::Warning => leaf::unit_variant("SlackAlertLevel", "Warning"),
        SlackAlertLevel::Error => leaf::unit_variant("SlackAlertLevel", "Error"),
        SlackAlertLevel::Info => leaf::unit_variant("SlackAlertLevel", "Info"),
        SlackAlertLevel::Success => leaf::unit_variant("SlackAlertLevel", "Success"),
    }
}

/// `SlackAlertBlock` declares `block_id, text, level` but only `text` is
/// required, so `new()` takes `text` alone.
pub fn emit_slack_alert_block(v: &SlackAlertBlock, ctx: &mut Ctx) -> Expr {
    let SlackAlertBlock {
        block_id,
        text,
        level,
    } = v;
    let mut call = Call::new("SlackAlertBlock::new").arg(leaf::block_text(text, ctx));
    if let Some(x) = block_id {
        call = call.set("with_block_id", leaf::value_str(x.value()));
    }
    if let Some(x) = level {
        call = call.set("with_level", emit_slack_alert_level(x));
    }
    call.into()
}

/// Read back only through `SlackCardBlock.hero_image`/`.icon` (a single-value
/// position) and `.actions` (a list position); the caller adds whichever
/// conversion the position needs, so no arm here adds its own.
pub fn emit_slack_card_image_element(v: &SlackCardImageElement, ctx: &mut Ctx) -> Expr {
    match v {
        SlackCardImageElement::Image(e) => elements::emit_slack_block_image_element(e, ctx),
    }
}

pub fn emit_slack_card_action_block_element(
    v: &SlackCardActionBlockElement,
    ctx: &mut Ctx,
) -> Expr {
    match v {
        SlackCardActionBlockElement::Button(e) => elements::emit_slack_block_button_element(e, ctx),
    }
}

/// `SlackCardBlock` declares every field as `Option`, so `new()` takes nothing.
pub fn emit_slack_card_block(v: &SlackCardBlock, ctx: &mut Ctx) -> Expr {
    let SlackCardBlock {
        block_id,
        title,
        subtitle,
        body,
        hero_image,
        icon,
        actions,
    } = v;
    let mut call = Call::new("SlackCardBlock::new");
    if let Some(x) = block_id {
        call = call.set("with_block_id", leaf::value_str(x.value()));
    }
    if let Some(x) = title {
        call = call.set("with_title", leaf::block_text(x, ctx));
    }
    if let Some(x) = subtitle {
        call = call.set("with_subtitle", leaf::block_text(x, ctx));
    }
    if let Some(x) = body {
        call = call.set("with_body", leaf::block_text(x, ctx));
    }
    if let Some(x) = hero_image {
        call = call.set(
            "with_hero_image",
            Expr::suffixed(emit_slack_card_image_element(x, ctx), ".into()"),
        );
    }
    if let Some(x) = icon {
        call = call.set(
            "with_icon",
            Expr::suffixed(emit_slack_card_image_element(x, ctx), ".into()"),
        );
    }
    if let Some(x) = actions {
        call = call.set(
            "with_actions",
            Expr::List {
                kind: ListKind::SlackBlocks,
                items: x
                    .iter()
                    .map(|a| emit_slack_card_action_block_element(a, ctx))
                    .collect(),
            },
        );
    }
    call.into()
}

pub fn emit_slack_carousel_block(v: &SlackCarouselBlock, ctx: &mut Ctx) -> Expr {
    let SlackCarouselBlock { block_id, elements } = v;
    let mut call = Call::new("SlackCarouselBlock::new").arg(Expr::List {
        kind: ListKind::SlackBlocks,
        items: elements
            .iter()
            .map(|e| blocks::emit_slack_block(e, ctx))
            .collect(),
    });
    if let Some(x) = block_id {
        call = call.set("with_block_id", leaf::value_str(x.value()));
    }
    call.into()
}

pub fn emit_slack_feedback_button_item(v: &SlackFeedbackButtonItem, ctx: &mut Ctx) -> Expr {
    let SlackFeedbackButtonItem {
        action_id,
        value,
        text,
        confirm,
    } = v;
    let mut call = Call::new("SlackFeedbackButtonItem::new")
        .arg(leaf::value_str(action_id.value()))
        .arg(leaf::value_str(value))
        .arg(leaf::plain_text_only(text, ctx));
    if let Some(x) = confirm {
        call = call.set(
            "with_confirm",
            elements::emit_slack_block_confirm_item(x, ctx),
        );
    }
    call.into()
}

pub fn emit_slack_block_feedback_buttons_element(
    v: &SlackBlockFeedbackButtonsElement,
    ctx: &mut Ctx,
) -> Expr {
    let SlackBlockFeedbackButtonsElement {
        action_id,
        positive,
        negative,
    } = v;
    Call::new("SlackBlockFeedbackButtonsElement::new")
        .arg(leaf::value_str(action_id.value()))
        .arg(emit_slack_feedback_button_item(positive, ctx))
        .arg(emit_slack_feedback_button_item(negative, ctx))
        .into()
}

pub fn emit_slack_block_icon_button_element(
    v: &SlackBlockIconButtonElement,
    ctx: &mut Ctx,
) -> Expr {
    let SlackBlockIconButtonElement {
        action_id,
        icon,
        text,
        value,
        confirm,
        accessibility_label,
        visible_to_user_ids,
    } = v;
    let mut call = Call::new("SlackBlockIconButtonElement::new")
        .arg(leaf::value_str(action_id.value()))
        .arg(leaf::value_str(icon))
        .arg(leaf::plain_text_only(text, ctx));
    if let Some(x) = value {
        call = call.set("with_value", leaf::value_str(x));
    }
    if let Some(x) = confirm {
        call = call.set(
            "with_confirm",
            elements::emit_slack_block_confirm_item(x, ctx),
        );
    }
    if let Some(x) = accessibility_label {
        call = call.set("with_accessibility_label", leaf::value_str(x.value()));
    }
    if let Some(x) = visible_to_user_ids {
        call = call.set(
            "with_visible_to_user_ids",
            Expr::List {
                kind: ListKind::Vec,
                items: x.iter().map(|u| leaf::value_str(u.value())).collect(),
            },
        );
    }
    call.into()
}

/// Every arm is spelled out: a new `SlackContextActionBlockElement` variant
/// must fail to compile here rather than fall into a silent default.
pub fn emit_slack_context_action_block_element(
    v: &SlackContextActionBlockElement,
    ctx: &mut Ctx,
) -> Expr {
    match v {
        SlackContextActionBlockElement::FeedbackButtons(e) => {
            emit_slack_block_feedback_buttons_element(e, ctx)
        }
        SlackContextActionBlockElement::IconButton(e) => {
            emit_slack_block_icon_button_element(e, ctx)
        }
    }
}

pub fn emit_slack_context_actions_block(v: &SlackContextActionsBlock, ctx: &mut Ctx) -> Expr {
    let SlackContextActionsBlock { block_id, elements } = v;
    let mut call = Call::new("SlackContextActionsBlock::new").arg(Expr::List {
        kind: ListKind::SlackBlocks,
        items: elements
            .iter()
            .map(|e| emit_slack_context_action_block_element(e, ctx))
            .collect(),
    });
    if let Some(x) = block_id {
        call = call.set("with_block_id", leaf::value_str(x.value()));
    }
    call.into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Options;
    use serde_json::json;

    #[test]
    fn spec_example_three_table_renders_verbatim() {
        let options = Options::default();
        let mut ctx = Ctx::new(&options);
        let block: SlackBlock = serde_json::from_value(json!({
            "type": "table", "rows": [
              [ { "type": "raw_text", "text": "Service" }, { "type": "raw_text", "text": "Status" } ],
              [ { "type": "raw_text", "text": "api" },
                { "type": "rich_text", "elements": [
                  { "type": "rich_text_section", "elements": [
                    { "type": "text", "text": "healthy", "style": { "bold": true } } ] } ] } ] ]
        }))
        .expect("parses");
        let mut w = crate::writer::Writer::new();
        w.open("x");
        let expected = "\
SlackTableBlock::new(vec![
        slack_blocks![\"Service\", \"Status\"],
        slack_blocks![
            \"api\",
            SlackTableRichTextCell::new(slack_blocks![
                SlackRichTextSection::new(slack_blocks![SlackRichTextText::new(\"healthy\".into()).bold()]),
            ]),
        ],
    ])";
        assert_eq!(
            crate::emit::blocks::emit_slack_block(&block, &mut ctx).render(&mut w),
            expected
        );
    }

    #[test]
    fn the_task_card_and_alert_fixtures_emit_builders() {
        let options = Options::default();
        let mut ctx = Ctx::new(&options);
        for payload in [
            include_str!("../../../../src/models/blocks/fixtures/slack_task_card_block.json"),
            include_str!("../../../../src/models/blocks/fixtures/slack_alert_block.json"),
        ] {
            let block: SlackBlock = serde_json::from_str(payload).expect("fixture parses");
            let out = crate::emit::blocks::emit_slack_block(&block, &mut ctx).flat();
            assert!(!out.contains("serde_json::from_value"), "{out}");
        }
    }

    #[test]
    fn the_card_and_context_actions_fixtures_emit_builders() {
        let options = Options::default();
        let mut ctx = Ctx::new(&options);
        for payload in [
            include_str!("../../../../src/models/blocks/fixtures/slack_card_block.json"),
            include_str!("../../../../src/models/blocks/fixtures/slack_context_actions_block.json"),
        ] {
            let block: SlackBlock = serde_json::from_str(payload).expect("fixture parses");
            let out = crate::emit::blocks::emit_slack_block(&block, &mut ctx).flat();
            assert!(!out.contains("serde_json::from_value"), "{out}");
        }
    }
}
