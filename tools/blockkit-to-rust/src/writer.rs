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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ListKind {
    /// `slack_blocks![..]`, for a `Vec` whose element type has `From` impls
    /// from its item structs or from `&str`.
    SlackBlocks,
    /// `vec![..]`, for element types whose items are `md!`/`pt!` or a generic
    /// `SlackBlockChoiceItem`, which cannot be bare `slack_blocks!` items.
    Vec,
    /// `[..]` inside a `json!` body.
    JsonArray,
}

impl ListKind {
    fn open(&self) -> &'static str {
        match self {
            ListKind::SlackBlocks => "slack_blocks![",
            ListKind::Vec => "vec![",
            ListKind::JsonArray => "[",
        }
    }

    fn close(&self) -> &'static str {
        "]"
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Setter {
    pub name: String,
    pub args: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    /// Rendered verbatim; never split.
    Atom(String),
    Call {
        head: String,
        args: Vec<Expr>,
        setters: Vec<Setter>,
    },
    List {
        kind: ListKind,
        items: Vec<Expr>,
    },
    /// `head { name: value, .. }`. With an empty `head` this is a `json!`
    /// object, whose names arrive already quoted.
    Struct {
        head: String,
        fields: Vec<(String, Expr)>,
    },
    /// Appends literal text such as `?`, `.into()` or `?.into()` to the last
    /// rendered line.
    Suffix {
        inner: Box<Expr>,
        suffix: String,
    },
    /// A line comment placed above the item, used for items that failed to
    /// convert.
    Commented {
        comment: String,
        inner: Box<Expr>,
    },
}

impl Expr {
    pub fn atom(s: &str) -> Self {
        Expr::Atom(s.to_string())
    }

    pub fn suffixed(inner: Expr, suffix: &str) -> Self {
        Expr::Suffix {
            inner: Box::new(inner),
            suffix: suffix.to_string(),
        }
    }

    /// The single-line form, whatever its length.
    pub fn flat(&self) -> String {
        match self {
            Expr::Atom(s) => s.clone(),
            Expr::Call {
                head,
                args,
                setters,
            } => {
                let args = args.iter().map(Expr::flat).collect::<Vec<_>>().join(", ");
                let setters = setters
                    .iter()
                    .map(|s| {
                        format!(
                            ".{}({})",
                            s.name,
                            s.args.iter().map(Expr::flat).collect::<Vec<_>>().join(", ")
                        )
                    })
                    .collect::<String>();
                format!("{head}({args}){setters}")
            }
            Expr::List { kind, items } => format!(
                "{}{}{}",
                kind.open(),
                items.iter().map(Expr::flat).collect::<Vec<_>>().join(", "),
                kind.close()
            ),
            Expr::Struct { head, fields } => {
                let body = fields
                    .iter()
                    .map(|(n, v)| format!("{n}: {}", v.flat()))
                    .collect::<Vec<_>>()
                    .join(", ");
                if head.is_empty() {
                    format!("{{ {body} }}")
                } else {
                    format!("{head} {{ {body} }}")
                }
            }
            Expr::Suffix { inner, suffix } => format!("{}{suffix}", inner.flat()),
            Expr::Commented { inner, .. } => inner.flat(),
        }
    }

    /// Renders at the writer's current indent. The first line carries no
    /// leading indent; the caller has already placed it.
    pub fn render(&self, w: &mut Writer) -> String {
        let indent = w.indent_cols();
        self.render_at(indent, w.width())
    }

    fn render_at(&self, indent: usize, width: usize) -> String {
        if let Expr::Commented { comment, inner } = self {
            let pad = " ".repeat(indent);
            return format!("{comment}\n{pad}{}", inner.render_at(indent, width));
        }

        let flat = self.flat();
        if indent + flat.chars().count() <= width && !flat.contains('\n') {
            return flat;
        }

        match self {
            Expr::Atom(s) => s.clone(),
            Expr::Suffix { inner, suffix } => {
                format!("{}{suffix}", inner.render_at(indent, width))
            }
            Expr::List { kind, items } => {
                render_items(kind.open(), kind.close(), items, indent, width)
            }
            Expr::Struct { head, fields } => {
                let open = if head.is_empty() {
                    "{".to_string()
                } else {
                    format!("{head} {{")
                };
                let pad = " ".repeat(indent + INDENT);
                let base = " ".repeat(indent);
                let body = fields
                    .iter()
                    .map(|(n, v)| format!("{pad}{n}: {},\n", v.render_at(indent + INDENT, width)))
                    .collect::<String>();
                format!("{open}\n{body}{base}}}")
            }
            Expr::Call {
                head,
                args,
                setters,
            } => {
                let hug =
                    args.len() == 1 && matches!(args[0], Expr::List { .. } | Expr::Struct { .. });
                let (call_text, args_were_multiline) = if hug {
                    let rendered = args[0].render_at(indent, width);
                    let multiline = rendered.contains('\n');
                    (format!("{head}({rendered})"), multiline)
                } else if args.is_empty() {
                    (format!("{head}()"), false)
                } else {
                    let flat_args = args.iter().map(Expr::flat).collect::<Vec<_>>().join(", ");
                    if indent + head.chars().count() + flat_args.chars().count() + 2 <= width {
                        (format!("{head}({flat_args})"), false)
                    } else {
                        (
                            render_items(&format!("{head}("), ")", args, indent, width),
                            true,
                        )
                    }
                };

                let setter_indent = if args_were_multiline {
                    indent
                } else {
                    indent + INDENT
                };
                let pad = " ".repeat(setter_indent);
                let mut out = call_text;
                for s in setters {
                    let inner_args = if s.args.is_empty() {
                        String::new()
                    } else {
                        // The setter name and its opening paren sit at
                        // `setter_indent`, so a multi-line argument nests
                        // exactly as a hugged `Call` argument does: its own
                        // items land at `setter_indent + INDENT`, not one
                        // level deeper still.
                        s.args
                            .iter()
                            .map(|a| a.render_at(setter_indent, width))
                            .collect::<Vec<_>>()
                            .join(", ")
                    };
                    out.push_str(&format!("\n{pad}.{}({inner_args})", s.name));
                }
                out
            }
            Expr::Commented { .. } => unreachable!("handled above"),
        }
    }
}

