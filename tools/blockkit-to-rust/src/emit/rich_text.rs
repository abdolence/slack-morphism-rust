//! Rich-text block, containers, inline runs, styles, and the single-run
//! collapse.

use slack_morphism::prelude::*;

use crate::leaf;
use crate::raw;
use crate::writer::{Call, Expr, ListKind};
use crate::Ctx;

pub fn emit_slack_rich_text_block(v: &SlackRichTextBlock, ctx: &mut Ctx) -> Expr {
    let SlackRichTextBlock { block_id, elements } = v;
    let mut call = Call::new("SlackRichTextBlock::new").arg(Expr::List {
        kind: ListKind::SlackBlocks,
        items: elements
            .iter()
            .map(|e| emit_slack_rich_text_element(e, ctx))
            .collect(),
    });
    if let Some(x) = block_id {
        call = call.set("with_block_id", leaf::value_str(x.value()));
    }
    call.into()
}

/// Every arm is spelled out: a new `SlackRichTextElement` variant must fail
/// to compile here rather than fall into a silent default.
pub fn emit_slack_rich_text_element(v: &SlackRichTextElement, ctx: &mut Ctx) -> Expr {
    match v {
        SlackRichTextElement::Section(e) => emit_slack_rich_text_section(e, ctx),
        SlackRichTextElement::List(e) => emit_slack_rich_text_list(e, ctx),
        SlackRichTextElement::Preformatted(e) => emit_slack_rich_text_preformatted(e, ctx),
        SlackRichTextElement::Quote(e) => emit_slack_rich_text_quote(e, ctx),
    }
}

pub fn emit_slack_rich_text_inline_content(v: &SlackRichTextInlineContent, ctx: &mut Ctx) -> Expr {
    match v {
        SlackRichTextInlineContent::RichText(b) => emit_slack_rich_text_block(b, ctx),
    }
}

pub fn emit_slack_rich_text_section(v: &SlackRichTextSection, ctx: &mut Ctx) -> Expr {
    let SlackRichTextSection { elements } = v;
    Call::new("SlackRichTextSection::new")
        .arg(Expr::List {
            kind: ListKind::SlackBlocks,
            items: elements
                .iter()
                .map(|e| emit_slack_rich_text_inline_element(e, ctx))
                .collect(),
        })
        .into()
}

pub fn emit_slack_rich_text_list_style(v: &SlackRichTextListStyle) -> Expr {
    match v {
        SlackRichTextListStyle::Bullet => leaf::unit_variant("SlackRichTextListStyle", "Bullet"),
        SlackRichTextListStyle::Ordered => leaf::unit_variant("SlackRichTextListStyle", "Ordered"),
    }
}

/// Every arm is spelled out: a new `SlackRichTextListElement` variant must
/// fail to compile here rather than fall into a silent default.
pub fn emit_slack_rich_text_list_element(v: &SlackRichTextListElement, ctx: &mut Ctx) -> Expr {
    match v {
        SlackRichTextListElement::Section(section) => match collapsible_text(section) {
            Some(text) => leaf::str_lit(text),
            None => emit_slack_rich_text_section(section, ctx),
        },
    }
}

pub fn emit_slack_rich_text_list(v: &SlackRichTextList, ctx: &mut Ctx) -> Expr {
    let SlackRichTextList {
        style,
        elements,
        indent,
        offset,
        border,
    } = v;
    let mut call = Call::new("SlackRichTextList::new")
        .arg(emit_slack_rich_text_list_style(style))
        .arg(Expr::List {
            kind: ListKind::SlackBlocks,
            items: elements
                .iter()
                .map(|e| emit_slack_rich_text_list_element(e, ctx))
                .collect(),
        });
    if let Some(x) = indent {
        call = call.set("with_indent", leaf::u64_lit(*x));
    }
    if let Some(x) = offset {
        call = call.set("with_offset", leaf::u64_lit(*x));
    }
    if let Some(x) = border {
        call = call.set("with_border", leaf::u64_lit(*x));
    }
    call.into()
}

