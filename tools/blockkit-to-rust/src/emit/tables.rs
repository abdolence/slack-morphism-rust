//! Table, task card, alert, card, carousel and context-actions blocks.

use slack_morphism::prelude::*;

use crate::emit::stub;
use crate::writer::Expr;
use crate::Ctx;

pub fn emit_slack_table_block(v: &SlackTableBlock, ctx: &mut Ctx) -> Expr {
    stub(v, "table block", ctx)
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
