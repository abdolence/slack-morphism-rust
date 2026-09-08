//! Decides what the pasted document is, and reports what the model did not
//! keep.

use serde_json::Value;

use crate::{ConvertError, Warning};

/// The four accepted input shapes of the Block Kit Builder, collapsed to
/// three: a message and a bare array both produce a list of blocks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputShape {
    Blocks(Vec<Value>),
    Block(Value),
    View(Value),
}

impl InputShape {
    /// The name the emitted snippet binds the result to.
    pub fn binding(&self) -> &'static str {
        match self {
            InputShape::Blocks(_) => "blocks",
            InputShape::Block(_) => "block",
            InputShape::View(_) => "view",
        }
    }
}

pub fn classify(root: &Value) -> Result<InputShape, ConvertError> {
    match root {
        Value::Array(items) => Ok(InputShape::Blocks(items.clone())),
        Value::Object(map) => match map.get("type").and_then(Value::as_str) {
            Some("modal") | Some("home") => Ok(InputShape::View(root.clone())),
            Some(_) => Ok(InputShape::Block(root.clone())),
            None => match map.get("blocks") {
                Some(Value::Array(items)) => Ok(InputShape::Blocks(items.clone())),
                _ => Err(ConvertError::Shape {
                    message: "expected a Block Kit message, a block array, a single block \
                              or a modal or home view"
                        .into(),
                }),
            },
        },
        _ => Err(ConvertError::Shape {
            message: "expected a JSON object or array".into(),
        }),
    }
}

/// Every path present in `input` and absent from `reparsed`.
///
/// No block struct sets `deny_unknown_fields`, so a field the crate does not
/// model deserializes without complaint and then vanishes on the way back out.
/// Comparing the input with a re-serialization of what was parsed is the only
/// way to see it. Object keys come out in `serde_json::Map`'s own order —
/// sorted, since this crate does not enable `preserve_order`.
pub fn dropped_paths(input: &Value, reparsed: &Value, base: &str) -> Vec<String> {
    let mut out = Vec::new();
    match (input, reparsed) {
        (Value::Object(a), Value::Object(b)) => {
            for (key, value) in a {
                let path = if base.is_empty() {
                    key.clone()
                } else {
                    format!("{base}.{key}")
                };
                match b.get(key) {
                    None => out.push(path),
                    Some(other) => out.extend(dropped_paths(value, other, &path)),
                }
            }
        }
        (Value::Array(a), Value::Array(b)) => {
            for (i, value) in a.iter().enumerate() {
                let path = format!("{base}[{i}]");
                match b.get(i) {
                    None => out.push(path),
                    Some(other) => out.extend(dropped_paths(value, other, &path)),
                }
            }
        }
        _ => {}
    }
    out
}

pub fn dropped_warnings(input: &Value, reparsed: &Value, base: &str) -> Vec<Warning> {
    dropped_paths(input, reparsed, base)
        .into_iter()
        .map(|path| Warning {
            message: format!("{path} is not modelled by slack-morphism and was dropped"),
            path,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn a_builder_message_classifies_as_blocks() {
        let v = json!({ "blocks": [{ "type": "divider" }] });
        match classify(&v).expect("classify") {
            InputShape::Blocks(items) => assert_eq!(items.len(), 1),
            other => panic!("expected Blocks, got {other:?}"),
        }
    }

    #[test]
    fn a_bare_array_classifies_as_blocks() {
        let v = json!([{ "type": "divider" }, { "type": "divider" }]);
        match classify(&v).expect("classify") {
            InputShape::Blocks(items) => assert_eq!(items.len(), 2),
            other => panic!("expected Blocks, got {other:?}"),
        }
    }

    #[test]
    fn a_modal_type_classifies_as_a_view_not_a_block() {
        let v = json!({ "type": "modal", "title": {}, "blocks": [] });
        assert!(matches!(
            classify(&v).expect("classify"),
            InputShape::View(_)
        ));
    }

    #[test]
    fn a_single_typed_object_classifies_as_one_block() {
        let v = json!({ "type": "section", "text": {} });
        assert!(matches!(
            classify(&v).expect("classify"),
            InputShape::Block(_)
        ));
    }

    #[test]
    fn an_unrecognised_shape_is_an_error_not_a_guess() {
        let v = json!({ "hello": "world" });
        assert!(matches!(classify(&v), Err(ConvertError::Shape { .. })));
    }

    #[test]
    fn dropped_fields_are_listed_as_warnings() {
        let input = json!({ "blocks": [{ "type": "divider", "foo": 1, "bar": { "baz": 2 } }] });
        let reparsed = json!({ "blocks": [{ "type": "divider", "bar": {} }] });
        let paths = dropped_paths(&input, &reparsed, "");
        // `serde_json::Map` is a `BTreeMap` without the `preserve_order`
        // feature, so keys iterate sorted rather than in input order.
        assert_eq!(paths, vec!["blocks[0].bar.baz", "blocks[0].foo"]);
    }
}
