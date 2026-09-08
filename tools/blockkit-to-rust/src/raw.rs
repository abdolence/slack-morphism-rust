//! The two escape hatches: passthrough `serde_json::Value` variants, and the
//! documented `from_value` path for anything the emitter cannot hand-build.

use serde_json::Value;

use crate::writer::{quoted, Call, Expr, ListKind};
use crate::Ctx;

/// A `json!` body: the JSON value as Rust-literal syntax, no macro wrapper.
///
/// `ctx` carries no signal for a raw literal today, but the signature matches
/// every other visitor's so a caller need not special-case this one.
#[allow(clippy::only_used_in_recursion)]
pub fn json_value(v: &Value, ctx: &mut Ctx) -> Expr {
    match v {
        Value::Null => Expr::atom("null"),
        Value::Bool(b) => Expr::Atom(b.to_string()),
        Value::Number(n) => Expr::Atom(n.to_string()),
        Value::String(s) => Expr::Atom(quoted(s)),
        Value::Array(items) => Expr::List {
            kind: ListKind::JsonArray,
            items: items.iter().map(|i| json_value(i, ctx)).collect(),
        },
        Value::Object(map) => Expr::Struct {
            head: String::new(),
            fields: map
                .iter()
                .map(|(k, v)| (quoted(k), json_value(v, ctx)))
                .collect(),
        },
    }
}

/// `json!(<body>)`, for `SlackBlock::Event`, `SlackBlock::ShareShortcut` and
/// `SlackRichTextInlineElement::Unknown`.
pub fn json_macro(v: &Value, ctx: &mut Ctx) -> Expr {
    ctx.needs_json = true;
    Call::new("json!").arg(json_value(v, ctx)).into()
}

/// Spec 5.5 case 2: the crate parses it, this package does not hand-build it
/// yet. The emitted call is the one the converter itself just made, so it
/// always compiles and always round-trips.
pub fn not_yet_emitted(v: &Value, ctx: &mut Ctx) -> Expr {
    ctx.needs_result = true;
    Expr::suffixed(
        Call::new("serde_json::from_value")
            .arg(json_macro(v, ctx))
            .into(),
        "?",
    )
}

/// The whole document under `EmitStyle::RawFromValue`.
pub fn raw_from_value(v: &Value, ctx: &mut Ctx) -> Expr {
    not_yet_emitted(v, ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Ctx, Options};
    use serde_json::json;

    #[test]
    fn passthrough_variants_emit_json_macro() {
        let options = Options::default();
        let mut ctx = Ctx::new(&options);
        let expr = json_macro(&json!({ "a": 1, "b": [true, null, "x"] }), &mut ctx);
        assert_eq!(
            expr.flat(),
            "json!({ \"a\": 1, \"b\": [true, null, \"x\"] })"
        );
        assert!(ctx.needs_json);
    }

    #[test]
    fn not_yet_emitted_uses_the_documented_from_value_path() {
        let options = Options::default();
        let mut ctx = Ctx::new(&options);
        let expr = not_yet_emitted(&json!({ "type": "table" }), &mut ctx);
        assert_eq!(
            expr.flat(),
            "serde_json::from_value(json!({ \"type\": \"table\" }))?"
        );
        assert!(ctx.needs_json);
        assert!(ctx.needs_result);
    }

    #[test]
    fn json_strings_go_through_the_same_escaping_as_rust_literals() {
        let options = Options::default();
        let mut ctx = Ctx::new(&options);
        assert_eq!(
            json_value(&json!("a\"b\nc"), &mut ctx).flat(),
            "\"a\\\"b\\nc\""
        );
    }
}
