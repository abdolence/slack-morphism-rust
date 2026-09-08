//! `SlackWorkflow`, its trigger and input parameter, and the workflow button.

use slack_morphism::prelude::*;

use crate::emit::stub;
use crate::writer::Expr;
use crate::Ctx;

pub fn emit_slack_block_workflow_button_element(
    v: &SlackBlockWorkflowButtonElement,
    ctx: &mut Ctx,
) -> Expr {
    stub(v, "workflow button", ctx)
}

pub fn emit_slack_workflow(v: &SlackWorkflow, ctx: &mut Ctx) -> Expr {
    stub(v, "workflow", ctx)
}

pub fn emit_slack_workflow_trigger(v: &SlackWorkflowTrigger, ctx: &mut Ctx) -> Expr {
    stub(v, "workflow trigger", ctx)
}

pub fn emit_slack_workflow_trigger_input_parameter(
    v: &SlackWorkflowTriggerInputParameter,
    ctx: &mut Ctx,
) -> Expr {
    stub(v, "workflow trigger input parameter", ctx)
}
