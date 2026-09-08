//! Renders the leaf values of the model: scalars, newtypes, URLs, timestamps
//! and the text objects.

use slack_morphism::prelude::*;
use url::Url;

use crate::writer::{quoted, Call, Expr};
use crate::Ctx;

/// A plain `&str` literal, for a position that wants `&str`.
pub fn str_lit(s: &str) -> Expr {
    Expr::Atom(quoted(s))
}

/// A literal for any position that accepts `impl Into<_>` from a string:
/// `String` itself and every `ValueStruct` newtype.
pub fn value_str(s: &str) -> Expr {
    Expr::Atom(format!("{}.into()", quoted(s)))
}

pub fn bool_lit(b: bool) -> Expr {
    Expr::Atom(if b { "true".into() } else { "false".into() })
}

pub fn u64_lit(n: u64) -> Expr {
    Expr::Atom(n.to_string())
}

pub fn unit_variant(type_name: &str, variant: &str) -> Expr {
    Expr::Atom(format!("{type_name}::{variant}"))
}

/// `Url::parse("..")?` for a field typed `Url`.
pub fn url_expr(u: &Url, ctx: &mut Ctx) -> Expr {
    ctx.needs_url = true;
    ctx.needs_result = true;
    Expr::suffixed(Call::new("Url::parse").arg(str_lit(u.as_str())).into(), "?")
}

/// `Url::parse("..")?.into()` where the target has a `From<Url>`.
pub fn url_into_expr(u: &Url, ctx: &mut Ctx) -> Expr {
    ctx.needs_url = true;
    ctx.needs_result = true;
    Expr::suffixed(
        Call::new("Url::parse").arg(str_lit(u.as_str())).into(),
        "?.into()",
    )
}

/// `SlackRelaxedUrl` holds an unvalidated string, so it takes the newtype form.
pub fn relaxed_url(u: &SlackRelaxedUrl) -> Expr {
    value_str(u.value())
}

/// `SlackDateTime` wraps `SlackUtcDateTime`, whose `Display` is RFC 3339 under
/// the default jiff backend. The companion pins `default-features = false`, so
/// that is always the backend in use; the round-trip test in this module is
/// what fails if that ever stops being true.
///
/// The literal carries a `?`, so this takes a `Ctx` rather than leaving the
/// flag to its two callers to remember.
pub fn datetime_expr(dt: &SlackDateTime, ctx: &mut Ctx) -> Expr {
    ctx.needs_result = true;
    Expr::Atom(format!(
        "SlackDateTime({}.parse()?)",
        quoted(&dt.value().to_string())
    ))
}

/// `pt!(t)` when the object carries nothing but text, the builder form
/// otherwise. Under `emoji_true_is_default` an `"emoji": true` counts as
/// nothing, which is what keeps Block Kit Builder labels compact.
pub fn plain_text(v: &SlackBlockPlainText, ctx: &mut Ctx) -> Expr {
    let SlackBlockPlainText { text, emoji } = v;
    let emoji_is_noise = ctx.options.emoji_true_is_default && *emoji == Some(true);
    if emoji.is_none() || emoji_is_noise {
        return Expr::Atom(format!("pt!({})", quoted(text)));
    }
    let mut call = Call::new("SlackBlockPlainText::new").arg(value_str(text));
    if let Some(e) = emoji {
        call = call.set("with_emoji", bool_lit(*e));
    }
    Expr::suffixed(call.into(), ".into()")
}

pub fn markdown_text(v: &SlackBlockMarkDownText, ctx: &mut Ctx) -> Expr {
    let SlackBlockMarkDownText { text, verbatim } = v;
    let _ = ctx;
    if verbatim.is_none() {
        return Expr::Atom(format!("md!({})", quoted(text)));
    }
    let mut call = Call::new("SlackBlockMarkDownText::new").arg(value_str(text));
    if let Some(v) = verbatim {
        call = call.set("with_verbatim", bool_lit(*v));
    }
    Expr::suffixed(call.into(), ".into()")
}

pub fn block_text(v: &SlackBlockText, ctx: &mut Ctx) -> Expr {
    match v {
        SlackBlockText::Plain(t) => plain_text(t, ctx),
        SlackBlockText::MarkDown(t) => markdown_text(t, ctx),
    }
}

