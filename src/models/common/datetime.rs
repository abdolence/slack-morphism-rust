//! Date and time types used across the Slack models.
//!
//! The concrete time crate is hidden behind [`SlackUtcDateTime`] and [`SlackCivilDate`].
//! By default they are backed by [`jiff`]. Enabling the deprecated `obsolete-chrono`
//! feature switches them back to the `chrono` types the crate used before.
//!
//! This module is the only place in the crate that refers to a time crate directly.

/// An instant in time in UTC, as used by the Slack API.
///
/// `jiff::Timestamp` by default, `chrono::DateTime<chrono::Utc>` with the
/// deprecated `obsolete-chrono` feature.
#[cfg(not(feature = "obsolete-chrono"))]
pub type SlackUtcDateTime = jiff::Timestamp;

/// An instant in time in UTC, as used by the Slack API.
///
/// `jiff::Timestamp` by default, `chrono::DateTime<chrono::Utc>` with the
/// deprecated `obsolete-chrono` feature.
#[cfg(feature = "obsolete-chrono")]
pub type SlackUtcDateTime = chrono::DateTime<chrono::Utc>;

/// A calendar date without a time zone, as used by the Slack API.
///
/// `jiff::civil::Date` by default, `chrono::NaiveDate` with the deprecated
/// `obsolete-chrono` feature.
#[cfg(not(feature = "obsolete-chrono"))]
pub type SlackCivilDate = jiff::civil::Date;

/// A calendar date without a time zone, as used by the Slack API.
///
/// `jiff::civil::Date` by default, `chrono::NaiveDate` with the deprecated
/// `obsolete-chrono` feature.
#[cfg(feature = "obsolete-chrono")]
pub type SlackCivilDate = chrono::NaiveDate;

/// Serde adapter for Slack's integer unix-seconds fields.
///
/// Used as `#[serde(with = "unix_seconds")]` on a [`SlackUtcDateTime`] field.
pub(crate) mod unix_seconds {
    #[cfg(not(feature = "obsolete-chrono"))]
    pub use jiff::fmt::serde::timestamp::second::required::{deserialize, serialize};

    #[cfg(feature = "obsolete-chrono")]
    pub use chrono::serde::ts_seconds::{deserialize, serialize};
}

/// The current instant in UTC.
pub(crate) fn now() -> SlackUtcDateTime {
    #[cfg(not(feature = "obsolete-chrono"))]
    {
        jiff::Timestamp::now()
    }
    #[cfg(feature = "obsolete-chrono")]
    {
        chrono::Utc::now()
    }
}

/// Builds an instant from unix seconds and an additional microseconds offset.
///
/// Returns `None` when the resulting instant is out of the supported range.
pub(crate) fn from_unix_seconds_micros(secs: i64, micros: u32) -> Option<SlackUtcDateTime> {
    let nanos = i32::try_from(micros).ok()?.checked_mul(1_000)?;

    #[cfg(not(feature = "obsolete-chrono"))]
    {
        jiff::Timestamp::new(secs, nanos).ok()
    }
    #[cfg(feature = "obsolete-chrono")]
    {
        use chrono::TimeZone;
        match chrono::Utc.timestamp_opt(secs, u32::try_from(nanos).ok()?) {
            chrono::LocalResult::None => None,
            chrono::LocalResult::Single(result) => Some(result),
            chrono::LocalResult::Ambiguous(first, _) => Some(first),
        }
    }
}

/// The number of whole seconds since the unix epoch.
// Only the jiff path of `fmt_slack_date` calls this: the chrono path keeps its generic
// `DateTime<TZ>` signature and uses chrono's own inherent methods. Still exercised by tests
// in both modes, so it is kept rather than gated out.
#[cfg_attr(feature = "obsolete-chrono", allow(dead_code))]
pub(crate) fn unix_seconds(date_time: &SlackUtcDateTime) -> i64 {
    #[cfg(not(feature = "obsolete-chrono"))]
    {
        date_time.as_second()
    }
    #[cfg(feature = "obsolete-chrono")]
    {
        date_time.timestamp()
    }
}

/// Formats an instant as an RFC 2822 date-time in UTC.
// See the note on `unix_seconds` for why this is allowed to be unused under `obsolete-chrono`.
#[cfg_attr(feature = "obsolete-chrono", allow(dead_code))]
pub(crate) fn to_rfc2822(date_time: &SlackUtcDateTime) -> String {
    #[cfg(not(feature = "obsolete-chrono"))]
    {
        jiff::fmt::rfc2822::to_string(&date_time.to_zoned(jiff::tz::TimeZone::UTC))
            .unwrap_or_else(|_| date_time.to_string())
    }
    #[cfg(feature = "obsolete-chrono")]
    {
        date_time.to_rfc2822()
    }
}

/// Parses a `YYYY-MM-DD` calendar date, rejecting anything else.
pub(crate) fn parse_civil_date(value: &str) -> Option<SlackCivilDate> {
    #[cfg(not(feature = "obsolete-chrono"))]
    {
        jiff::fmt::strtime::parse("%Y-%m-%d", value)
            .and_then(|parsed| parsed.to_date())
            .ok()
    }
    #[cfg(feature = "obsolete-chrono")]
    {
        SlackCivilDate::parse_from_str(value, "%Y-%m-%d").ok()
    }
}

/// The current number of whole seconds since the unix epoch, read from the system clock.
///
/// Deliberately built on `std` only: the places that need a coarse wall clock reading
/// (multipart boundaries, signature freshness) should not pull in a date/time crate.
#[cfg(feature = "signature-verifier")]
pub(crate) fn current_unix_seconds() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_secs() as i64)
        .unwrap_or_default()
}
