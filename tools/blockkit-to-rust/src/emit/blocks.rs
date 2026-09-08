//! `SlackBlock` and the block structs P1 owns.

use slack_morphism::prelude::*;

use crate::emit::{elements, rich_text, tables};
use crate::leaf;
use crate::raw;
use crate::writer::{Call, Expr, ListKind};
use crate::Ctx;

/// Every arm is spelled out: a new `SlackBlock` variant must fail to compile
/// here rather than fall into a silent default.
pub fn emit_slack_block(v: &SlackBlock, ctx: &mut Ctx) -> Expr {
    match v {
        SlackBlock::Section(b) => emit_slack_section_block(b, ctx),
        SlackBlock::Header(b) => emit_slack_header_block(b, ctx),
        SlackBlock::Divider(b) => emit_slack_divider_block(b, ctx),
        SlackBlock::Image(b) => emit_slack_image_block(b, ctx),
        SlackBlock::Actions(b) => emit_slack_actions_block(b, ctx),
        SlackBlock::Context(b) => emit_slack_context_block(b, ctx),
        SlackBlock::Input(b) => emit_slack_input_block(b, ctx),
        SlackBlock::File(b) => emit_slack_file_block(b, ctx),
        SlackBlock::Video(b) => emit_slack_video_block(b, ctx),
        SlackBlock::Markdown(b) => emit_slack_markdown_block(b, ctx),
        SlackBlock::RichText(b) => rich_text::emit_slack_rich_text_block(b, ctx),
        SlackBlock::Table(b) => tables::emit_slack_table_block(b, ctx),
        SlackBlock::TaskCard(b) => tables::emit_slack_task_card_block(b, ctx),
        SlackBlock::Alert(b) => tables::emit_slack_alert_block(b, ctx),
        SlackBlock::Card(b) => tables::emit_slack_card_block(b, ctx),
        SlackBlock::Carousel(b) => tables::emit_slack_carousel_block(b, ctx),
        SlackBlock::ContextActions(b) => tables::emit_slack_context_actions_block(b, ctx),
        SlackBlock::ShareShortcut(value) => Call::new("SlackBlock::ShareShortcut")
            .arg(raw::json_macro(value, ctx))
            .into(),
        SlackBlock::Event(value) => Call::new("SlackBlock::Event")
            .arg(raw::json_macro(value, ctx))
            .into(),
    }
}

/// `SlackSectionBlock` declares `block_id, text, fields, accessory, expand`
/// (`src/models/blocks/kit.rs:65-71`), all optional, so `new()` takes nothing.
pub fn emit_slack_section_block(v: &SlackSectionBlock, ctx: &mut Ctx) -> Expr {
    let SlackSectionBlock {
        block_id,
        text,
        fields,
        accessory,
        expand,
    } = v;
    let mut call = Call::new("SlackSectionBlock::new");
    if let Some(x) = block_id {
        call = call.set("with_block_id", leaf::value_str(x.value()));
    }
    if let Some(x) = text {
        call = call.set("with_text", leaf::block_text(x, ctx));
    }
    if let Some(x) = fields {
        call = call.set(
            "with_fields",
            Expr::List {
                kind: ListKind::Vec,
                items: x.iter().map(|t| leaf::block_text(t, ctx)).collect(),
            },
        );
    }
    if let Some(x) = accessory {
        call = call.set(
            "with_accessory",
            Expr::suffixed(
                elements::emit_slack_section_block_element(x, ctx),
                ".into()",
            ),
        );
    }
    if let Some(x) = expand {
        call = call.set("with_expand", leaf::bool_lit(*x));
    }
    call.into()
}

/// `SlackHeaderBlock` declares `block_id` before `text`, but only `text` is
/// required, so `new()` takes `text` alone.
pub fn emit_slack_header_block(v: &SlackHeaderBlock, ctx: &mut Ctx) -> Expr {
    let SlackHeaderBlock { block_id, text } = v;
    let mut call = Call::new("SlackHeaderBlock::new").arg(leaf::plain_text_only(text, ctx));
    if let Some(x) = block_id {
        call = call.set("with_block_id", leaf::value_str(x.value()));
    }
    call.into()
}