/// `SlackBlockPlainTextOnly` keeps its inner value private, so it is read
/// through the crate's own `From` impl rather than a new accessor.
pub fn plain_text_only(v: &SlackBlockPlainTextOnly, ctx: &mut Ctx) -> Expr {
    block_text(&SlackBlockText::from(v.clone()), ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Ctx, Options};

    #[test]
    fn value_struct_newtypes_use_into_not_to_string() {
        assert_eq!(value_str("btn-1").flat(), "\"btn-1\".into()");
    }

    #[test]
    fn url_fields_parse_with_a_question_mark_and_set_the_import_flags() {
        let options = Options::default();
        let mut ctx = Ctx::new(&options);
        let url = Url::parse("https://example.com/a b").expect("test url");
        let expr = url_into_expr(&url, &mut ctx);
        assert_eq!(
            expr.flat(),
            "Url::parse(\"https://example.com/a%20b\")?.into()"
        );
        assert!(ctx.needs_url);
        assert!(ctx.needs_result);
    }

    #[test]
    fn slack_date_time_literal_round_trips_through_parse() {
        let options = Options::default();
        let mut ctx = Ctx::new(&options);
        let dt = SlackDateTime("2020-01-01T00:42:42Z".parse().expect("test timestamp"));
        assert_eq!(
            datetime_expr(&dt, &mut ctx).flat(),
            "SlackDateTime(\"2020-01-01T00:42:42Z\".parse()?)"
        );
        assert!(ctx.needs_result);
        let reparsed: SlackUtcDateTime = "2020-01-01T00:42:42Z".parse().expect("reparse");
        assert_eq!(SlackDateTime(reparsed), dt);
    }

    #[test]
    fn unit_variants_render_as_type_colon_colon_variant() {
        assert_eq!(
            unit_variant("SlackBlockButtonStyle", "Primary").flat(),
            "SlackBlockButtonStyle::Primary"
        );
    }

    #[test]
    fn emoji_true_is_pt_under_default_option() {
        let options = Options::default();
        let mut ctx = Ctx::new(&options);
        let text = SlackBlockPlainText::new("Approve".into()).with_emoji(true);
        assert_eq!(plain_text(&text, &mut ctx).flat(), "pt!(\"Approve\")");
    }

    #[test]
    fn emoji_true_is_builder_form_when_exact() {
        let options = Options {
            emoji_true_is_default: false,
            ..Options::default()
        };
        let mut ctx = Ctx::new(&options);
        let text = SlackBlockPlainText::new("Approve".into()).with_emoji(true);
        assert_eq!(
            plain_text(&text, &mut ctx).flat(),
            "SlackBlockPlainText::new(\"Approve\".into()).with_emoji(true).into()"
        );
    }

    #[test]
    fn emoji_false_always_takes_the_builder_form() {
        let options = Options::default();
        let mut ctx = Ctx::new(&options);
        let text = SlackBlockPlainText::new("Approve".into()).with_emoji(false);
        assert_eq!(
            plain_text(&text, &mut ctx).flat(),
            "SlackBlockPlainText::new(\"Approve\".into()).with_emoji(false).into()"
        );
    }

    #[test]
    fn markdown_with_verbatim_takes_the_builder_form() {
        let options = Options::default();
        let mut ctx = Ctx::new(&options);
        let plain = SlackBlockMarkDownText::new("a *b*".into());
        assert_eq!(markdown_text(&plain, &mut ctx).flat(), "md!(\"a *b*\")");
        let verbatim = SlackBlockMarkDownText::new("a *b*".into()).with_verbatim(true);
        assert_eq!(
            markdown_text(&verbatim, &mut ctx).flat(),
            "SlackBlockMarkDownText::new(\"a *b*\".into()).with_verbatim(true).into()"
        );
    }

    #[test]
    fn plain_text_only_reads_through_the_existing_from_impl() {
        let options = Options::default();
        let mut ctx = Ctx::new(&options);
        let only: SlackBlockPlainTextOnly = "Title".into();
        assert_eq!(plain_text_only(&only, &mut ctx).flat(), "pt!(\"Title\")");
    }
}
