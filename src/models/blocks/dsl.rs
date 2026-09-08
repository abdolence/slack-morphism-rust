#![macro_use]

/// Builds a [`SlackBlockMarkDownText`](crate::blocks::SlackBlockMarkDownText),
/// converted into the caller's target type via [`Into`].
///
/// `md!(text)` wraps `text` as-is; `md!(fmt, args...)` formats first, the same
/// way [`format!`] does.
///
/// Because the expansion already ends in an untyped `.into()`, a bare
/// `md!(..)` item cannot be placed directly into `slack_blocks!` for a
/// text-only list — the compiler has nothing to infer the target type from
/// (`E0283`). Wrap it with `some(..)` (`slack_blocks![some(md!("hi"))]`) or
/// give the list an explicit element type, e.g.
/// `SlackContextBlock::new(vec![md!("hi")])`.
///
/// ```
/// use slack_morphism::prelude::*;
///
/// let text: SlackBlockMarkDownText = md!("hello");
/// let formatted: SlackBlockMarkDownText = md!("hello {}", "world");
/// ```
#[macro_export]
macro_rules! md {
    ($e : expr) => {
        $crate::blocks::SlackBlockMarkDownText::new($e.into()).into()
    };

    ($e : expr, $($es:expr),+ $(,)?) => {
        $crate::md!(::std::format!($e,$($es),+))
    };
}

/// Builds a [`SlackBlockPlainText`](crate::blocks::SlackBlockPlainText),
/// converted into the caller's target type via [`Into`].
///
/// Same rules and same `E0283` limitation as [`md!`]: `pt!(text)` wraps as-is,
/// `pt!(fmt, args...)` formats first, and a bare `pt!(..)` item needs an
/// explicit target type to appear directly in a `slack_blocks!` list.
///
/// ```
/// use slack_morphism::prelude::*;
///
/// let text: SlackBlockPlainText = pt!("hello");
/// let formatted: SlackBlockPlainText = pt!("hello {}", "world");
/// ```
#[macro_export]
macro_rules! pt {
    ($e : expr) => {
        $crate::blocks::SlackBlockPlainText::new($e.into()).into()
    };

    ($e : expr, $($es:expr),+ $(,)?) => {
        $crate::pt!(::std::format!($e,$($es),+))
    };
}

#[macro_export]
macro_rules! slack_block_item {
    (optionally ($pred:expr => $item:expr)) => {{
        if $pred {
            $crate::slack_block_item! (some $item)
        }
        else {
            None
        }
    }};

    (some $item:expr) => {{
        Some($item)
    }};

    (optionally_into ($pred:expr => $item:expr)) => {{
        if $pred {
            $crate::slack_block_item! (some_into $item)
        }
        else {
            None
        }
    }};

    (some_into $item:expr ) => {{
        Some($item.into())
    }};
}

