//! `SlackWorkflow`, its trigger and input parameter, and the workflow button.

use slack_morphism::prelude::*;

use crate::emit::elements;
use crate::leaf;
use crate::writer::{Call, Expr, ListKind};
use crate::Ctx;

pub fn emit_slack_block_workflow_button_element(
    v: &SlackBlockWorkflowButtonElement,
    ctx: &mut Ctx,
) -> Expr {
    let SlackBlockWorkflowButtonElement {
        action_id,
        text,
        workflow,
        style,
        accessibility_label,
    } = v;
    let mut call = Call::new("SlackBlockWorkflowButtonElement::new")
        .arg(leaf::value_str(action_id.value()))
        .arg(leaf::plain_text_only(text, ctx))
        .arg(emit_slack_workflow(workflow, ctx));
    if let Some(x) = style {
        call = call.set("with_style", elements::emit_slack_block_button_style(x));
    }
    if let Some(x) = accessibility_label {
        call = call.set("with_accessibility_label", leaf::value_str(x.value()));
    }
    call.into()
}

pub fn emit_slack_workflow(v: &SlackWorkflow, ctx: &mut Ctx) -> Expr {
    let SlackWorkflow { trigger } = v;
    Call::new("SlackWorkflow::new")
        .arg(emit_slack_workflow_trigger(trigger, ctx))
        .into()
}

pub fn emit_slack_workflow_trigger(v: &SlackWorkflowTrigger, ctx: &mut Ctx) -> Expr {
    let SlackWorkflowTrigger {
        url,
        customizable_input_parameters,
    } = v;
    let mut call = Call::new("SlackWorkflowTrigger::new").arg(leaf::url_expr(url, ctx));
    if let Some(x) = customizable_input_parameters {
        call = call.set(
            "with_customizable_input_parameters",
            Expr::List {
                kind: ListKind::Vec,
                items: x
                    .iter()
                    .map(|p| emit_slack_workflow_trigger_input_parameter(p, ctx))
                    .collect(),
            },
        );
    }
    call.into()
}

pub fn emit_slack_workflow_trigger_input_parameter(
    v: &SlackWorkflowTriggerInputParameter,
    _ctx: &mut Ctx,
) -> Expr {
    let SlackWorkflowTriggerInputParameter { name, value } = v;
    Call::new("SlackWorkflowTriggerInputParameter::new")
        .arg(leaf::value_str(name))
        .arg(leaf::value_str(value))
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Options;

    #[test]
    fn the_workflow_button_fixture_emits_builders_not_from_value() {
        let options = Options::default();
        let mut ctx = Ctx::new(&options);
        let payload =
            include_str!("../../../../src/models/blocks/fixtures/slack_workflow_button.json");
        let block: SlackBlock = serde_json::from_str(payload).expect("fixture parses");
        let out = crate::emit::blocks::emit_slack_block(&block, &mut ctx).flat();
        assert!(
            out.contains("SlackBlockWorkflowButtonElement::new("),
            "{out}"
        );
        assert!(
            out.contains("SlackWorkflow::new(SlackWorkflowTrigger::new("),
            "{out}"
        );
        assert!(!out.contains("serde_json::from_value"), "{out}");
    }
}
