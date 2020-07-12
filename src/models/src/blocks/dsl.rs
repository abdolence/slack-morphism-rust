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
macro_rules! slack_block_item {
    (optionally ($pred:expr => $item:expr)) => {{
        if $pred {
            slack_block_item! (some $item)
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
            slack_block_item! (some_into $item)
        }
        else {
            None
        }
    }};

    (some_into $item:expr ) => {{
        Some($item.into())
    }};
}

#[macro_export]
macro_rules! slack_blocks {

    () => { vec![] };

    ($($pred : tt($item:expr $(=> $item_r:expr)?)),+) => {{
        vec![
            $(slack_block_item! ($pred($item $(=> $item_r)?))),*
        ].into_iter().flatten().collect()
    }};
}
