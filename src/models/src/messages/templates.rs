use rvstruct::ValueStruct;

use crate::{SlackMessageContent, SlackUserId};
use chrono::prelude::*;

pub trait SlackMessageTemplate {
    fn render_template(&self) -> SlackMessageContent;

    fn fmt_user_id(uid: &SlackUserId) -> String {
        format!("<@${}", uid.value())
    }

    fn fmt_date<TZ: TimeZone>(
        date: DateTime<TZ>,
        token_string: &str,
        link: Option<&String>,
    ) -> String
    where
        <TZ as chrono::offset::TimeZone>::Offset: std::fmt::Display,
    {
        let link_part = link
            .map(|value| format!("^${}", value))
            .unwrap_or("".into());
        let fallback = date.to_rfc2822();
        format!(
            "<!date^${timestamp}^${token_string}${link_part}|${fallback}>",
            timestamp = date.timestamp(),
            token_string = token_string,
            link_part = link_part,
            fallback = fallback
        )
    }
}
