//! Table, task card, alert, card, carousel and context-actions blocks.

use slack_morphism::prelude::*;

use crate::emit::{rich_text, stub};
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

pub fn emit_slack_task_card_block(v: &SlackTaskCardBlock, ctx: &mut Ctx) -> Expr {
    stub(v, "task card block", ctx)
}

pub fn emit_slack_alert_block(v: &SlackAlertBlock, ctx: &mut Ctx) -> Expr {
    stub(v, "alert block", ctx)
}

pub fn emit_slack_card_block(v: &SlackCardBlock, ctx: &mut Ctx) -> Expr {
    stub(v, "card block", ctx)
}

pub fn emit_slack_carousel_block(v: &SlackCarouselBlock, ctx: &mut Ctx) -> Expr {
    stub(v, "carousel block", ctx)
}

pub fn emit_slack_context_actions_block(v: &SlackContextActionsBlock, ctx: &mut Ctx) -> Expr {
    stub(v, "context actions block", ctx)
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
}
