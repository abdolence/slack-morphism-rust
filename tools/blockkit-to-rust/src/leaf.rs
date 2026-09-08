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
}
