# Block Kit support

Slack Block Kit messages and views are JSON documents built from a fixed set
of block and element types. This library models each of those types as a
Rust struct or enum, and provides a small set of macros — `slack_blocks!`,
`md!`, `pt!` — to build the `Vec` of blocks or elements each block-holding
type expects, without writing `.into()` on every item by hand.

Everything below is real, compiling code. Run it yourself from
`examples/blocks_showcase.rs`, or copy any snippet directly.

## Quick start

The smallest possible message is a single section block with some markdown
text, wrapped in a `SlackMessageContent` and posted with
`SlackApiChatPostMessageRequest`:

```rust,no_run,noplaypen
use slack_morphism::prelude::*;

# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
let client = SlackClient::new(SlackClientHyperConnector::new()?);
let token: SlackApiToken = SlackApiToken::new("xoxb-your-token".into());
let session = client.open_session(&token);

let blocks: Vec<SlackBlock> = slack_blocks![
    SlackSectionBlock::new().with_text(md!("A message *with some bold text*.")),
];

let content = SlackMessageContent::new().with_blocks(blocks);
let request = SlackApiChatPostMessageRequest::new("#general".into(), content);

session.chat_post_message(&request).await?;
# Ok(())
# }
```

The rest of this page builds up from here: how text objects work, how
`slack_blocks!` builds lists, and how each block and element family is
constructed.

## Text objects

