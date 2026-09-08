//! Converts Slack Block Kit JSON into slack-morphism builder code.

pub mod emit;
pub mod input;
pub mod leaf;
pub mod raw;
pub mod writer;

#[cfg(not(target_arch = "wasm32"))]
pub mod snapshot;
pub mod testkit;

#[cfg(target_arch = "wasm32")]
mod wasm;

use std::fmt;

/// How much of Block Kit Builder's boilerplate the emitter is allowed to drop.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Options {
    /// Builder stamps `"emoji": true` on every `plain_text` object. When true
    /// the key is treated as absent, so labels render as `pt!(..)`; when false
    /// they take the builder form and the output round-trips byte-exact.
    pub emoji_true_is_default: bool,
    pub style: EmitStyle,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            emoji_true_is_default: true,
            style: EmitStyle::Builders,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmitStyle {
    Builders,
    RawFromValue,
}

/// What `convert` produces when at least some of the input was understood.
///
/// `errors` is non-empty when individual items failed; the code still contains
/// every item that converted, with a comment in place of each that did not.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Output {
    pub code: String,
    pub warnings: Vec<Warning>,
    pub errors: Vec<ConvertError>,
}

/// A field the crate parsed past: present in the input, absent from the model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Warning {
    pub path: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConvertError {
    /// The document is not JSON at all.
    Json { message: String },
    /// One item did not deserialize into the model.
    Item { path: String, message: String },
    /// The top-level shape is not one the converter recognises.
    Shape { message: String },
}

impl fmt::Display for ConvertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConvertError::Json { message } => write!(f, "{message}"),
            ConvertError::Item { path, message } => write!(f, "{path}: {message}"),
            ConvertError::Shape { message } => write!(f, "{message}"),
        }
    }
}

impl std::error::Error for ConvertError {}

/// Carried through every visitor: the options in force, the imports the
/// emitted snippet turned out to need, and the diagnostics collected so far.
///
/// `path` is set once per top-level item and is not refined further; a
/// diagnostic raised deep inside a block is attributed to that block.
pub struct Ctx<'a> {
    pub options: &'a Options,
    pub path: String,
    pub needs_json: bool,
    pub needs_url: bool,
    pub needs_result: bool,
    pub warnings: Vec<Warning>,
    pub errors: Vec<ConvertError>,
}

impl<'a> Ctx<'a> {
    pub fn new(options: &'a Options) -> Self {
        Self {
            options,
            path: String::new(),
            needs_json: false,
            needs_url: false,
            needs_result: false,
            warnings: Vec::new(),
            errors: Vec::new(),
        }
    }
}

/// Converts a pasted Block Kit document into slack-morphism builder code.
///
/// `Err` means nothing could be produced: the document is not JSON, or its
/// top-level shape is not one of the four the Builder emits. A document whose
/// individual items fail still returns `Ok`, with those items reported in
/// `Output::errors` and replaced by a comment in the code.
pub fn convert(source: &str, options: &Options) -> Result<Output, ConvertError> {
    let root: serde_json::Value = serde_json::from_str(source).map_err(|e| ConvertError::Json {
        message: e.to_string(),
    })?;

    let shape = input::classify(&root)?;
    let mut ctx = Ctx::new(options);

    let (binding_type, body) = match (&shape, options.style) {
        (_, EmitStyle::RawFromValue) => {
            let value = match &shape {
                input::InputShape::Blocks(items) => serde_json::Value::Array(items.clone()),
                input::InputShape::Block(v) | input::InputShape::View(v) => v.clone(),
            };
            (
                raw_binding_type(&shape),
                raw::raw_from_value(&value, &mut ctx),
            )
        }
        (input::InputShape::Blocks(items), _) => {
            ("Vec<SlackBlock>", emit_block_list(items, &mut ctx))
        }
        (input::InputShape::Block(item), _) => (
            // `emit_slack_block`'s arms return each variant's own struct type
            // (matching its use as a bare item inside `slack_blocks![..]`, the
            // only other place it is called), so a single-block binding needs
            // its own conversion into `SlackBlock` here; a list binding gets
            // one for free from the macro's per-item `.into()`.
            "SlackBlock",
            writer::Expr::suffixed(emit_one_block(item, 0, &mut ctx), ".into()"),
        ),
        (input::InputShape::View(item), _) => ("SlackView", emit_one_view(item, &mut ctx)),
    };

    let mut writer = writer::Writer::new();
    let rendered = body.render(&mut writer);
    let code = assemble(&ctx, binding_type, shape.binding(), &rendered);

    Ok(Output {
        code,
        warnings: ctx.warnings,
        errors: ctx.errors,
    })
}

