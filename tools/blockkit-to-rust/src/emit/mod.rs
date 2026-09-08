pub mod blocks;
pub mod elements;
pub mod rich_text;
pub mod tables;
pub mod views;
pub mod workflow;

use crate::writer::Expr;
use crate::{ConvertError, Ctx};

/// Re-serializes a typed value so a stub visitor can hand it to
/// `raw::not_yet_emitted`. Serialization of a value the crate itself just
/// produced cannot fail; if it somehow does, the caller gets an error item
/// rather than a panic.
pub fn stub<T: serde::Serialize>(v: &T, kind: &str, ctx: &mut Ctx) -> Expr {
    match serde_json::to_value(v) {
        Ok(value) => crate::raw::not_yet_emitted(&value, ctx),
        Err(e) => {
            ctx.errors.push(ConvertError::Item {
                path: ctx.path.clone(),
                message: format!("could not re-serialize {kind}: {e}"),
            });
            Expr::atom("Default::default()")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Ctx, Options};
    use slack_morphism::prelude::*;

    #[test]
    fn a_stub_visitor_emits_the_from_value_path_and_records_no_error() {
        let options = Options::default();
        let mut ctx = Ctx::new(&options);
        let block = SlackTableBlock::new(vec![vec!["a".into()]]);
        let out = tables::emit_slack_table_block(&block, &mut ctx).flat();
        assert!(out.starts_with("serde_json::from_value(json!("), "{out}");
        assert!(ctx.errors.is_empty());
        assert!(ctx.needs_json && ctx.needs_result);
    }
}
