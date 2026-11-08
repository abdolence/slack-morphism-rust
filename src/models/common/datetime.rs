//! Date and time types used across the Slack models.
//!
//! The concrete time crate is hidden behind [`SlackUtcDateTime`] and [`SlackCivilDate`],
//! which are backed by [`jiff`].
//!
//! This module is the only place in the crate that refers to a time crate directly.

/// An instant in time in UTC, as used by the Slack API.
///
/// Backed by `jiff::Timestamp`.
pub type SlackUtcDateTime = jiff::Timestamp;

/// A calendar date without a time zone, as used by the Slack API.
///
/// Backed by `jiff::civil::Date`.
pub type SlackCivilDate = jiff::civil::Date;

/// Serde adapter for Slack's integer unix-seconds fields.
///
/// Used as `#[serde(with = "unix_seconds")]` on a [`SlackUtcDateTime`] field.
pub(crate) mod unix_seconds {
    pub use jiff::fmt::serde::timestamp::second::required::{deserialize, serialize};
}

/// The current instant in UTC.
pub(crate) fn now() -> SlackUtcDateTime {
    jiff::Timestamp::now()
}

/// Builds an instant from unix seconds and an additional microseconds offset.
///
/// Returns `None` when the resulting instant is out of the supported range.
pub(crate) fn from_unix_seconds_micros(secs: i64, micros: u32) -> Option<SlackUtcDateTime> {
    let nanos = i32::try_from(micros).ok()?.checked_mul(1_000)?;

    jiff::Timestamp::new(secs, nanos).ok()
}

/// The number of whole seconds since the unix epoch.
pub(crate) fn unix_seconds(date_time: &SlackUtcDateTime) -> i64 {
    date_time.as_second()
}

/// Formats an instant as an RFC 2822 date-time in UTC.
pub(crate) fn to_rfc2822(date_time: &SlackUtcDateTime) -> String {
    jiff::fmt::rfc2822::to_string(&date_time.to_zoned(jiff::tz::TimeZone::UTC))
        .unwrap_or_else(|_| date_time.to_string())
}

/// Parses a `YYYY-MM-DD` calendar date, rejecting anything else.
pub(crate) fn parse_civil_date(value: &str) -> Option<SlackCivilDate> {
    jiff::fmt::strtime::parse("%Y-%m-%d", value)
        .and_then(|parsed| parsed.to_date())
        .ok()
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
