//! `SlackView`, `SlackModalView`, `SlackHomeView`.

use slack_morphism::prelude::*;

use crate::emit::stub;
use crate::writer::Expr;
use crate::Ctx;

pub fn emit_slack_view(v: &SlackView, ctx: &mut Ctx) -> Expr {
    stub(v, "view", ctx)
}

pub fn emit_slack_home_view(v: &SlackHomeView, ctx: &mut Ctx) -> Expr {
    stub(v, "home view", ctx)
}

pub fn emit_slack_modal_view(v: &SlackModalView, ctx: &mut Ctx) -> Expr {
    stub(v, "modal view", ctx)
}