pub fn emit_slack_rich_text_preformatted(v: &SlackRichTextPreformatted, ctx: &mut Ctx) -> Expr {
    let SlackRichTextPreformatted {
        elements,
        border,
        language,
    } = v;
    let mut call = Call::new("SlackRichTextPreformatted::new").arg(Expr::List {
        kind: ListKind::SlackBlocks,
        items: elements
            .iter()
            .map(|e| emit_slack_rich_text_inline_element(e, ctx))
            .collect(),
    });
    if let Some(x) = border {
        call = call.set("with_border", leaf::u64_lit(*x));
    }
    if let Some(x) = language {
        call = call.set("with_language", leaf::value_str(x));
    }
    call.into()
}

pub fn emit_slack_rich_text_quote(v: &SlackRichTextQuote, ctx: &mut Ctx) -> Expr {
    let SlackRichTextQuote { elements, border } = v;
    let mut call = Call::new("SlackRichTextQuote::new").arg(Expr::List {
        kind: ListKind::SlackBlocks,
        items: elements
            .iter()
            .map(|e| emit_slack_rich_text_inline_element(e, ctx))
            .collect(),
    });
    if let Some(x) = border {
        call = call.set("with_border", leaf::u64_lit(*x));
    }
    call.into()
}

/// The single unstyled run a bare `&str` would produce, if that is all the
/// section holds. `From<&str>` for the list element and the table cell both
/// build exactly this shape (`kit.rs:1234`, `:1581`), so collapsing is
/// lossless.
pub(crate) fn collapsible_text(section: &SlackRichTextSection) -> Option<&str> {
    let SlackRichTextSection { elements } = section;
    match elements.as_slice() {
        [SlackRichTextInlineElement::Text(SlackRichTextText { text, style: None })] => {
            Some(text.as_str())
        }
        _ => None,
    }
}