fn raw_binding_type(shape: &input::InputShape) -> &'static str {
    match shape {
        input::InputShape::Blocks(_) => "Vec<SlackBlock>",
        input::InputShape::Block(_) => "SlackBlock",
        input::InputShape::View(_) => "SlackView",
    }
}

fn emit_block_list(items: &[serde_json::Value], ctx: &mut Ctx) -> writer::Expr {
    writer::Expr::List {
        kind: writer::ListKind::SlackBlocks,
        items: items
            .iter()
            .enumerate()
            .map(|(i, item)| emit_one_block(item, i, ctx))
            .collect(),
    }
}

/// Deserializes one block on its own so a single bad block does not cost the
/// whole document, and so the error carries `blocks[i]`.
fn emit_one_block(item: &serde_json::Value, index: usize, ctx: &mut Ctx) -> writer::Expr {
    let path = format!("blocks[{index}]");
    ctx.path = path.clone();
    match serde_json::from_value::<slack_morphism::prelude::SlackBlock>(item.clone()) {
        Ok(block) => {
            collect_dropped(item, &block, &path, ctx);
            emit::blocks::emit_slack_block(&block, ctx)
        }
        Err(e) => {
            let message = e.to_string();
            ctx.errors.push(ConvertError::Item {
                path: path.clone(),
                message: message.clone(),
            });
            writer::Expr::Commented {
                comment: format!("// {path}: not converted: {message}"),
                inner: Box::new(raw::not_yet_emitted_as(item, ctx, "SlackBlock")),
            }
        }
    }
}

fn collect_dropped<T: serde::Serialize>(
    item: &serde_json::Value,
    parsed: &T,
    path: &str,
    ctx: &mut Ctx,
) {
    if let Ok(reparsed) = serde_json::to_value(parsed) {
        ctx.warnings
            .extend(input::dropped_warnings(item, &reparsed, path));
    }
}

/// A view deserializes whole: `SlackModalView` owns its blocks
/// (`src/models/blocks/view.rs:38-42`), so there is no per-block recovery here.
fn emit_one_view(item: &serde_json::Value, ctx: &mut Ctx) -> writer::Expr {
    ctx.path = "view".into();
    match serde_json::from_value::<slack_morphism::prelude::SlackView>(item.clone()) {
        Ok(view) => {
            collect_dropped(item, &view, "view", ctx);
            emit::views::emit_slack_view(&view, ctx)
        }
        Err(e) => {
            let message = e.to_string();
            ctx.errors.push(ConvertError::Item {
                path: "view".into(),
                message: message.clone(),
            });
            writer::Expr::Commented {
                comment: format!("// view: not converted: {message}"),
                inner: Box::new(raw::not_yet_emitted(item, ctx)),
            }
        }
    }
}

