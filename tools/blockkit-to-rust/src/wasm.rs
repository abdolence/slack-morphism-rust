//! The browser entry point and its JSON envelope.
//!
//! Everything crossing the boundary is a JSON string: the widget is a classic
//! script with no bindings of its own, and a string keeps the glue trivial.

use serde::{Deserialize, Serialize};

use crate::{convert, EmitStyle, Options};

#[derive(Debug, Deserialize)]
#[serde(default, deny_unknown_fields)]
struct WasmOptions {
    emoji_true_is_default: bool,
    style: String,
}

impl Default for WasmOptions {
    fn default() -> Self {
        Self {
            emoji_true_is_default: true,
            style: "builders".into(),
        }
    }
}

#[derive(Debug, Serialize)]
struct Diagnostic {
    path: String,
    message: String,
}

#[derive(Debug, Serialize)]
struct Envelope {
    ok: bool,
    code: String,
    warnings: Vec<Diagnostic>,
    errors: Vec<Diagnostic>,
}

/// A last-resort envelope, built by hand so that a failure to serialize the
/// real one still produces something the widget can parse.
const SERIALIZE_FAILURE: &str = "{\"ok\":false,\"code\":\"\",\"warnings\":[],\"errors\":[{\"path\":\"\",\"message\":\"the converter could not encode its result\"}]}";

pub fn convert_envelope(input: &str, options_json: &str) -> String {
    let parsed: WasmOptions = match serde_json::from_str(options_json) {
        Ok(o) => o,
        Err(e) => return failure(&format!("bad options: {e}")),
    };
    let options = Options {
        emoji_true_is_default: parsed.emoji_true_is_default,
        style: if parsed.style == "raw" {
            EmitStyle::RawFromValue
        } else {
            EmitStyle::Builders
        },
    };

    match convert(input, &options) {
        Ok(output) => {
            let envelope = Envelope {
                ok: true,
                code: output.code,
                warnings: output
                    .warnings
                    .into_iter()
                    .map(|w| Diagnostic {
                        path: w.path,
                        message: w.message,
                    })
                    .collect(),
                errors: output
                    .errors
                    .into_iter()
                    .map(|e| Diagnostic {
                        path: diagnostic_path(&e),
                        message: e.to_string(),
                    })
                    .collect(),
            };
            serde_json::to_string(&envelope).unwrap_or_else(|_| SERIALIZE_FAILURE.to_string())
        }
        Err(e) => failure(&e.to_string()),
    }
}

fn diagnostic_path(e: &crate::ConvertError) -> String {
    match e {
        crate::ConvertError::Item { path, .. } => path.clone(),
        crate::ConvertError::Json { .. } | crate::ConvertError::Shape { .. } => String::new(),
    }
}

fn failure(message: &str) -> String {
    let envelope = Envelope {
        ok: false,
        code: String::new(),
        warnings: Vec::new(),
        errors: vec![Diagnostic {
            path: String::new(),
            message: message.to_string(),
        }],
    };
    serde_json::to_string(&envelope).unwrap_or_else(|_| SERIALIZE_FAILURE.to_string())
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn convert_json(input: &str, options_json: &str) -> String {
    convert_envelope(input, options_json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_successful_conversion_is_an_ok_envelope() {
        let out = convert_envelope(r#"[{"type":"divider"}]"#, "{}");
        let v: serde_json::Value = serde_json::from_str(&out).expect("envelope is json");
        assert_eq!(v["ok"], true);
        assert!(v["code"]
            .as_str()
            .expect("code")
            .contains("SlackDividerBlock::new()"));
        assert_eq!(v["errors"].as_array().expect("errors").len(), 0);
    }

    #[test]
    fn an_unparseable_document_is_a_not_ok_envelope_with_one_error() {
        let out = convert_envelope("nope", "{}");
        let v: serde_json::Value = serde_json::from_str(&out).expect("envelope is json");
        assert_eq!(v["ok"], false);
        assert_eq!(v["errors"].as_array().expect("errors").len(), 1);
    }

    #[test]
    fn unknown_option_keys_and_empty_options_fall_back_to_defaults() {
        let out = convert_envelope(r#"[{"type":"divider"}]"#, r#"{"style":"raw"}"#);
        let v: serde_json::Value = serde_json::from_str(&out).expect("envelope is json");
        assert!(v["code"]
            .as_str()
            .expect("code")
            .contains("serde_json::from_value"));
        let bad = convert_envelope(r#"[{"type":"divider"}]"#, "not json");
        let v: serde_json::Value = serde_json::from_str(&bad).expect("envelope is json");
        assert_eq!(v["ok"], false);
    }
}
