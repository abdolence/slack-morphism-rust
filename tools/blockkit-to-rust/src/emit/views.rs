//! `SlackView`, `SlackModalView`, `SlackHomeView`.

use slack_morphism::prelude::*;

use crate::emit::blocks;
use crate::leaf;
use crate::writer::{Call, Expr, ListKind};
use crate::Ctx;

/// Wrapped in the explicit variant rather than `.into()`: `SlackModalView` and
/// `SlackHomeView` have no `From` impl into `SlackView`, only the enum's own
/// constructors do.
pub fn emit_slack_view(v: &SlackView, ctx: &mut Ctx) -> Expr {
    match v {
        SlackView::Home(view) => Call::new("SlackView::Home")
            .arg(emit_slack_home_view(view, ctx))
            .into(),
        SlackView::Modal(view) => Call::new("SlackView::Modal")
            .arg(emit_slack_modal_view(view, ctx))
            .into(),
    }
}

/// `SlackHomeView` declares `blocks` before its three `NoneAsEmptyString`
/// fields (`src/models/blocks/view.rs:22-32`), all optional, so `new()` takes
/// `blocks` alone.
pub fn emit_slack_home_view(v: &SlackHomeView, ctx: &mut Ctx) -> Expr {
    let SlackHomeView {
        blocks,
        private_metadata,
        callback_id,
        external_id,
    } = v;
    let mut call = Call::new("SlackHomeView::new").arg(emit_view_blocks(blocks, ctx));
    if let Some(x) = private_metadata {
        call = call.set("with_private_metadata", leaf::value_str(x));
    }
    if let Some(x) = callback_id {
        call = call.set("with_callback_id", leaf::value_str(x.value()));
    }
    if let Some(x) = external_id {
        call = call.set("with_external_id", leaf::value_str(x));
    }
    call.into()
}

/// `SlackModalView` declares `title, blocks` as its only required fields
/// (`src/models/blocks/view.rs:38-55`); every setter below follows in
/// declaration order.
pub fn emit_slack_modal_view(v: &SlackModalView, ctx: &mut Ctx) -> Expr {
    let SlackModalView {
        title,
        blocks,
        close,
        submit,
        private_metadata,
        callback_id,
        clear_on_close,
        notify_on_close,
        hash,
        external_id,
    } = v;
    let mut call = Call::new("SlackModalView::new")
        .arg(leaf::plain_text_only(title, ctx))
        .arg(emit_view_blocks(blocks, ctx));
    if let Some(x) = close {
        call = call.set("with_close", leaf::plain_text_only(x, ctx));
    }
    if let Some(x) = submit {
        call = call.set("with_submit", leaf::plain_text_only(x, ctx));
    }
    if let Some(x) = private_metadata {
        call = call.set("with_private_metadata", leaf::value_str(x));
    }
    if let Some(x) = callback_id {
        call = call.set("with_callback_id", leaf::value_str(x.value()));
    }
    if let Some(x) = clear_on_close {
        call = call.set("with_clear_on_close", leaf::bool_lit(*x));
    }
    if let Some(x) = notify_on_close {
        call = call.set("with_notify_on_close", leaf::bool_lit(*x));
    }
    if let Some(x) = hash {
        call = call.set("with_hash", leaf::value_str(x));
    }
    if let Some(x) = external_id {
        call = call.set("with_external_id", leaf::value_str(x));
    }
    call.into()
}

fn emit_view_blocks(blocks: &[SlackBlock], ctx: &mut Ctx) -> Expr {
    Expr::List {
        kind: ListKind::SlackBlocks,
        items: blocks
            .iter()
            .map(|b| blocks::emit_slack_block(b, ctx))
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use crate::{convert, Options};

    #[test]
    fn a_modal_view_binds_the_explicit_modal_variant_and_orders_setters_by_declaration() {
        let out = convert(
            r#"{ "type": "modal", "callback_id": "cb", "title": { "type": "plain_text", "text": "T" },
                 "submit": { "type": "plain_text", "text": "Go" }, "blocks": [ { "type": "divider" } ] }"#,
            &Options::default(),
        )
        .expect("converts");
        assert!(
            out.code.contains("let view: SlackView = SlackView::Modal("),
            "{}",
            out.code
        );
        assert!(
            out.code.contains(
                "SlackModalView::new(pt!(\"T\"), slack_blocks![SlackDividerBlock::new()])"
            ),
            "{}",
            out.code
        );
        let submit_at = out
            .code
            .find(".with_submit(pt!(\"Go\"))")
            .expect("submit setter present");
        let callback_at = out
            .code
            .find(".with_callback_id(\"cb\".into())")
            .expect("callback_id setter present");
        assert!(
            submit_at < callback_at,
            "submit must be emitted before callback_id: {}",
            out.code
        );
    }

    #[test]
    fn a_home_view_binds_the_explicit_home_variant() {
        let out = convert(
            r#"{ "type": "home", "blocks": [ { "type": "divider" } ] }"#,
            &Options::default(),
        )
        .expect("converts");
        assert!(
            out.code.contains(
                "SlackView::Home(SlackHomeView::new(slack_blocks![SlackDividerBlock::new()]))"
            ),
            "{}",
            out.code
        );
    }
}
