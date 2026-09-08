//! Rich-text block, containers, inline runs, styles, and the single-run
//! collapse.

use slack_morphism::prelude::*;

use crate::emit::stub;
use crate::writer::Expr;
use crate::Ctx;

pub fn emit_slack_rich_text_block(v: &SlackRichTextBlock, ctx: &mut Ctx) -> Expr {
    stub(v, "rich text block", ctx)
}

pub fn emit_slack_rich_text_element(v: &SlackRichTextElement, ctx: &mut Ctx) -> Expr {
    stub(v, "rich text element", ctx)
}

pub fn emit_slack_rich_text_inline_content(v: &SlackRichTextInlineContent, ctx: &mut Ctx) -> Expr {
    stub(v, "rich text inline content", ctx)
}