pub fn emit_slack_divider_block(v: &SlackDividerBlock, _ctx: &mut Ctx) -> Expr {
    let SlackDividerBlock { block_id } = v;
    let mut call = Call::new("SlackDividerBlock::new");
    if let Some(x) = block_id {
        call = call.set("with_block_id", leaf::value_str(x.value()));
    }
    call.into()
}

pub fn emit_slack_image_block(v: &SlackImageBlock, ctx: &mut Ctx) -> Expr {
    let SlackImageBlock {
        block_id,
        image_url_or_file,
        alt_text,
        title,
    } = v;
    let mut call = Call::new("SlackImageBlock::new")
        .arg(emit_slack_image_url_or_file(image_url_or_file, ctx))
        .arg(leaf::value_str(alt_text));
    if let Some(x) = block_id {
        call = call.set("with_block_id", leaf::value_str(x.value()));
    }
    if let Some(x) = title {
        call = call.set("with_title", leaf::plain_text_only(x, ctx));
    }
    call.into()
}

pub fn emit_slack_actions_block(v: &SlackActionsBlock, ctx: &mut Ctx) -> Expr {
    let SlackActionsBlock { block_id, elements } = v;
    let mut call = Call::new("SlackActionsBlock::new").arg(Expr::List {
        kind: ListKind::SlackBlocks,
        items: elements
            .iter()
            .map(|e| elements::emit_slack_action_block_element(e, ctx))
            .collect(),
    });
    if let Some(x) = block_id {
        call = call.set("with_block_id", leaf::value_str(x.value()));
    }
    call.into()
}

pub fn emit_slack_context_block(v: &SlackContextBlock, ctx: &mut Ctx) -> Expr {
    let SlackContextBlock { block_id, elements } = v;
    let mut call = Call::new("SlackContextBlock::new").arg(Expr::List {
        kind: ListKind::Vec,
        items: elements
            .iter()
            .map(|e| elements::emit_slack_context_block_element(e, ctx))
            .collect(),
    });
    if let Some(x) = block_id {
        call = call.set("with_block_id", leaf::value_str(x.value()));
    }
    call.into()
}

pub fn emit_slack_input_block(v: &SlackInputBlock, ctx: &mut Ctx) -> Expr {
    let SlackInputBlock {
        block_id,
        label,
        element,
        hint,
        optional,
        dispatch_action,
    } = v;
    let mut call = Call::new("SlackInputBlock::new")
        .arg(leaf::plain_text_only(label, ctx))
        .arg(Expr::suffixed(
            elements::emit_slack_input_block_element(element, ctx),
            ".into()",
        ));
    if let Some(x) = block_id {
        call = call.set("with_block_id", leaf::value_str(x.value()));
    }
    if let Some(x) = hint {
        call = call.set("with_hint", leaf::plain_text_only(x, ctx));
    }
    if let Some(x) = optional {
        call = call.set("with_optional", leaf::bool_lit(*x));
    }
    if let Some(x) = dispatch_action {
        call = call.set("with_dispatch_action", leaf::bool_lit(*x));
    }
    call.into()
}

pub fn emit_slack_file_block(v: &SlackFileBlock, _ctx: &mut Ctx) -> Expr {
    let SlackFileBlock {
        block_id,
        external_id,
        source,
    } = v;
    let mut call = Call::new("SlackFileBlock::new").arg(leaf::value_str(external_id));
    if let Some(x) = block_id {
        call = call.set("with_block_id", leaf::value_str(x.value()));
    }
    call = call.set("with_source", leaf::value_str(source));
    call.into()
}

pub fn emit_slack_video_block(v: &SlackVideoBlock, ctx: &mut Ctx) -> Expr {
    let SlackVideoBlock {
        alt_text,
        author_name,
        block_id,
        description,
        provider_icon_url,
        provider_name,
        title,
        title_url,
        thumbnail_url,
        video_url,
    } = v;
    let mut call = Call::new("SlackVideoBlock::new")
        .arg(leaf::value_str(alt_text))
        .arg(leaf::plain_text_only(title, ctx))
        .arg(leaf::url_expr(thumbnail_url, ctx))
        .arg(leaf::url_expr(video_url, ctx));
    if let Some(x) = author_name {
        call = call.set("with_author_name", leaf::value_str(x));
    }
    if let Some(x) = block_id {
        call = call.set("with_block_id", leaf::value_str(x.value()));
    }
    if let Some(x) = description {
        call = call.set("with_description", leaf::plain_text_only(x, ctx));
    }
    if let Some(x) = provider_icon_url {
        call = call.set("with_provider_icon_url", leaf::url_expr(x, ctx));
    }
    if let Some(x) = provider_name {
        call = call.set("with_provider_name", leaf::value_str(x));
    }
    if let Some(x) = title_url {
        call = call.set("with_title_url", leaf::url_expr(x, ctx));
    }
    call.into()
}