/// Builds a `Vec<T>` of block-kit items from a comma-separated list, in one
/// of two syntaxes that may be freely mixed within a single list:
///
/// - **Bare form**: any expression is pushed via `.into()`. `..iter` splices
///   every element of `iter` in, each also converted via `.into()`.
///   `optionally(pred => item)` pushes `item` only when `pred` is true,
///   evaluating neither `pred` nor `item` otherwise.
/// - **Legacy keyword form**: `some(item)`, `some_into(item)`,
///   `optionally(pred => item)`, `optionally_into(pred => item)`, matching
///   [`slack_block_item!`].
///
/// `slack_blocks![]` and a trailing comma on the last item are both allowed.
///
/// See [`md!`] for a caveat on placing a bare `md!(..)`/`pt!(..)` item
/// directly in a list.
///
/// ```
/// use slack_morphism::prelude::*;
///
/// let items: Vec<SlackBlock> = slack_blocks![
///     some_into(SlackHeaderBlock::new(pt!("Title"))),
///     optionally_into(true => SlackDividerBlock::new()),
/// ];
/// assert_eq!(items.len(), 2);
/// ```
#[macro_export]
macro_rules! slack_blocks {
    (@acc $v:ident;) => {};

    (@acc $v:ident; some_into($item:expr) $(, $($rest:tt)*)?) => {
        $v.push(::core::convert::Into::into($item));
        $crate::slack_blocks!(@acc $v; $($($rest)*)?);
    };

    (@acc $v:ident; some($item:expr) $(, $($rest:tt)*)?) => {
        $v.push($item);
        $crate::slack_blocks!(@acc $v; $($($rest)*)?);
    };

    (@acc $v:ident; optionally($pred:expr => $item:expr) $(, $($rest:tt)*)?) => {
        if $pred { $v.push($item); }
        $crate::slack_blocks!(@acc $v; $($($rest)*)?);
    };

    (@acc $v:ident; optionally_into($pred:expr => $item:expr) $(, $($rest:tt)*)?) => {
        if $pred { $v.push(::core::convert::Into::into($item)); }
        $crate::slack_blocks!(@acc $v; $($($rest)*)?);
    };

    (@acc $v:ident; ..$iter:expr $(, $($rest:tt)*)?) => {
        for __slack_item in $iter { $v.push(::core::convert::Into::into(__slack_item)); }
        $crate::slack_blocks!(@acc $v; $($($rest)*)?);
    };

    (@acc $v:ident; $kw:ident($pred:expr => $item:expr) $(, $($rest:tt)*)?) => {
        ::core::compile_error!(::core::concat!("slack_blocks!: unknown keyword `", ::core::stringify!($kw),
            "`; expected `optionally(pred => item)` or `optionally_into(pred => item)`"));
    };

    (@acc $v:ident; $item:expr $(, $($rest:tt)*)?) => {
        $v.push(::core::convert::Into::into($item));
        $crate::slack_blocks!(@acc $v; $($($rest)*)?);
    };

    (@acc $v:ident; $($t:tt)+) => {
        ::core::compile_error!(::core::concat!("slack_blocks!: expected a comma-separated list of items, found `",
            ::core::stringify!($($t)+), "`"));
    };

    () => { ::std::vec::Vec::new() };

    ($($t:tt)*) => {{
        let mut __slack_blocks = ::std::vec::Vec::new();
        { $crate::slack_blocks!(@acc __slack_blocks; $($t)*); }
        __slack_blocks
    }};
}

#[cfg(test)]
mod tests {
    use crate::blocks::{
        SlackActionBlockElement, SlackActionsBlock, SlackBlock, SlackBlockButtonElement,
        SlackBlockMarkDownText, SlackContextBlock, SlackContextBlockElement, SlackDividerBlock,
        SlackHeaderBlock,
    };

    #[test]
    fn legacy_mixed_forms_build_vec_slack_block() {
        let blocks: Vec<SlackBlock> = crate::slack_blocks![
            some_into(SlackHeaderBlock::new(crate::pt!("Title"))),
            some_into(SlackDividerBlock::new()),
            optionally_into(true => SlackDividerBlock::new()),
            optionally_into(false => SlackDividerBlock::new()),
        ];
        assert_eq!(blocks.len(), 3);
    }

    #[test]
    fn bare_items_include_method_chain_and_nested_list() {
        let blocks: Vec<SlackBlock> = crate::slack_blocks![
            SlackHeaderBlock::new(crate::pt!("Title")),
            SlackContextBlock::new(crate::slack_blocks![some(crate::md!("x"))]),
        ];
        assert_eq!(blocks.len(), 2);
        let json = serde_json::to_value(&blocks).unwrap();
        assert_eq!(json[1]["elements"][0]["text"], "x");
    }

    #[test]
    fn empty_list_is_empty() {
        let blocks: Vec<SlackBlock> = crate::slack_blocks![];
        assert!(blocks.is_empty());
    }

    #[test]
    fn trailing_comma_allowed_for_bare_and_legacy_items() {
        let bare: Vec<SlackBlock> = crate::slack_blocks![SlackDividerBlock::new(),];
        assert_eq!(bare.len(), 1);

        let legacy: Vec<SlackBlock> = crate::slack_blocks![some_into(SlackDividerBlock::new()),];
        assert_eq!(legacy.len(), 1);
    }

    #[test]
    fn mixed_legacy_and_bare_items_in_one_list() {
        let blocks: Vec<SlackBlock> = crate::slack_blocks![
            some_into(SlackDividerBlock::new()),
            SlackDividerBlock::new(),
            optionally(true => SlackBlock::Divider(SlackDividerBlock::new())),
        ];
        assert_eq!(blocks.len(), 3);
    }

