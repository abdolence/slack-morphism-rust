//! Turns an [`Expr`] tree into Rust source at a given indent.
//!
//! The width rule and the escaping rule live here and nowhere else: a visitor
//! describes structure and never decides layout.

const DEFAULT_WIDTH: usize = 100;
const INDENT: usize = 4;

pub struct Writer {
    buf: String,
    indent: usize,
    width: usize,
}

impl Writer {
    pub fn new() -> Self {
        Self::with_width(DEFAULT_WIDTH)
    }

    pub fn with_width(width: usize) -> Self {
        Self {
            buf: String::new(),
            indent: 0,
            width,
        }
    }

    /// Writes one complete line at the current indent.
    pub fn line(&mut self, s: &str) {
        for _ in 0..self.indent_cols() {
            self.buf.push(' ');
        }
        self.buf.push_str(s);
        self.buf.push('\n');
    }

    /// Writes a line that opens a block, then indents.
    pub fn open(&mut self, s: &str) {
        self.line(s);
        self.indent += 1;
    }

    /// Dedents, then writes the line that closes the block.
    pub fn close(&mut self, s: &str) {
        self.indent = self.indent.saturating_sub(1);
        self.line(s);
    }

    /// Appends without an indent and without a newline.
    pub fn push_str(&mut self, s: &str) {
        self.buf.push_str(s);
    }

    pub fn indent_cols(&self) -> usize {
        self.indent * INDENT
    }

    /// Whether `s` placed at the current indent stays inside the width budget.
    pub fn fits(&self, s: &str) -> bool {
        self.indent_cols() + s.chars().count() <= self.width
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn finish(self) -> String {
        self.buf
    }
}

impl Default for Writer {
    fn default() -> Self {
        Self::new()
    }
}

/// Escapes a Rust string literal body.
///
/// `{` and `}` are left alone: the one-argument `md!`/`pt!` arms wrap their
/// argument without formatting it (`src/models/blocks/dsl.rs:24-26`), so a
/// brace in the text is not a format specifier.
pub fn escape_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            '\r' => out.push_str("\\r"),
            c if c.is_control() => out.push_str(&format!("\\u{{{:x}}}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

/// Escapes and wraps in double quotes.
pub fn quoted(s: &str) -> String {
    format!("\"{}\"", escape_str(s))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_and_close_indent_by_four_columns() {
        let mut w = Writer::new();
        w.open("slack_blocks![");
        w.line("SlackDividerBlock::new(),");
        w.close("];");
        assert_eq!(
            w.finish(),
            "slack_blocks![\n    SlackDividerBlock::new(),\n];\n"
        );
    }

    #[test]
    fn escape_str_covers_the_style_contract() {
        assert_eq!(escape_str("a\"b\\c\nd\te\rf"), "a\\\"b\\\\c\\nd\\te\\rf");
        assert_eq!(escape_str("bell\u{7}"), "bell\\u{7}");
        assert_eq!(
            escape_str("braces {ok} and emoji \u{1f600}"),
            "braces {ok} and emoji \u{1f600}"
        );
    }

    #[test]
    fn fits_accounts_for_the_current_indent() {
        let mut w = Writer::with_width(20);
        assert!(w.fits("0123456789"));
        w.open("x");
        w.open("y");
        assert!(!w.fits("0123456789012"));
    }
}