Block Kit has two flavors of text: `mrkdwn` (Slack's own markdown) and
`plain_text`. The library models them as `SlackBlockMarkDownText` and
`SlackBlockPlainText`, and the `md!` and `pt!` macros build them:

```rust,noplaypen
use slack_morphism::prelude::*;

let markdown: SlackBlockMarkDownText = md!("A message *with some bold text*.");
let plain: SlackBlockPlainText = pt!("Plain text");

// Both macros format like `format!` when given more than one argument.
let user_id = SlackUserId("U1234".into());
let greeting: SlackBlockMarkDownText = md!("Hey {}, welcome!", user_id.to_slack_format());
```

`md!` and `pt!` convert into whatever type the call site expects via
`Into`, so the same macro call can produce a `SlackBlockText`, a
`SlackContextBlockElement`, or the raw `SlackBlockMarkDownText`/
`SlackBlockPlainText`, depending on where it is used.

Some fields require plain text specifically and reject markdown at the API
level: block headers, button labels, and input labels are typed as
`SlackBlockPlainTextOnly`, not `SlackBlockText`. Passing markdown there does
not fail to compile — a plain string still converts — but Slack rejects the
request at runtime with `invalid_blocks` if the string contains markdown
syntax Slack cannot render as plain text. Use `pt!`, or a bare string
literal, for these fields.

## Building block lists

`slack_blocks!` builds a `Vec<T>` from a comma-separated list, converting
each item with `.into()`. Any block or element type that has a `From` (or
`Into`) conversion to the target item type can appear as a bare item:

```rust,noplaypen
use slack_morphism::prelude::*;

let show_divider = true;

let blocks: Vec<SlackBlock> = slack_blocks![
    SlackHeaderBlock::new(pt!("Weekly report")),
    optionally(show_divider => SlackDividerBlock::new().into()),
    ..(1..=3).map(|n| SlackSectionBlock::new().with_text(md!("Item {}", n))),
];

assert_eq!(blocks.len(), 5);
```

- A bare expression is pushed via `.into()`.
- `optionally(pred => item)` pushes `item` as-is only when `pred` is true;
  neither `pred` nor `item` is evaluated when it is false. Unlike the bare
  form, `item` is not converted automatically — end it with `.into()`
  yourself when the list's element type differs from `item`'s own type, as
  above.
- `..iter` splices every element of an iterator or `Vec` in, each converted
  via `.into()` — useful for building a run of blocks from data instead of
  writing them out by hand, or for spreading in blocks assembled elsewhere,
  as in [Templates](#templates) below.

These forms mix freely in one list. An older `some(item)` / `some_into(item)`
/ `optionally_into(pred => item)` syntax still works and appears throughout
older code and examples; prefer the bare form above in new code.

One limit worth knowing up front: a bare `md!(..)` or `pt!(..)` call cannot
be a list item directly, because its expansion already ends in an untyped
`.into()` and the compiler has nothing left to infer the item type from
(`E0283`). This only matters for a list whose items are text objects
themselves, such as a context block's elements or a section's fields — give
the list an explicit element type instead, as in
`SlackContextBlock::new(vec![md!("hi")])` or
`.with_fields(vec![md!("hi")])`, both used below.

## Sections

A section block carries a text object, optional fields (short text/value
pairs laid out in a grid), and an optional accessory element:

```rust,noplaypen
use slack_morphism::prelude::*;

let section = SlackSectionBlock::new()
    .with_text(md!("*Deploy status*: all systems green"))
    .with_fields(vec![md!("*Region:*\nus-east-1"), md!("*Duration:*\n42s")])
    .with_accessory(
        SlackBlockButtonElement::new("view-details".into(), pt!("Details")).into(),
    );

let block: SlackBlock = section.into();
```

## Actions and inputs

Action blocks hold interactive elements — buttons, selects, pickers. Input
blocks pair one element with a required label, for use in modals.

```rust,noplaypen
use slack_morphism::prelude::*;

let approve = SlackBlockButtonElement::new("approve".into(), pt!("Approve"))
    .with_style(SlackBlockButtonStyle::Primary);
let deny = SlackBlockButtonElement::new("deny".into(), pt!("Deny"))
    .with_style(SlackBlockButtonStyle::Danger);

let actions: SlackBlock = SlackActionsBlock::new(slack_blocks![approve, deny]).into();

let region_select = SlackBlockStaticSelectElement::new("region".into())
    .with_placeholder(pt!("Choose a region"))
    .with_options(vec![
        SlackBlockChoiceItem::new("US East".into(), "us-east-1".into()),
        SlackBlockChoiceItem::new("EU West".into(), "eu-west-1".into()),
    ]);

let date_picker = SlackBlockDatePickerElement::new("deploy-date".into())
    .with_placeholder(pt!("Pick a date"));

let date_input: SlackBlock =
    SlackInputBlock::new("Deploy date".into(), date_picker.into()).into();

let _ = (actions, region_select, date_input);
```

## Context

Context blocks hold a small run of text and image elements, usually shown in
a muted style below other content. As noted above, its element list needs an
explicit element type, so bare `md!`/`pt!` calls work directly inside
`vec![...]` without going through `slack_blocks!`:

```rust,noplaypen
use slack_morphism::prelude::*;

let context: SlackBlock =
    SlackContextBlock::new(vec![md!("Posted by "), pt!("the release bot")]).into();
```

## Rich text

Rich text blocks are Slack's structured formatting model: sections made of
inline runs (text, links, users, emoji, ...), lists, quotes, and
preformatted code. A bare string converts into an unstyled text run
wherever an inline element is expected, and `SlackRichTextText` has
`.bold()`, `.italic()`, `.strike()`, and `.code()` helpers for styled runs:

```rust,noplaypen
use slack_morphism::prelude::*;

let section: SlackRichTextElement = SlackRichTextSection::new(vec![
    "Build ".into(),
    SlackRichTextText::new("passed".to_string()).bold().into(),
    " ".into(),
    SlackRichTextEmoji::new(SlackEmojiName("white_check_mark".into())).into(),
])
.into();

let list: SlackRichTextElement = SlackRichTextList::new(
    SlackRichTextListStyle::Bullet,
    vec!["Unit tests".into(), "Integration tests".into()],
)
.into();

let quote: SlackRichTextElement =
    SlackRichTextQuote::new(vec!["Ship it.".into()]).into();

let preformatted: SlackRichTextElement =
    SlackRichTextPreformatted::new(vec!["cargo test --doc".into()]).into();

let rich_text: SlackBlock =
    SlackRichTextBlock::new(vec![section, list, quote, preformatted]).into();
```

The same bare-string conversion applies inside a `SlackRichTextList`'s items
and a rich text table cell, below.

## Tables

Table blocks are a `Vec<Vec<SlackTableCell>>`. A bare string becomes a
raw-text cell; a `SlackTableRichTextCell` holds rich text elements the same
way a rich text block does:

```rust,noplaypen
use slack_morphism::prelude::*;

let table: SlackBlock = SlackTableBlock::new(vec![
    vec!["Service".into(), "Status".into()],
    vec![
        "api".into(),
        SlackTableRichTextCell::new(vec![SlackRichTextSection::new(vec![
            SlackRichTextText::new("healthy".to_string()).bold().into(),
        ])
        .into()])
        .into(),
    ],
])
.into();
```

## Templates

`SlackMessageTemplate` renders a full `SlackMessageContent`;
`SlackBlocksTemplate` renders just a `Vec<SlackBlock>` meant to be spread
into a larger message with `..`. A typical template takes its parameters as
a struct built with `rsb_derive::Builder`:

```rust,noplaypen
use slack_morphism::prelude::*;
use rsb_derive::Builder;

#[derive(Debug, Clone, Builder)]
struct DeploySummaryParams {
    service: String,
    ok: bool,
}

impl SlackBlocksTemplate for DeploySummaryParams {
    fn render_template(&self) -> Vec<SlackBlock> {
        slack_blocks![
            SlackDividerBlock::new(),
            SlackContextBlock::new(vec![md!("Deployed by the release bot")]),
        ]
    }
}

#[derive(Debug, Clone, Builder)]
struct WelcomeMessageParams {
    user_id: SlackUserId,
}

impl SlackMessageTemplate for WelcomeMessageParams {
    fn render_template(&self) -> SlackMessageContent {
        let footer = DeploySummaryParams::new("api".into(), true);

        SlackMessageContent::new()
            .with_text(format!("Hey {}", self.user_id.to_slack_format()))
            .with_blocks(slack_blocks![
                SlackSectionBlock::new()
                    .with_text(md!("Hey {}, welcome!", self.user_id.to_slack_format())),
                ..footer.render_template(),
            ])
    }
}

let message = WelcomeMessageParams::new("U1234".into());
let content = message.render_template();
assert_eq!(content.blocks.map(|b| b.len()), Some(3));
```

## Views and modals

A modal is a `SlackModalView`: a title, a list of blocks (typically inputs),
and optional submit/close button text. Opening one requires a `trigger_id`
from the interaction that triggered it — a button click or a shortcut
invocation, not something you can obtain outside of a live request. Publishing
a home tab view uses `views.publish` and a user id instead:

```rust,no_run,noplaypen
use slack_morphism::prelude::*;

# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
let client = SlackClient::new(SlackClientHyperConnector::new()?);
let token: SlackApiToken = SlackApiToken::new("xoxb-your-token".into());
let session = client.open_session(&token);

let name_input = SlackBlockPlainTextInputElement::new("name".into());

let modal = SlackModalView::new(
    "Deploy".into(),
    vec![SlackInputBlock::new("Service name".into(), name_input.into()).into()],
)
.with_submit(pt!("Deploy"));

let open_request = SlackApiViewsOpenRequest::new(
    SlackTriggerId("123.456.abcdef".into()),
    SlackView::Modal(modal),
);
session.views_open(&open_request).await?;

let home = SlackHomeView::new(vec![SlackSectionBlock::new()
    .with_text(md!("Welcome to your home tab."))
    .into()]);

let publish_request =
    SlackApiViewsPublishRequest::new(SlackUserId("U1234".into()), SlackView::Home(home));
session.views_publish(&publish_request).await?;
# Ok(())
# }
```

## Troubleshooting `invalid_blocks`

Slack validates blocks server-side and returns `invalid_blocks` with little
detail beyond that. The usual causes:

- **Markdown where plain text is required.** Header text, button labels,
  input labels, and similar `SlackBlockPlainTextOnly` fields do not render
  `mrkdwn` syntax; passing text with `*bold*` or a link in one of them either
  shows the literal asterisks or is rejected outright, depending on the
  field.
- **Text over 3000 characters.** Section and context text objects have a
  3000-character limit; button and option text has a much smaller one (75
  characters for a button label).
- **More than 50 blocks in one message**, or more than 100 in a modal or
  home tab.
- **A missing or duplicate `action_id`.** Every interactive element needs an
  `action_id` unique within its view or message; the button, select, and
  input constructors above all take it as their first argument for this
  reason.

When none of the above explains it, the fastest way to isolate a bad block is
to paste the JSON from
[Block Kit Builder](https://app.slack.com/block-kit-builder) straight into
`serde_json::from_str::<Vec<SlackBlock>>`, confirm it deserializes and
matches what you expected, then compare it field by field against the blocks
you built:

```rust,noplaypen
use slack_morphism::prelude::*;

let payload = r#"[
    {
        "type": "section",
        "text": { "type": "mrkdwn", "text": "A message *with some bold text*." }
    }
]"#;

let blocks: Vec<SlackBlock> = serde_json::from_str(payload)?;
assert_eq!(blocks.len(), 1);
# Ok::<(), serde_json::Error>(())
```

## More examples

`examples/blocks_showcase.rs` posts one message per block family covered on
this page and prints a modal view. `examples/client.rs` shows a complete,
runnable client setup including authentication. `examples/socket_mode.rs`
shows handling interactions (button clicks, view submissions) that come back
from the blocks you post.