fn render_items(open: &str, close: &str, items: &[Expr], indent: usize, width: usize) -> String {
    let pad = " ".repeat(indent + INDENT);
    let base = " ".repeat(indent);
    let body = items
        .iter()
        .map(|i| format!("{pad}{},\n", i.render_at(indent + INDENT, width)))
        .collect::<String>();
    format!("{open}\n{body}{base}{close}")
}

/// Builds a `Type::new(..)` call with its `with_x` chain, in declaration order.
pub struct Call {
    head: String,
    args: Vec<Expr>,
    setters: Vec<Setter>,
}

impl Call {
    pub fn new(head: &str) -> Self {
        Self {
            head: head.to_string(),
            args: Vec::new(),
            setters: Vec::new(),
        }
    }

    #[must_use]
    pub fn arg(mut self, expr: Expr) -> Self {
        self.args.push(expr);
        self
    }

    #[must_use]
    pub fn set(mut self, name: &str, expr: Expr) -> Self {
        self.setters.push(Setter {
            name: name.to_string(),
            args: vec![expr],
        });
        self
    }

    /// A no-argument chained method, such as `.bold()`.
    #[must_use]
    pub fn set0(mut self, name: &str) -> Self {
        self.setters.push(Setter {
            name: name.to_string(),
            args: Vec::new(),
        });
        self
    }

    pub fn into_expr(self) -> Expr {
        Expr::Call {
            head: self.head,
            args: self.args,
            setters: self.setters,
        }
    }
}

impl From<Call> for Expr {
    fn from(c: Call) -> Self {
        c.into_expr()
    }
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

    fn render(expr: &Expr) -> String {
        expr.render(&mut Writer::new())
    }

    #[test]
    fn short_call_with_setters_stays_on_one_line() {
        let expr = Call::new("SlackDividerBlock::new")
            .set("with_block_id", Expr::atom("\"d1\".into()"))
            .into_expr();
        assert_eq!(
            render(&expr),
            "SlackDividerBlock::new().with_block_id(\"d1\".into())"
        );
    }

    #[test]
    fn flat_arguments_put_setters_one_indent_deeper() {
        let expr = Call::new("SlackBlockButtonElement::new")
            .arg(Expr::atom("\"approve\".into()"))
            .arg(Expr::atom("pt!(\"Approve\")"))
            .set("with_value", Expr::atom("\"approve\".into()"))
            .set("with_style", Expr::atom("SlackBlockButtonStyle::Primary"))
            .into_expr();
        assert_eq!(
            render(&expr),
            "SlackBlockButtonElement::new(\"approve\".into(), pt!(\"Approve\"))\n    \
             .with_value(\"approve\".into())\n    \
             .with_style(SlackBlockButtonStyle::Primary)"
        );
    }

    #[test]
    fn a_single_list_argument_is_hugged_and_setters_return_to_base_indent() {
        let inner = Expr::List {
            kind: ListKind::SlackBlocks,
            items: vec![Expr::atom(&format!(
                "SlackDividerBlock::new() /* {} */",
                "x".repeat(90)
            ))],
        };
        let expr = Call::new("SlackActionsBlock::new")
            .arg(inner)
            .set("with_block_id", Expr::atom("\"deploy-actions\".into()"))
            .into_expr();
        let out = render(&expr);
        assert!(
            out.starts_with("SlackActionsBlock::new(slack_blocks![\n"),
            "{out}"
        );
        assert!(
            out.ends_with("])\n.with_block_id(\"deploy-actions\".into())"),
            "{out}"
        );
    }

    #[test]
    fn two_long_arguments_each_take_a_line() {
        let long = Expr::atom(&format!("\"{}\"", "a".repeat(80)));
        let expr = Call::new("SlackRichTextList::new")
            .arg(Expr::atom("SlackRichTextListStyle::Bullet"))
            .arg(long)
            .into_expr();
        assert_eq!(
            render(&expr),
            format!(
                "SlackRichTextList::new(\n    SlackRichTextListStyle::Bullet,\n    \"{}\",\n)",
                "a".repeat(80)
            )
        );
    }

    #[test]
    fn suffix_attaches_to_the_last_line() {
        let expr = Expr::suffixed(
            Call::new("Url::parse")
                .arg(Expr::atom("\"https://example.com/\""))
                .into_expr(),
            "?.into()",
        );
        assert_eq!(
            render(&expr),
            "Url::parse(\"https://example.com/\")?.into()"
        );
    }

    #[test]
    fn a_comment_precedes_its_item() {
        let expr = Expr::Commented {
            comment: "// blocks[2]: not converted: unknown variant `foo`".into(),
            inner: Box::new(Expr::atom("SlackDividerBlock::new()")),
        };
        assert_eq!(
            render(&expr),
            "// blocks[2]: not converted: unknown variant `foo`\nSlackDividerBlock::new()"
        );
    }
}
