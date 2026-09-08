//! Guards that cannot live beside the code they check.

use std::path::{Path, PathBuf};

use blockkit_to_rust::snapshot::{generated_path, render_generated_fixtures};

#[test]
fn generated_fixtures_are_fresh() -> Result<(), Box<dyn std::error::Error>> {
    let rendered = render_generated_fixtures()?;
    let committed = std::fs::read_to_string(generated_path())?;
    if rendered == committed {
        return Ok(());
    }
    let first_difference = rendered
        .lines()
        .zip(committed.lines())
        .enumerate()
        .find(|(_, (a, b))| a != b)
        .map(|(i, (a, b))| format!("line {}:\n  rendered:  {a}\n  committed: {b}", i + 1))
        .unwrap_or_else(|| "the files differ in length".to_string());
    panic!(
        "tests/generated_fixtures.rs is stale.\n{first_difference}\n\nRegenerate with:\n  \
         cargo run --manifest-path tools/blockkit-to-rust/Cargo.toml --bin bk2rs -- \
         --regen-fixtures"
    );
}

/// A visitor that destructures with `..` stops failing to compile when the
/// model gains a field, which is the whole drift-detection mechanism. This
/// test is what keeps the rule from eroding one convenience at a time.
#[test]
fn visitors_destructure_every_field() -> Result<(), Box<dyn std::error::Error>> {
    let emit_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/emit");
    let mut offences = Vec::new();
    for entry in std::fs::read_dir(&emit_dir)? {
        let path = entry?.path();
        if path.extension().and_then(|e| e.to_str()) != Some("rs") {
            continue;
        }
        offences.extend(scan_file(&path)?);
    }
    assert!(offences.is_empty(), "{}", offences.join("\n"));
    Ok(())
}

/// Collects `let Slack… { … } =` patterns that contain `..`.
fn scan_file(path: &Path) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let text = std::fs::read_to_string(path)?;
    let lines: Vec<&str> = text.lines().collect();
    let mut offences = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        if !line.trim_start().starts_with("let Slack") {
            continue;
        }
        let mut pattern = String::new();
        for candidate in &lines[i..] {
            pattern.push_str(candidate);
            pattern.push('\n');
            if candidate.contains("} =") {
                break;
            }
        }
        if pattern.contains("..") {
            offences.push(format!(
                "{}:{}: destructuring uses `..`; list every field",
                path.display(),
                i + 1
            ));
        }
    }
    Ok(offences)
}
