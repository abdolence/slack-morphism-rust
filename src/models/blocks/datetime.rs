use crate::SlackTextFormat;
use chrono::prelude::*;

pub enum SlackDateTimeFormats {
    DateNum,
    Date,
    DateShort,
    DateLong,
    DatePretty,
    DateShortPretty,
    DateLongPretty,
    Time,
    TimeSecs,
}

#[allow(clippy::to_string_trait_impl)]
impl ToString for SlackDateTimeFormats {
    fn to_string(&self) -> String {
        match self {
            SlackDateTimeFormats::DateNum => "{date_num}".into(),
            SlackDateTimeFormats::Date => "{date}".into(),
            SlackDateTimeFormats::DateShort => "{date_short}".into(),
            SlackDateTimeFormats::DateLong => "{date_long}".into(),
            SlackDateTimeFormats::DatePretty => "{date_pretty}".into(),
            SlackDateTimeFormats::DateShortPretty => "{date_short_pretty}".into(),
            SlackDateTimeFormats::DateLongPretty => "{date_long_pretty}".into(),
            SlackDateTimeFormats::Time => "{time}".into(),
            SlackDateTimeFormats::TimeSecs => "{time_secs}".into(),
        }
    }
}

pub fn fmt_slack_date<TZ: TimeZone>(
    date: &DateTime<TZ>,
    token_string: &str,
    link: Option<&String>,
) -> String
where
    <TZ as chrono::offset::TimeZone>::Offset: std::fmt::Display,
{
    let link_part = link
        .map(|value| format!("^{value}"))
        .unwrap_or_else(|| "".into());
    let fallback = date.to_rfc2822();
    format!(
        "<!date^{timestamp}^{token_string}{link_part}|{fallback}>",
        timestamp = date.timestamp(),
        token_string = token_string,
        link_part = link_part,
        fallback = fallback
    )
}

impl<TZ: TimeZone> SlackTextFormat for DateTime<TZ>
where
    <TZ as chrono::offset::TimeZone>::Offset: std::fmt::Display,
{
    fn to_slack_format(&self) -> String {
        fmt_slack_date(
            self,
            SlackDateTimeFormats::DatePretty.to_string().as_str(),
            None,
        )
    }
}
