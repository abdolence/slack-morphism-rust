#![macro_use]

#[macro_export]
macro_rules! md {
    ($e : expr) => {
        SlackBlockMarkDownText::new($e.into()).into()
    };

    ($e : expr, $($es:expr),+) => {
        md!(format!($e,$($es),+))
    };
}

#[macro_export]
macro_rules! pt {
    ($e : expr) => {
        SlackBlockPlainText::new($e.into()).into()
    };

    ($e : expr, $($es:expr),+) => {
        pt!(format!($e,$($es),+))
    };
}

#[macro_export]
macro_rules! slack_optional_item {
    (optionally ($pred:expr => $item:expr)) => {{
        if $pred {
            Some($item)
        } else {
            None
        }
    }};

    (some $item:expr) => {{
        Some($item)
    }};
}

#[macro_export]
macro_rules! slack_optional_item_into {
    (optionally ($pred:expr => $item:expr)) => {{
        if $pred {
            slack_optional_item_into! (some $item)
        }
        else {
            None
        }
    }};

    (some $item:expr) => {{
        Some($item.into())
    }};
}

#[macro_export]
macro_rules! slack_blocks {

    () => { vec![] };

    ($($pred : tt($item:expr $(=> $item_r:expr)?)),+) => {{
        vec![
            $(slack_optional_item_into! ($pred($item $(=> $item_r)?))),*
        ].into_iter().flatten().collect()
    }};
}

#[macro_export]
macro_rules! slack_items {
    () => { vec![] };

    ($($pred : tt($item:expr $(=> $item_r:expr)?)),+) => {{
        let items = vec![
            $(slack_optional_item! ($pred($item $(=> $item_r)?))),*
        ];
        items.into_iter().flatten().collect()
    }};
}
