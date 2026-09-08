//! Helpers the generated fixture tests call. Public because
//! `tests/generated_fixtures.rs` is an integration test and sees only the
//! crate's public API.

use std::fmt;

use serde::Serialize;
use serde_json::Value;
use slack_morphism::prelude::*;

#[derive(Debug)]
pub struct Mismatch(pub String);

impl fmt::Display for Mismatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for Mismatch {}

/// Whether Block Kit Builder's `"emoji": true` counts as a difference.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Normalize {
    Exact,
    StripEmojiTrue,
}

pub fn parse_blocks(fixture: &str) -> Result<Vec<SlackBlock>, Box<dyn std::error::Error>> {
    let root: Value = serde_json::from_str(fixture)?;
    let items = match root {
        Value::Array(items) => Value::Array(items),
        Value::Object(mut map) => map
            .remove("blocks")
            .ok_or_else(|| Mismatch("fixture has no `blocks` array".into()))?,
        _ => {
            return Err(Box::new(Mismatch(
                "fixture is not an object or array".into(),
            )))
        }
    };
    Ok(serde_json::from_value(items)?)
}

pub fn parse_block(fixture: &str) -> Result<SlackBlock, Box<dyn std::error::Error>> {
    Ok(serde_json::from_str(fixture)?)
}

/// `slack_home_view.json` predates the `"type"` tag that Block Kit Builder now
/// puts on exported views, so the tag is supplied here rather than teaching
/// the classifier to guess at an untagged object.
pub fn as_home_view(fixture: &str) -> Result<SlackView, Box<dyn std::error::Error>> {
    Ok(serde_json::from_str(&tagged_home_view(fixture)?)?)
}

pub fn tagged_home_view(fixture: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut root: Value = serde_json::from_str(fixture)?;
    match root.as_object_mut() {
        Some(map) => {
            map.insert("type".into(), Value::String("home".into()));
        }
        None => {
            return Err(Box::new(Mismatch(
                "home view fixture is not an object".into(),
            )))
        }
    }
    Ok(serde_json::to_string(&root)?)
}

/// Removes `"emoji": true` from every object that is a `plain_text` text
/// object, leaving `"emoji": false` and every other object alone.
pub fn strip_emoji_true(v: &mut Value) {
    match v {
        Value::Object(map) => {
            let is_plain_text = map.get("type").and_then(Value::as_str) == Some("plain_text");
            if is_plain_text && map.get("emoji") == Some(&Value::Bool(true)) {
                map.remove("emoji");
            }
            for value in map.values_mut() {
                strip_emoji_true(value);
            }
        }
        Value::Array(items) => {
            for item in items {
                strip_emoji_true(item);
            }
        }
        _ => {}
    }
}

/// Every place `actual` differs from `expected`, as a JSON path with both
/// values, so a forgotten field reads
/// `blocks[0].text.verbatim: expected true, got missing`.
pub fn json_paths_diff(expected: &Value, actual: &Value, base: &str) -> Vec<String> {
    let mut out = Vec::new();
    match (expected, actual) {
        (Value::Object(a), Value::Object(b)) => {
            for (key, value) in a {
                let path = join(base, key);
                match b.get(key) {
                    None => out.push(format!("{path}: expected {value}, got missing")),
                    Some(other) => out.extend(json_paths_diff(value, other, &path)),
                }
            }
            for key in b.keys() {
                if !a.contains_key(key) {
                    let path = join(base, key);
                    out.push(format!("{path}: unexpected {}", b[key]));
                }
            }
        }
        (Value::Array(a), Value::Array(b)) => {
            if a.len() != b.len() {
                out.push(format!(
                    "{base}: expected {} items, got {}",
                    a.len(),
                    b.len()
                ));
            }
            for (i, value) in a.iter().enumerate() {
                let path = format!("{base}[{i}]");
                match b.get(i) {
                    None => out.push(format!("{path}: expected {value}, got missing")),
                    Some(other) => out.extend(json_paths_diff(value, other, &path)),
                }
            }
        }
        (a, b) if a != b => out.push(format!("{base}: expected {a}, got {b}")),
        _ => {}
    }
    out
}

fn join(base: &str, key: &str) -> String {
    if base.is_empty() {
        key.to_string()
    } else {
        format!("{base}.{key}")
    }
}

pub fn assert_same_values(built: &Value, parsed: &Value, mode: Normalize) -> Result<(), Mismatch> {
    let mut expected = parsed.clone();
    if mode == Normalize::StripEmojiTrue {
        strip_emoji_true(&mut expected);
    }
    if &expected == built {
        return Ok(());
    }
    Err(Mismatch(json_paths_diff(&expected, built, "").join("\n")))
}

/// What every generated test calls: serialize both sides, then compare.
pub fn assert_same<T: Serialize>(
    built: &T,
    parsed: &T,
    mode: Normalize,
) -> Result<(), Box<dyn std::error::Error>> {
    let built = serde_json::to_value(built)?;
    let parsed = serde_json::to_value(parsed)?;
    assert_same_values(&built, &parsed, mode)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn strip_emoji_true_removes_only_the_true_key_on_plain_text() {
        let mut v = json!({
            "type": "plain_text", "text": "a", "emoji": true,
            "nested": { "type": "plain_text", "text": "b", "emoji": false },
            "other": { "emoji": true }
        });
        strip_emoji_true(&mut v);
        assert_eq!(
            v,
            json!({
                "type": "plain_text", "text": "a",
                "nested": { "type": "plain_text", "text": "b", "emoji": false },
                "other": { "emoji": true }
            })
        );
    }

    #[test]
    fn json_paths_diff_names_the_field_that_differs() {
        let expected = json!({ "blocks": [{ "text": { "verbatim": true } }] });
        let actual = json!({ "blocks": [{ "text": {} }] });
        assert_eq!(
            json_paths_diff(&expected, &actual, ""),
            vec!["blocks[0].text.verbatim: expected true, got missing"]
        );
    }

    #[test]
    fn assert_same_passes_when_only_emoji_true_differs_under_strip() {
        let built = json!({ "type": "plain_text", "text": "a" });
        let parsed = json!({ "type": "plain_text", "text": "a", "emoji": true });
        assert!(assert_same_values(&built, &parsed, Normalize::StripEmojiTrue).is_ok());
        assert!(assert_same_values(&built, &parsed, Normalize::Exact).is_err());
    }
}