    #[test]
    fn spread_from_iterator_map_converts_each_item() {
        let ids = ["a", "b", "c"];
        let blocks: Vec<SlackBlock> = crate::slack_blocks![
            ..ids
                .iter()
                .map(|id| SlackDividerBlock::new().with_block_id(id.to_string().into())),
        ];
        assert_eq!(blocks.len(), 3);
    }

    #[test]
    fn spread_of_vec_slack_block() {
        let source: Vec<SlackBlock> = vec![
            SlackDividerBlock::new().into(),
            SlackDividerBlock::new().into(),
        ];
        let blocks: Vec<SlackBlock> = crate::slack_blocks![..source];
        assert_eq!(blocks.len(), 2);
    }

    #[test]
    fn spread_of_option_yields_zero_or_one_items() {
        let some_block: Option<SlackBlock> = Some(SlackDividerBlock::new().into());
        let none_block: Option<SlackBlock> = None;

        let blocks: Vec<SlackBlock> = crate::slack_blocks![..some_block, ..none_block];
        assert_eq!(blocks.len(), 1);
    }

    #[test]
    fn spread_only_list_is_typed_by_the_let_binding() {
        let source: Vec<SlackBlock> = vec![SlackDividerBlock::new().into()];
        let blocks: Vec<SlackBlock> = crate::slack_blocks![..source];
        assert_eq!(blocks.len(), 1);
    }

    #[test]
    fn optionally_predicate_is_lazy_and_short_circuits_item_evaluation() {
        let mut evaluations = 0;
        let items: Vec<SlackBlock> = crate::slack_blocks![optionally(false => {
            evaluations += 1;
            SlackBlock::Divider(SlackDividerBlock::new())
        })];
        assert!(items.is_empty());
        assert_eq!(evaluations, 0);
    }

    #[test]
    fn context_block_elements_from_some_and_bare_markdown() {
        let elements: Vec<SlackContextBlockElement> = crate::slack_blocks![
            some(crate::md!("a")),
            SlackBlockMarkDownText::new("b".into()),
        ];
        assert_eq!(elements.len(), 2);
    }

    #[test]
    fn action_block_elements_from_bare_button() {
        let elements: Vec<SlackActionBlockElement> =
            crate::slack_blocks![SlackBlockButtonElement::new(
                "btn-1".into(),
                crate::pt!("Click")
            ),];
        assert_eq!(elements.len(), 1);
        let action_block = SlackActionsBlock::new(elements);
        let json = serde_json::to_value(&action_block).unwrap();
        assert_eq!(json["elements"][0]["action_id"], "btn-1");
    }

    #[test]
    fn slack_block_item_still_works() {
        let some_item =
            crate::slack_block_item!(some SlackBlock::Divider(SlackDividerBlock::new()));
        assert!(some_item.is_some());

        let none_item = crate::slack_block_item!(optionally (false => SlackBlock::Divider(SlackDividerBlock::new())));
        assert!(none_item.is_none());
    }

    #[test]
    fn md_and_pt_format_args_arms_produce_expected_text() {
        let markdown: SlackBlockMarkDownText = crate::md!("hi {}", "there");
        assert_eq!(markdown.text, "hi there");

        let plain: crate::blocks::SlackBlockPlainText = crate::pt!("hi {}", "there");
        assert_eq!(plain.text, "hi there");
    }

    #[test]
    fn macros_are_crate_hygienic_without_prelude_glob_import() {
        let blocks: Vec<SlackBlock> = crate::slack_blocks![
            SlackDividerBlock::new(),
            some(SlackBlock::Divider(SlackDividerBlock::new())),
            ..[SlackBlock::Divider(SlackDividerBlock::new())],
        ];
        assert_eq!(blocks.len(), 3);

        let markdown: SlackBlockMarkDownText = crate::md!("x");
        assert_eq!(markdown.text, "x");

        let plain: crate::blocks::SlackBlockPlainText = crate::pt!("x");
        assert_eq!(plain.text, "x");
    }
}
