use crate::SlackTextFormat;

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

fn fmt_slack_date_parts(
    timestamp: i64,
    fallback: &str,
    token_string: &str,
    link: Option<&String>,
) -> String {
    let link_part = link
        .map(|value| format!("^{value}"))
        .unwrap_or_else(|| "".into());
    format!(
        "<!date^{timestamp}^{token_string}{link_part}|{fallback}>",
        timestamp = timestamp,
        token_string = token_string,
        link_part = link_part,
        fallback = fallback
    )
}

#[cfg(not(feature = "obsolete-chrono"))]
pub fn fmt_slack_date(
    date: &crate::SlackUtcDateTime,
    token_string: &str,
    link: Option<&String>,
) -> String {
    use crate::models::common::datetime::{to_rfc2822, unix_seconds};

    fmt_slack_date_parts(
        unix_seconds(date),
        to_rfc2822(date).as_str(),
        token_string,
        link,
    )
}

#[cfg(feature = "obsolete-chrono")]
pub fn fmt_slack_date<TZ: chrono::TimeZone>(
    date: &chrono::DateTime<TZ>,
    token_string: &str,
    link: Option<&String>,
) -> String
where
    <TZ as chrono::offset::TimeZone>::Offset: std::fmt::Display,
{
    fmt_slack_date_parts(
        date.timestamp(),
        date.to_rfc2822().as_str(),
        token_string,
        link,
    )
}

#[cfg(feature = "obsolete-chrono")]
impl<TZ: chrono::TimeZone> SlackTextFormat for chrono::DateTime<TZ>
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

#[cfg(not(feature = "obsolete-chrono"))]
impl SlackTextFormat for jiff::Timestamp {
    fn to_slack_format(&self) -> String {
        fmt_slack_date(
            self,
            SlackDateTimeFormats::DatePretty.to_string().as_str(),
            None,
        )
    }
}

#[cfg(not(feature = "obsolete-chrono"))]
impl SlackTextFormat for jiff::Zoned {
    fn to_slack_format(&self) -> String {
        let fallback = jiff::fmt::rfc2822::to_string(self).unwrap_or_else(|_| self.to_string());
        fmt_slack_date_parts(
            self.timestamp().as_second(),
            fallback.as_str(),
            SlackDateTimeFormats::DatePretty.to_string().as_str(),
            None,
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::prelude::SlackUtcDateTime;

    /// Both the jiff and the chrono backend render RFC 2822 identically here,
    /// so the expected fallback is the same in both feature modes.
    const EXPECTED_FALLBACK: &str = "Wed, 1 Jan 2020 00:42:42 +0000";

    #[test]
    fn test_fmt_slack_date() {
        let dt = "2020-01-01T00:42:42Z".parse::<SlackUtcDateTime>().unwrap();

        assert_eq!(
            fmt_slack_date(
                &dt,
                SlackDateTimeFormats::DatePretty.to_string().as_str(),
                None
            ),
            format!("<!date^1577839362^{{date_pretty}}|{EXPECTED_FALLBACK}>")
        );
    }

    #[test]
    fn test_fmt_slack_date_with_link() {
        let dt = "2020-01-01T00:42:42Z".parse::<SlackUtcDateTime>().unwrap();

        assert_eq!(
            fmt_slack_date(
                &dt,
                SlackDateTimeFormats::DateNum.to_string().as_str(),
                Some(&"https://example.net/".to_string())
            ),
            format!("<!date^1577839362^{{date_num}}^https://example.net/|{EXPECTED_FALLBACK}>")
        );
    }
}
