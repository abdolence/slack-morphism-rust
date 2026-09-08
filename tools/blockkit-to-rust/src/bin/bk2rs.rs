//! `bk2rs` converts Block Kit JSON on stdin into slack-morphism builder code,
//! and regenerates the committed fixture snapshots.

use std::io::Read;
use std::process::ExitCode;

use blockkit_to_rust::snapshot::{generated_path, render_generated_fixtures};
use blockkit_to_rust::{convert, EmitStyle, Options};

#[derive(Debug)]
enum Command {
    Convert(Options),
    RegenFixtures,
}

const USAGE: &str = "\
usage: bk2rs [--exact] [--raw] < input.json
       bk2rs --regen-fixtures

  --exact            keep `\"emoji\": true`, so the output round-trips byte-exact
  --raw              emit `serde_json::from_value` for the whole document
  --regen-fixtures   rewrite tests/generated_fixtures.rs from the fixture corpus
";

fn parse_args(args: &[String]) -> Result<Command, String> {
    if args.iter().any(|a| a == "--regen-fixtures") {
        if args.len() > 1 {
            return Err("--regen-fixtures takes no other flags".into());
        }
        return Ok(Command::RegenFixtures);
    }
    let mut options = Options::default();
    for arg in args {
        match arg.as_str() {
            "--exact" => options.emoji_true_is_default = false,
            "--raw" => options.style = EmitStyle::RawFromValue,
            other => return Err(format!("unknown argument `{other}`")),
        }
    }
    Ok(Command::Convert(options))
}

fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(message) => {
            eprintln!("{message}");
            eprintln!("{USAGE}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<ExitCode, String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match parse_args(&args)? {
        Command::RegenFixtures => {
            let rendered = render_generated_fixtures().map_err(|e| e.to_string())?;
            let path = generated_path();
            std::fs::write(&path, rendered).map_err(|e| format!("{}: {e}", path.display()))?;
            eprintln!("wrote {}", path.display());
            Ok(ExitCode::SUCCESS)
        }
        Command::Convert(options) => {
            let mut source = String::new();
            std::io::stdin()
                .read_to_string(&mut source)
                .map_err(|e| e.to_string())?;
            let output = convert(&source, &options).map_err(|e| e.to_string())?;
            print!("{}", output.code);
            for warning in &output.warnings {
                eprintln!("warning: {}", warning.message);
            }
            for error in &output.errors {
                eprintln!("error: {error}");
            }
            Ok(if output.errors.is_empty() {
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flags_map_onto_options() {
        let o = parse_args(&["--exact".into(), "--raw".into()]).expect("parses");
        match o {
            Command::Convert(options) => {
                assert!(!options.emoji_true_is_default);
                assert_eq!(options.style, EmitStyle::RawFromValue);
            }
            other => panic!("expected Convert, got {other:?}"),
        }
    }

    #[test]
    fn regen_is_its_own_command_and_takes_no_other_flags() {
        assert!(matches!(
            parse_args(&["--regen-fixtures".into()]),
            Ok(Command::RegenFixtures)
        ));
        assert!(parse_args(&["--nope".into()]).is_err());
    }
}
