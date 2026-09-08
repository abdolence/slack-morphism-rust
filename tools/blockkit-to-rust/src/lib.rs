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
}
