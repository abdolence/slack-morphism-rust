use rvstruct::ValueStruct;

use crate::{SlackMessageContent, SlackUserId};
use chrono::prelude::*;

pub trait SlackMessageTemplate {
    fn render_template(&self) -> SlackMessageContent;

    fn fmt_user_id(uid: &SlackUserId) -> String {
        format!("<@${}", uid.value())
    }

    fn fmt_date<TZ: TimeZone>(date: DateTime<TZ>) -> String {
        format!(
            "<!date^${timestamp}^${token_string}${link_part}|${fallback}>",
            timestamp = date.timestamp(),
            token_string = "",
            link_part = "",
            fallback = ""
        )
    }
}