pub fn emit_slack_markdown_block(v: &SlackMarkdownBlock, _ctx: &mut Ctx) -> Expr {
    let SlackMarkdownBlock { block_id, text } = v;
    let mut call = Call::new("SlackMarkdownBlock::new").arg(leaf::value_str(text));
    if let Some(x) = block_id {
        call = call.set("with_block_id", leaf::value_str(x.value()));
    }
    call.into()
}

/// `SlackImageUrlOrFile` is `#[serde(untagged)]` and flattened into the image
/// block and element, so the JSON key is what selects the variant.
pub fn emit_slack_image_url_or_file(v: &SlackImageUrlOrFile, ctx: &mut Ctx) -> Expr {
    match v {
        SlackImageUrlOrFile::ImageUrl { image_url } => leaf::url_into_expr(image_url, ctx),
        SlackImageUrlOrFile::SlackFile { slack_file } => Expr::Struct {
            head: "SlackImageUrlOrFile::SlackFile".into(),
            fields: vec![(
                "slack_file".into(),
                emit_slack_file_id_or_url(slack_file, ctx),
            )],
        },
    }
}

pub fn emit_slack_file_id_or_url(v: &SlackFileIdOrUrl, ctx: &mut Ctx) -> Expr {
    match v {
        SlackFileIdOrUrl::Id { id } => Expr::Struct {
            head: "SlackFileIdOrUrl::Id".into(),
            fields: vec![("id".into(), leaf::value_str(id.value()))],
        },
        SlackFileIdOrUrl::Url { url } => Expr::Struct {
            head: "SlackFileIdOrUrl::Url".into(),
            fields: vec![("url".into(), leaf::url_expr(url, ctx))],
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Options;
    use serde_json::json;

    fn emit(value: serde_json::Value) -> String {
        let options = Options::default();
        let mut ctx = Ctx::new(&options);
        let block: SlackBlock = serde_json::from_value(value).expect("fixture parses");
        emit_slack_block(&block, &mut ctx).render(&mut crate::writer::Writer::new())
    }

    #[test]
    fn section_and_divider_match_the_style_contract() {
        assert_eq!(
            emit(json!({
                "type": "section",
                "text": { "type": "mrkdwn", "text": "Deploy *finished* for `api`." }
            })),
            "SlackSectionBlock::new().with_text(md!(\"Deploy *finished* for `api`.\"))"
        );
        assert_eq!(
            emit(json!({ "type": "divider" })),
            "SlackDividerBlock::new()"
        );
    }

    #[test]
    fn section_fields_use_vec_of_macros() {
        assert_eq!(
            emit(json!({
                "type": "section",
                "fields": [
                    { "type": "mrkdwn", "text": "a" },
                    { "type": "plain_text", "text": "b" }
                ]
            })),
            "SlackSectionBlock::new().with_fields(vec![md!(\"a\"), pt!(\"b\")])"
        );
    }

    #[test]
    fn context_elements_use_vec_of_macros() {
        assert_eq!(
            emit(json!({
                "type": "context",
                "elements": [{ "type": "mrkdwn", "text": "a" }]
            })),
            "SlackContextBlock::new(vec![md!(\"a\")])"
        );
    }

    #[test]
    fn a_defaulted_field_is_emitted_whenever_the_json_carries_it() {
        assert_eq!(
            emit(json!({ "type": "file", "external_id": "F1", "source": "remote" })),
            "SlackFileBlock::new(\"F1\".into()).with_source(\"remote\".into())"
        );
    }

    #[test]
    fn passthrough_variants_emit_json_macro() {
        let out = emit(json!({ "type": "event", "payload": { "a": 1 } }));
        assert_eq!(
            out,
            "SlackBlock::Event(json!({ \"payload\": { \"a\": 1 } }))"
        );
    }
}