/// Assembles the header, the optional notes and the binding.
fn assemble(ctx: &Ctx, binding_type: &str, binding: &str, rendered: &str) -> String {
    let mut out = String::from("use slack_morphism::prelude::*;\n");
    if ctx.needs_json {
        out.push_str("use serde_json::json;\n");
    }
    if ctx.needs_url {
        out.push_str("use url::Url;\n");
    }
    out.push('\n');
    if ctx.needs_url {
        out.push_str("// `url` must be a dependency of your crate; slack-morphism uses url 2.\n");
    }
    if ctx.needs_result {
        out.push_str("// This snippet uses `?`, so place it in a function returning `Result`.\n");
    }
    out.push_str(&format!("let {binding}: {binding_type} = {rendered};\n"));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_options_treat_emoji_true_as_default() {
        let options = Options::default();
        assert!(options.emoji_true_is_default);
        assert_eq!(options.style, EmitStyle::Builders);
    }

    #[test]
    fn convert_error_displays_its_json_path() {
        let error = ConvertError::Item {
            path: "blocks[2]".into(),
            message: "unknown variant `foo`".into(),
        };
        assert_eq!(error.to_string(), "blocks[2]: unknown variant `foo`");
    }

    #[test]
    fn spec_example_one_round_trips_from_paste_to_snippet() {
        let out = convert(
            r#"{ "blocks": [
                 { "type": "section", "text": { "type": "mrkdwn", "text": "Deploy *finished* for `api`." } },
                 { "type": "divider" } ] }"#,
            &Options::default(),
        )
        .expect("converts");
        assert_eq!(
            out.code,
            "use slack_morphism::prelude::*;\n\
             \n\
             let blocks: Vec<SlackBlock> = slack_blocks![\n    \
                 SlackSectionBlock::new().with_text(md!(\"Deploy *finished* for `api`.\")),\n    \
                 SlackDividerBlock::new(),\n\
             ];\n"
        );
        assert!(out.warnings.is_empty());
        assert!(out.errors.is_empty());
    }

    #[test]
    fn single_block_binding_converts_into_slack_block() {
        let out = convert(r#"{ "type": "divider" }"#, &Options::default()).expect("converts");
        assert_eq!(
            out.code,
            "use slack_morphism::prelude::*;\n\
             \n\
             let block: SlackBlock = SlackDividerBlock::new().into();\n"
        );
    }

    #[test]
    fn single_unknown_block_falls_back_with_a_typed_from_value() {
        let out = convert(r#"{ "type": "nope" }"#, &Options::default()).expect("converts");
        assert!(
            out.code
                .contains("serde_json::from_value::<SlackBlock>(json!("),
            "fallback must name its target type so `.into()` can infer:\n{}",
            out.code
        );
        assert!(out.code.contains(")?.into();\n"), "{}", out.code);
        assert_eq!(out.errors.len(), 1);
    }

    #[test]
    fn unknown_block_type_reports_path_and_keeps_other_blocks() {
        let out = convert(
            r#"[{ "type": "divider" }, { "type": "nope" }, { "type": "divider" }]"#,
            &Options::default(),
        )
        .expect("converts");
        assert_eq!(out.errors.len(), 1);
        assert_eq!(
            out.errors[0],
            ConvertError::Item {
                path: "blocks[1]".into(),
                message: "unknown variant `nope`, expected one of `section`, `header`, \
                          `divider`, `image`, `actions`, `context`, `input`, `file`, `video`, \
                          `markdown`, `rich_text`, `table`, `task_card`, `alert`, `card`, \
                          `carousel`, `context_actions`, `share_shortcut`, `event`"
                    .into(),
            }
        );
        assert!(out.code.contains("// blocks[1]: not converted:"));
        assert_eq!(out.code.matches("SlackDividerBlock::new()").count(), 2);
    }

    #[test]
    fn dropped_fields_are_listed_as_warnings() {
        let out = convert(
            r#"{ "blocks": [{ "type": "divider", "made_up": true }] }"#,
            &Options::default(),
        )
        .expect("converts");
        assert_eq!(out.warnings.len(), 1);
        assert_eq!(out.warnings[0].path, "blocks[0].made_up");
    }

    #[test]
    fn a_url_snippet_carries_both_the_import_and_the_result_note() {
        let out = convert(
            r#"{ "blocks": [{ "type": "image", "image_url": "https://example.com/a.png",
                              "alt_text": "a" }] }"#,
            &Options::default(),
        )
        .expect("converts");
        assert!(out
            .code
            .starts_with("use slack_morphism::prelude::*;\nuse url::Url;\n"));
        assert!(out
            .code
            .contains("// `url` must be a dependency of your crate"));
        assert!(out.code.contains("// This snippet uses `?`"));
    }

    #[test]
    fn not_json_is_an_error_not_an_empty_snippet() {
        assert!(matches!(
            convert("this is not json", &Options::default()),
            Err(ConvertError::Json { .. })
        ));
    }
}