/// `bold`, `italic`, `strike` and `code` have chainable helpers on
/// `SlackRichTextText` (`src/models/blocks/kit.rs:1387-1415`); nothing else
/// does, and a helper only expresses `true`. A style that is exactly some
/// subset of those four set to `true` becomes a chain; anything else becomes
/// one `with_style(..)`.
enum StyleForm {
    None,
    Helpers(Vec<&'static str>),
    Full(Expr),
}

fn style_form(style: &Option<SlackRichTextStyle>, ctx: &mut Ctx) -> StyleForm {
    let Some(style) = style else {
        return StyleForm::None;
    };
    let SlackRichTextStyle {
        bold,
        italic,
        strike,
        code,
        underline,
        highlight,
        client_highlight,
        unlink,
    } = style;

    let others_absent = underline.is_none()
        && highlight.is_none()
        && client_highlight.is_none()
        && unlink.is_none();
    let chainable = [
        ("bold", bold),
        ("italic", italic),
        ("strike", strike),
        ("code", code),
    ];
    let all_true_or_absent = chainable.iter().all(|(_, v)| **v != Some(false));

    if others_absent && all_true_or_absent {
        let helpers: Vec<&'static str> = chainable
            .iter()
            .filter(|(_, v)| **v == Some(true))
            .map(|(n, _)| *n)
            .collect();
        return if helpers.is_empty() {
            StyleForm::None
        } else {
            StyleForm::Helpers(helpers)
        };
    }
    StyleForm::Full(emit_slack_rich_text_style(style, ctx))
}

/// Every flag, in declaration order, whenever the chain cannot express it.
pub fn emit_slack_rich_text_style(v: &SlackRichTextStyle, ctx: &mut Ctx) -> Expr {
    let SlackRichTextStyle {
        bold,
        italic,
        strike,
        code,
        underline,
        highlight,
        client_highlight,
        unlink,
    } = v;
    let _ = ctx;
    let mut call = Call::new("SlackRichTextStyle::new");
    for (name, value) in [
        ("with_bold", bold),
        ("with_italic", italic),
        ("with_strike", strike),
        ("with_code", code),
        ("with_underline", underline),
        ("with_highlight", highlight),
        ("with_client_highlight", client_highlight),
        ("with_unlink", unlink),
    ] {
        if let Some(x) = value {
            call = call.set(name, leaf::bool_lit(*x));
        }
    }
    call.into()
}

/// A collapsed run is only ever offered from a list position (an inline
/// element inside a section, preformatted block or quote); every caller of
/// this function is exactly such a position, so `collapse` is always `true`
/// for a `Text` element with no style and never for any other variant.
pub fn emit_slack_rich_text_text(v: &SlackRichTextText, collapse: bool, ctx: &mut Ctx) -> Expr {
    let SlackRichTextText { text, style } = v;
    match style_form(style, ctx) {
        StyleForm::None if collapse => leaf::str_lit(text),
        StyleForm::None => Call::new("SlackRichTextText::new")
            .arg(leaf::value_str(text))
            .into(),
        StyleForm::Helpers(helpers) => {
            let mut call = Call::new("SlackRichTextText::new").arg(leaf::value_str(text));
            for helper in helpers {
                call = call.set0(helper);
            }
            call.into()
        }
        StyleForm::Full(style_expr) => Call::new("SlackRichTextText::new")
            .arg(leaf::value_str(text))
            .set("with_style", style_expr)
            .into(),
    }
}

pub fn emit_slack_rich_text_link(v: &SlackRichTextLink, ctx: &mut Ctx) -> Expr {
    let SlackRichTextLink {
        url,
        text,
        unsafe_,
        style,
    } = v;
    let mut call = Call::new("SlackRichTextLink::new").arg(leaf::relaxed_url(url));
    if let Some(x) = text {
        call = call.set("with_text", leaf::value_str(x));
    }
    if let Some(x) = unsafe_ {
        call = call.set("with_unsafe_", leaf::bool_lit(*x));
    }
    if let Some(x) = style {
        call = call.set("with_style", emit_slack_rich_text_style(x, ctx));
    }
    call.into()
}

pub fn emit_slack_rich_text_user(v: &SlackRichTextUser, ctx: &mut Ctx) -> Expr {
    let SlackRichTextUser { user_id, style } = v;
    let mut call = Call::new("SlackRichTextUser::new").arg(leaf::value_str(user_id.value()));
    if let Some(x) = style {
        call = call.set("with_style", emit_slack_rich_text_style(x, ctx));
    }
    call.into()
}

pub fn emit_slack_rich_text_channel(v: &SlackRichTextChannel, ctx: &mut Ctx) -> Expr {
    let SlackRichTextChannel { channel_id, style } = v;
    let mut call = Call::new("SlackRichTextChannel::new").arg(leaf::value_str(channel_id.value()));
    if let Some(x) = style {
        call = call.set("with_style", emit_slack_rich_text_style(x, ctx));
    }
    call.into()
}

pub fn emit_slack_rich_text_user_group(v: &SlackRichTextUserGroup, ctx: &mut Ctx) -> Expr {
    let SlackRichTextUserGroup {
        usergroup_id,
        style,
    } = v;
    let mut call =
        Call::new("SlackRichTextUserGroup::new").arg(leaf::value_str(usergroup_id.value()));
    if let Some(x) = style {
        call = call.set("with_style", emit_slack_rich_text_style(x, ctx));
    }
    call.into()
}

pub fn emit_slack_rich_text_emoji(v: &SlackRichTextEmoji, _ctx: &mut Ctx) -> Expr {
    let SlackRichTextEmoji { name, unicode } = v;
    let mut call = Call::new("SlackRichTextEmoji::new").arg(leaf::value_str(name.value()));
    if let Some(x) = unicode {
        call = call.set("with_unicode", leaf::value_str(x));
    }
    call.into()
}

pub fn emit_slack_rich_text_date(v: &SlackRichTextDate, ctx: &mut Ctx) -> Expr {
    let SlackRichTextDate {
        timestamp,
        format,
        fallback,
        style,
    } = v;
    let mut call = Call::new("SlackRichTextDate::new")
        .arg(leaf::datetime_expr(timestamp, ctx))
        .arg(leaf::value_str(format));
    if let Some(x) = fallback {
        call = call.set("with_fallback", leaf::value_str(x));
    }
    if let Some(x) = style {
        call = call.set("with_style", emit_slack_rich_text_style(x, ctx));
    }
    call.into()
}

pub fn emit_slack_rich_text_broadcast_range(v: &SlackRichTextBroadcastRange) -> Expr {
    match v {
        SlackRichTextBroadcastRange::Here => {
            leaf::unit_variant("SlackRichTextBroadcastRange", "Here")
        }
        SlackRichTextBroadcastRange::Channel => {
            leaf::unit_variant("SlackRichTextBroadcastRange", "Channel")
        }
        SlackRichTextBroadcastRange::Everyone => {
            leaf::unit_variant("SlackRichTextBroadcastRange", "Everyone")
        }
    }
}

pub fn emit_slack_rich_text_broadcast(v: &SlackRichTextBroadcast, ctx: &mut Ctx) -> Expr {
    let SlackRichTextBroadcast { range, style } = v;
    let mut call =
        Call::new("SlackRichTextBroadcast::new").arg(emit_slack_rich_text_broadcast_range(range));
    if let Some(x) = style {
        call = call.set("with_style", emit_slack_rich_text_style(x, ctx));
    }
    call.into()
}

pub fn emit_slack_rich_text_color(v: &SlackRichTextColor, _ctx: &mut Ctx) -> Expr {
    let SlackRichTextColor { value } = v;
    Call::new("SlackRichTextColor::new")
        .arg(leaf::value_str(value))
        .into()
}

pub fn emit_slack_rich_text_message_mention(
    v: &SlackRichTextMessageMention,
    ctx: &mut Ctx,
) -> Expr {
    let SlackRichTextMessageMention {
        url,
        text,
        channel_id,
        author_id,
        message_ts,
        thread_ts,
        style,
    } = v;
    let mut call = Call::new("SlackRichTextMessageMention::new").arg(leaf::relaxed_url(url));
    if let Some(x) = text {
        call = call.set("with_text", leaf::value_str(x));
    }
    if let Some(x) = channel_id {
        call = call.set("with_channel_id", leaf::value_str(x.value()));
    }
    if let Some(x) = author_id {
        call = call.set("with_author_id", leaf::value_str(x.value()));
    }
    if let Some(x) = message_ts {
        call = call.set("with_message_ts", leaf::value_str(x.value()));
    }
    if let Some(x) = thread_ts {
        call = call.set("with_thread_ts", leaf::value_str(x.value()));
    }
    if let Some(x) = style {
        call = call.set("with_style", emit_slack_rich_text_style(x, ctx));
    }
    call.into()
}

/// Every arm is spelled out: a new `SlackRichTextInlineElement` variant must
/// fail to compile here rather than fall into a silent default.
pub fn emit_slack_rich_text_inline_element(v: &SlackRichTextInlineElement, ctx: &mut Ctx) -> Expr {
    match v {
        SlackRichTextInlineElement::Text(e) => emit_slack_rich_text_text(e, true, ctx),
        SlackRichTextInlineElement::Link(e) => emit_slack_rich_text_link(e, ctx),
        SlackRichTextInlineElement::User(e) => emit_slack_rich_text_user(e, ctx),
        SlackRichTextInlineElement::Channel(e) => emit_slack_rich_text_channel(e, ctx),
        SlackRichTextInlineElement::UserGroup(e) => emit_slack_rich_text_user_group(e, ctx),
        SlackRichTextInlineElement::Emoji(e) => emit_slack_rich_text_emoji(e, ctx),
        SlackRichTextInlineElement::Date(e) => emit_slack_rich_text_date(e, ctx),
        SlackRichTextInlineElement::Broadcast(e) => emit_slack_rich_text_broadcast(e, ctx),
        SlackRichTextInlineElement::Color(e) => emit_slack_rich_text_color(e, ctx),
        SlackRichTextInlineElement::MessageMention(e) => {
            emit_slack_rich_text_message_mention(e, ctx)
        }
        SlackRichTextInlineElement::Unknown(value) => {
            Call::new("SlackRichTextInlineElement::Unknown")
                .arg(raw::json_macro(value, ctx))
                .into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Options;
    use serde_json::json;

    fn inline(value: serde_json::Value) -> String {
        let options = Options::default();
        let mut ctx = Ctx::new(&options);
        let element: SlackRichTextInlineElement = serde_json::from_value(value).expect("parses");
        emit_slack_rich_text_inline_element(&element, &mut ctx).flat()
    }

    #[test]
    fn an_unstyled_run_collapses_to_a_bare_literal() {
        assert_eq!(
            inline(json!({ "type": "text", "text": "Build " })),
            "\"Build \""
        );
    }

    #[test]
    fn a_bold_only_run_chains_the_helper() {
        assert_eq!(
            inline(json!({ "type": "text", "text": "passed", "style": { "bold": true } })),
            "SlackRichTextText::new(\"passed\".into()).bold()"
        );
    }

    #[test]
    fn a_run_with_any_other_flag_uses_with_style() {
        assert_eq!(
            inline(json!({ "type": "text", "text": "t", "style": { "underline": true } })),
            "SlackRichTextText::new(\"t\".into())\
             .with_style(SlackRichTextStyle::new().with_underline(true))"
        );
    }

    #[test]
    fn a_false_flag_is_not_a_chainable_helper() {
        assert_eq!(
            inline(json!({ "type": "text", "text": "t", "style": { "bold": false } })),
            "SlackRichTextText::new(\"t\".into())\
             .with_style(SlackRichTextStyle::new().with_bold(false))"
        );
    }

    #[test]
    fn an_unknown_inline_element_goes_through_the_json_macro() {
        let out = inline(json!({ "type": "made_up", "x": 1 }));
        assert_eq!(
            out,
            "SlackRichTextInlineElement::Unknown(json!({ \"type\": \"made_up\", \"x\": 1 }))"
        );
    }

    #[test]
    fn spec_example_three_rich_text_renders_verbatim() {
        let options = Options::default();
        let mut ctx = Ctx::new(&options);
        let block: SlackBlock = serde_json::from_value(json!({
            "type": "rich_text", "elements": [
              { "type": "rich_text_section", "elements": [
                { "type": "text", "text": "Build " },
                { "type": "text", "text": "passed", "style": { "bold": true } },
                { "type": "text", "text": " for " },
                { "type": "link", "url": "https://ci.example.com/run/42", "text": "run 42" } ] },
              { "type": "rich_text_list", "style": "bullet", "elements": [
                { "type": "rich_text_section", "elements": [ { "type": "text", "text": "unit tests" } ] },
                { "type": "rich_text_section", "elements": [ { "type": "text", "text": "clippy" } ] } ] } ]
        }))
        .expect("parses");
        let mut w = crate::writer::Writer::new();
        w.open("x");
        let expected = "\
SlackRichTextBlock::new(slack_blocks![
        SlackRichTextSection::new(slack_blocks![
            \"Build \",
            SlackRichTextText::new(\"passed\".into()).bold(),
            \" for \",
            SlackRichTextLink::new(\"https://ci.example.com/run/42\".into())
                .with_text(\"run 42\".into()),
        ]),
        SlackRichTextList::new(
            SlackRichTextListStyle::Bullet,
            slack_blocks![\"unit tests\", \"clippy\"],
        ),
    ])";
        assert_eq!(
            crate::emit::blocks::emit_slack_block(&block, &mut ctx).render(&mut w),
            expected
        );
    }
}
