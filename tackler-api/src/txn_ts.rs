/*
 * Copyright 2023 E257.FI
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */

//! Timestamp utilities
//!
//! `txn_ts` is collection of utilities to generate
//! different representations of Txn timestamps.
//!
use time::format_description::well_known::Rfc3339;
use time::macros::format_description;
use time::{OffsetDateTime, UtcOffset};
use time_tz::OffsetDateTimeExt;
use time_tz::Tz;

/// RFC-3339 timestamp as string
///
/// # Examples
/// ```
/// use time::macros::datetime;
/// use tackler_api::txn_ts;
///
/// let ts = datetime!(2022-12-24 14:15:16 +02:00);
/// assert_eq!(txn_ts::rfc_3339(ts), "2022-12-24T14:15:16+02:00");
///
/// let ns = datetime!(2022-06-24 14:15:16.123456789 +03:00);
/// assert_eq!(txn_ts::rfc_3339(ns), "2022-06-24T14:15:16.123456789+03:00");
/// ```
pub fn rfc_3339(ts: OffsetDateTime) -> String {
    ts.format(&Rfc3339)
        .unwrap_or_else(|_| "IE: ts frmt error, rfc_3339".to_string())
}

/// Human readable timestamp with seconds precision and with zone
///
/// This is ISO-8601 style timestamp with space separator between components.
///
/// # Examples
/// ```
/// use time::macros::datetime;
/// use tackler_api::txn_ts;
///
/// let ts = datetime!(2022-12-24 14:15:16 +02:00);
/// assert_eq!(txn_ts::seconds_tz(ts), "2022-12-24 14:15:16 +02:00");
///
/// let ns = datetime!(2022-06-24 14:15:16.123456789 +03:00);
/// assert_eq!(txn_ts::seconds_tz(ns), "2022-06-24 14:15:16 +03:00");
/// ```
pub fn seconds_tz(ts: OffsetDateTime) -> String {
    let fmt = format_description!(
        "[year base:calendar]-[month]-[day] [hour]:[minute]:[second] [offset_hour sign:mandatory]:[offset_minute]");
    ts.format(fmt)
        .unwrap_or_else(|_| "IE: ts frmt error, seconds_tz".to_string())
}

/// Human readable timestamp with full precision and with zone
///
/// This is ISO-8601 style timestamp with space separator between components.
///
/// # Examples
/// ```
/// use time::macros::datetime;
/// use tackler_api::txn_ts;
///
/// let ts = datetime!(2022-12-24 14:15:16 +02:00);
/// assert_eq!(txn_ts::full_tz(ts), "2022-12-24 14:15:16.0 +02:00");
///
/// let ns = datetime!(2022-06-24 14:15:16.123456789 +03:00);
/// assert_eq!(txn_ts::full_tz(ns), "2022-06-24 14:15:16.123456789 +03:00");
/// ```
pub fn full_tz(ts: OffsetDateTime) -> String {
    let fmt = format_description!(
        "[year base:calendar]-[month]-[day] [hour]:[minute]:[second][optional [.[subsecond digits:1+]]] [offset_hour sign:mandatory]:[offset_minute]");
    ts.format(fmt)
        .unwrap_or_else(|_| "IE: ts frmt error, full_tz".to_string())
}

fn fmt_seconds(ts: OffsetDateTime) -> String {
    let fmt = format_description!("[year base:calendar]-[month]-[day] [hour]:[minute]:[second]");
    ts.format(fmt)
        .unwrap_or_else(|_| "IE: ts frmt error, fmt_seconds".to_string())
}

fn fmt_full(ts: OffsetDateTime) -> String {
    let fmt = format_description!(
        "[year base:calendar]-[month]-[day] [hour]:[minute]:[second][optional [.[subsecond digits:1+]]]");
    ts.format(fmt)
        .unwrap_or_else(|_| "IE: ts frmt error, fmt_full".to_string())
}

fn fmt_date(ts: OffsetDateTime) -> String {
    let fmt = format_description!("[year base:calendar]-[month]-[day]");
    ts.format(fmt)
        .unwrap_or_else(|_| "IE: ts frmt error, fmt_date".to_string())
}

fn fmt_month(ts: OffsetDateTime) -> String {
    let fmt = format_description!("[year base:calendar]-[month]");
    ts.format(fmt)
        .unwrap_or_else(|_| "IE: ts frmt error, fmt_month".to_string())
}

fn fmt_year(ts: OffsetDateTime) -> String {
    let fmt = format_description!("[year base:calendar]");
    ts.format(fmt)
        .unwrap_or_else(|_| "IE: ts frmt error, fmt_year".to_string())
}

fn fmt_week(ts: OffsetDateTime) -> String {
    let fmt = format_description!("[year base:iso_week]-W[week_number repr:iso]");
    ts.format(fmt)
        .unwrap_or_else(|_| "IE: ts frmt error, fmt_week".to_string())
}

fn fmt_week_date(ts: OffsetDateTime) -> String {
    let fmt = format_description!(
        "[year base:iso_week]-W[week_number repr:iso]-[weekday repr:monday one_indexed:true]"
    );
    ts.format(fmt)
        .unwrap_or_else(|_| "IE: ts frmt error, fmt_week_date".to_string())
}

/// Human readable timestamp with seconds precision in UTC zone
///
/// This is ISO-8601 style timestamp with space separator between components.
/// Timestamp is converted into UTC zone.
///
/// # Examples
/// ```
/// use time::macros::datetime;
/// use tackler_api::txn_ts;
///
/// let ts = datetime!(2022-12-24 14:15:16 +02:00);
/// assert_eq!(txn_ts::utc_seconds(ts), "2022-12-24 12:15:16");
///
/// let ns = datetime!(2022-06-24 14:15:16.123456789 +03:00);
/// assert_eq!(txn_ts::utc_seconds(ns), "2022-06-24 11:15:16");
/// ```
pub fn utc_seconds(ts: OffsetDateTime) -> String {
    fmt_seconds(ts.to_offset(UtcOffset::UTC))
}

/// Human readable timestamp with full precision in UTC zone
///
/// This is ISO-8601 style timestamp with space separator between components.
/// Timestamp is converted into UTC zone.
///
/// # Examples
/// ```
/// use time::macros::datetime;
/// use tackler_api::txn_ts;
///
/// let ts = datetime!(2022-12-24 14:15:16 +02:00);
/// assert_eq!(txn_ts::utc_full(ts), "2022-12-24 12:15:16.0");
///
/// let ns = datetime!(2022-06-24 14:15:16.123456789 +03:00);
/// assert_eq!(txn_ts::utc_full(ns), "2022-06-24 11:15:16.123456789");
/// ```
pub fn utc_full(ts: OffsetDateTime) -> String {
    fmt_full(ts.to_offset(UtcOffset::UTC))
}

/// Human readable timestamp with date (day) precision in UTC zone
///
/// This is ISO-8601 style timestamp with space separator between components.
/// Timestamp is converted into UTC zone.
///
/// # Examples
/// ```
/// use time::macros::datetime;
/// use tackler_api::txn_ts;
///
/// let ts = datetime!(2022-12-31 19:00:00 -05:00);
/// assert_eq!(txn_ts::utc_date(ts), "2023-01-01");
/// ```
pub fn utc_date(ts: OffsetDateTime) -> String {
    fmt_date(ts.to_offset(UtcOffset::UTC))
}

/// Human readable timestamp with month precision in UTC zone
///
/// This is ISO-8601 style timestamp with space separator between components.
/// Timestamp is converted into UTC zone.
///
/// # Examples
/// ```
/// use time::macros::datetime;
/// use tackler_api::txn_ts;
///
/// let ts = datetime!(2022-12-31 19:00:00 -05:00);
/// assert_eq!(txn_ts::utc_month(ts), "2023-01");
/// ```
pub fn utc_month(ts: OffsetDateTime) -> String {
    fmt_month(ts.to_offset(UtcOffset::UTC))
}

/// Human readable timestamp with year precision in UTC zone
///
/// This is ISO-8601 style timestamp with space separator between components.
/// Timestamp is converted into UTC zone.
///
/// # Examples
/// ```
/// use time::macros::datetime;
/// use tackler_api::txn_ts;
///
/// let ts = datetime!(2022-12-31 19:00:00 -05:00);
/// assert_eq!(txn_ts::utc_year(ts), "2023");
/// ```
pub fn utc_year(ts: OffsetDateTime) -> String {
    fmt_year(ts.to_offset(UtcOffset::UTC))
}

/// Timestamp with ISO-8601 week precision in UTC timezone
///
/// Timestamp is converted into UTC timezone.
///
/// # Examples
/// ```
/// use time::macros::datetime;
/// use tackler_api::txn_ts;
///
/// let ts = datetime!(2010-01-03 00:00:00 +00:00);
/// assert_eq!(txn_ts::utc_iso_week(ts), "2009-W53");
/// let ts = datetime!(2010-01-04 00:00:00 +00:00);
/// assert_eq!(txn_ts::utc_iso_week(ts), "2010-W01");
///
/// let ny = datetime!(2010-01-03 19:00:00 -05:00);
/// assert_eq!(txn_ts::utc_iso_week(ny), "2010-W01");
///
/// ```
pub fn utc_iso_week(ts: OffsetDateTime) -> String {
    fmt_week(ts.to_offset(UtcOffset::UTC))
}

/// Timestamp with ISO-8601 week-date precision in UTC timezone
///
/// Timestamp is converted into UTC timezone.
///
/// # Examples
/// ```
/// use time::macros::datetime;
/// use tackler_api::txn_ts;
///
/// let ts = datetime!(2010-01-03 00:00:00 +00:00);
/// assert_eq!(txn_ts::utc_iso_week_date(ts), "2009-W53-7");
/// let ts = datetime!(2010-01-04 00:00:00 +00:00);
/// assert_eq!(txn_ts::utc_iso_week_date(ts), "2010-W01-1");
///
/// let ny = datetime!(2010-01-03 19:00:00 -05:00);
/// assert_eq!(txn_ts::utc_iso_week_date(ny), "2010-W01-1");
///
/// ```
pub fn utc_iso_week_date(ts: OffsetDateTime) -> String {
    fmt_week_date(ts.to_offset(UtcOffset::UTC))
}

/// Human readable timestamp with seconds precision in provided timezone
///
/// This is ISO-8601 style timestamp with space separator between components.
/// Timestamp is converted into provided timezone.
///
/// # Examples
/// ```
/// # use std::error::Error;
/// # use time::macros::datetime;
/// use time_tz::timezones;
/// use tackler_api::txn_ts;
///
/// let new_york_tz = timezones::get_by_name("America/New_York").ok_or("tz not found")?;
/// let helsinki_tz = timezones::get_by_name("Europe/Helsinki").ok_or("tz not found")?;
///
/// let ts = datetime!(2022-12-24 12:15:16 +00:00);
/// assert_eq!(txn_ts::zoned_seconds(ts, new_york_tz), "2022-12-24 07:15:16");
/// assert_eq!(txn_ts::zoned_seconds(ts, helsinki_tz), "2022-12-24 14:15:16");
///
/// let ns = datetime!(2022-06-24 12:15:16.123456789 +00:00);
/// assert_eq!(txn_ts::zoned_seconds(ns, helsinki_tz), "2022-06-24 15:15:16");
/// # Ok::<(), Box<dyn Error>>(())
/// ```
pub fn zoned_seconds(ts: OffsetDateTime, tz: &Tz) -> String {
    fmt_seconds(ts.to_timezone(tz))
}

/// Human readable timestamp with full precision in provided timezone
///
/// This is ISO-8601 style timestamp with space separator between components.
/// Timestamp is converted into provided timezone.
///
/// # Examples
/// ```
/// # use std::error::Error;
/// # use time::macros::datetime;
/// use time_tz::timezones;
/// use tackler_api::txn_ts;
///
/// let new_york_tz = timezones::get_by_name("America/New_York").ok_or("tz not found")?;
/// let helsinki_tz = timezones::get_by_name("Europe/Helsinki").ok_or("tz not found")?;
///
/// let ts = datetime!(2022-12-24 12:15:16 +00:00);
/// assert_eq!(txn_ts::zoned_full(ts, new_york_tz), "2022-12-24 07:15:16.0");
/// assert_eq!(txn_ts::zoned_full(ts, helsinki_tz), "2022-12-24 14:15:16.0");
///
/// let ns = datetime!(2022-06-24 12:15:16.123456789 +00:00);
/// assert_eq!(txn_ts::zoned_full(ns, helsinki_tz), "2022-06-24 15:15:16.123456789");
/// # Ok::<(), Box<dyn Error>>(())
/// ```
pub fn zoned_full(ts: OffsetDateTime, tz: &Tz) -> String {
    fmt_full(ts.to_timezone(tz))
}

/// Human readable timestamp with date precision in provided timezone
///
/// This is ISO-8601 style timestamp.
/// Timestamp is converted into provided timezone.
///
/// # Examples
/// ```
/// # use std::error::Error;
/// # use time::macros::datetime;
/// use time_tz::timezones;
/// use tackler_api::txn_ts;
///
/// let new_york_tz = timezones::get_by_name("America/New_York").ok_or("tz not found")?;
/// let helsinki_tz = timezones::get_by_name("Europe/Helsinki").ok_or("tz not found")?;
///
/// let ts = datetime!(2022-12-23 22:00:00 +00:00);
/// assert_eq!(txn_ts::zoned_date(ts, new_york_tz), "2022-12-23");
/// assert_eq!(txn_ts::zoned_date(ts, helsinki_tz), "2022-12-24");
///
/// let ns = datetime!(2022-06-23 21:00:00 +00:00);
/// assert_eq!(txn_ts::zoned_date(ns, helsinki_tz), "2022-06-24");
/// # Ok::<(), Box<dyn Error>>(())
/// ```
pub fn zoned_date(ts: OffsetDateTime, tz: &Tz) -> String {
    fmt_date(ts.to_timezone(tz))
}

/// Human readable timestamp with month precision in provided timezone
///
/// This is ISO-8601 style timestamp.
/// Timestamp is converted into provided timezone.
///
/// # Examples
/// ```
/// # use std::error::Error;
/// # use time::macros::datetime;
/// use time_tz::timezones;
/// use tackler_api::txn_ts;
///
/// let new_york_tz = timezones::get_by_name("America/New_York").ok_or("tz not found")?;
/// let helsinki_tz = timezones::get_by_name("Europe/Helsinki").ok_or("tz not found")?;
///
/// let ts = datetime!(2022-12-31 22:00:00 +00:00);
/// assert_eq!(txn_ts::zoned_month(ts, new_york_tz), "2022-12");
/// assert_eq!(txn_ts::zoned_month(ts, helsinki_tz), "2023-01");
///
/// let ns = datetime!(2022-06-30 21:00:00 +00:00);
/// assert_eq!(txn_ts::zoned_month(ns, helsinki_tz), "2022-07");
/// # Ok::<(), Box<dyn Error>>(())
/// ```
pub fn zoned_month(ts: OffsetDateTime, tz: &Tz) -> String {
    fmt_month(ts.to_timezone(tz))
}

/// Human readable timestamp with year precision in provided timezone
///
/// This is ISO-8601 style timestamp.
/// Timestamp is converted into provided timezone.
///
/// # Examples
/// ```
/// # use std::error::Error;
/// # use time::macros::datetime;
/// use time_tz::timezones;
/// use tackler_api::txn_ts;
///
/// let new_york_tz = timezones::get_by_name("America/New_York").ok_or("tz not found")?;
/// let helsinki_tz = timezones::get_by_name("Europe/Helsinki").ok_or("tz not found")?;
///
/// let ts = datetime!(2022-12-31 22:00:00 +00:00);
/// assert_eq!(txn_ts::zoned_year(ts, new_york_tz), "2022");
/// assert_eq!(txn_ts::zoned_year(ts, helsinki_tz), "2023");
///
/// # Ok::<(), Box<dyn Error>>(())
/// ```
pub fn zoned_year(ts: OffsetDateTime, tz: &Tz) -> String {
    fmt_year(ts.to_timezone(tz))
}

/// Timestamp with ISO-8601 week precision in provided timezone
///
/// Timestamp is converted into provided timezone.
///
/// # Examples
/// ```
/// # use std::error::Error;
/// # use time::macros::datetime;
/// use time_tz::timezones;
/// use tackler_api::txn_ts;
///
/// let new_york_tz = timezones::get_by_name("America/New_York").ok_or("tz not found")?;
/// let helsinki_tz = timezones::get_by_name("Europe/Helsinki").ok_or("tz not found")?;
///
/// let ts = datetime!(2010-01-04 00:00:00 +00:00);
/// assert_eq!(txn_ts::zoned_iso_week(ts, new_york_tz), "2009-W53");
/// assert_eq!(txn_ts::zoned_iso_week(ts, helsinki_tz), "2010-W01");
///
/// # Ok::<(), Box<dyn Error>>(())
/// ```
pub fn zoned_iso_week(ts: OffsetDateTime, tz: &Tz) -> String {
    fmt_week(ts.to_timezone(tz))
}

/// Timestamp with ISO-8601 week date precision in provided timezone
///
/// Timestamp is converted into provided timezone.
///
/// # Examples
/// ```
/// # use std::error::Error;
/// # use time::macros::datetime;
/// use time_tz::timezones;
/// use tackler_api::txn_ts;
///
/// let new_york_tz = timezones::get_by_name("America/New_York").ok_or("tz not found")?;
/// let helsinki_tz = timezones::get_by_name("Europe/Helsinki").ok_or("tz not found")?;
///
/// let ts = datetime!(2010-01-04 00:00:00 +00:00);
/// assert_eq!(txn_ts::zoned_iso_week_date(ts, new_york_tz), "2009-W53-7");
/// assert_eq!(txn_ts::zoned_iso_week_date(ts, helsinki_tz), "2010-W01-1");
///
/// # Ok::<(), Box<dyn Error>>(())
/// ```
pub fn zoned_iso_week_date(ts: OffsetDateTime, tz: &Tz) -> String {
    fmt_week_date(ts.to_timezone(tz))
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::datetime;
    use time_tz::timezones;

    fn txt2ts(txt_ts: &str) -> OffsetDateTime {
        OffsetDateTime::parse(txt_ts, &Rfc3339).unwrap(/*:test*/)
    }

    #[test]
    fn doc_test() {
        let ts = datetime!(2022-12-24 13:14:15 +02:00);
        assert_eq!(rfc_3339(ts), "2022-12-24T13:14:15+02:00");
    }

    #[test]
    fn test_rfc_3339() {
        assert_eq!(
            rfc_3339(txt2ts("2010-01-02T13:14:15+16:00")),
            "2010-01-02T13:14:15+16:00"
        );
        assert_eq!(
            rfc_3339(txt2ts("2010-12-24T01:02:03.456+16:00")),
            "2010-12-24T01:02:03.456+16:00"
        );
        assert_eq!(
            rfc_3339(txt2ts("2010-12-24T01:02:03.456789+16:00")),
            "2010-12-24T01:02:03.456789+16:00"
        );
        assert_eq!(
            rfc_3339(txt2ts("2010-12-24T01:02:03.700-16:00")),
            "2010-12-24T01:02:03.7-16:00"
        );

        assert_eq!(
            rfc_3339(txt2ts("2010-12-24T00:00:00+00:00")),
            "2010-12-24T00:00:00Z"
        );
        assert_eq!(
            rfc_3339(txt2ts("2020-12-31T23:58:59+00:00")),
            "2020-12-31T23:58:59Z"
        );
    }

    #[test]
    fn test_seconds_tz() {
        assert_eq!(
            seconds_tz(txt2ts("2010-01-02T00:00:00+00:00")),
            "2010-01-02 00:00:00 +00:00"
        ); // todo: time: Z
        assert_eq!(
            seconds_tz(txt2ts("2010-12-24T13:14:15+16:00")),
            "2010-12-24 13:14:15 +16:00"
        );
        assert_eq!(
            seconds_tz(txt2ts("2010-12-24T01:02:03.456+16:00")),
            "2010-12-24 01:02:03 +16:00"
        );
        assert_eq!(
            seconds_tz(txt2ts("2010-12-24T01:02:03.456789+16:00")),
            "2010-12-24 01:02:03 +16:00"
        );
        assert_eq!(
            seconds_tz(txt2ts("2020-12-24T23:58:59.123456789+00:00")),
            "2020-12-24 23:58:59 +00:00"
        ); // todo: time: Z
        assert_eq!(
            seconds_tz(txt2ts("2010-12-24T01:02:03.700-16:00")),
            "2010-12-24 01:02:03 -16:00"
        );

        assert_eq!(
            seconds_tz(txt2ts("2020-12-31T23:58:59+00:00")),
            "2020-12-31 23:58:59 +00:00"
        ); // todo: time: Z
    }

    #[test]
    fn test_full_tz() {
        assert_eq!(
            full_tz(txt2ts("2010-01-02T13:14:15+16:00")),
            "2010-01-02 13:14:15.0 +16:00"
        );
        assert_eq!(
            full_tz(txt2ts("2010-12-24T01:02:03.456+16:00")),
            "2010-12-24 01:02:03.456 +16:00"
        );
        assert_eq!(
            full_tz(txt2ts("2010-12-24T01:02:03.456789+16:00")),
            "2010-12-24 01:02:03.456789 +16:00"
        );
        assert_eq!(
            full_tz(txt2ts("2010-12-24T01:02:03.700-16:00")),
            "2010-12-24 01:02:03.7 -16:00"
        );

        assert_eq!(
            full_tz(txt2ts("2010-12-24T00:00:00+00:00")),
            "2010-12-24 00:00:00.0 +00:00"
        ); // todo: time: Z
        assert_eq!(
            full_tz(txt2ts("2020-12-31T23:58:59.123456789+00:00")),
            "2020-12-31 23:58:59.123456789 +00:00"
        ); // todo: time: Z
    }

    #[test]
    fn test_fmt_seconds() {
        assert_eq!(
            fmt_seconds(txt2ts("2010-01-02T00:00:00+00:00")),
            "2010-01-02 00:00:00"
        );
        assert_eq!(
            fmt_seconds(txt2ts("2010-12-24T13:14:15+16:00")),
            "2010-12-24 13:14:15"
        );
        assert_eq!(
            fmt_seconds(txt2ts("2010-12-24T01:02:03.456+16:00")),
            "2010-12-24 01:02:03"
        );
        assert_eq!(
            fmt_seconds(txt2ts("2010-12-24T01:02:03.456789+16:00")),
            "2010-12-24 01:02:03"
        );
        assert_eq!(
            fmt_seconds(txt2ts("2020-12-24T23:58:59.123456789+00:00")),
            "2020-12-24 23:58:59"
        );
        assert_eq!(
            fmt_seconds(txt2ts("2010-12-24T01:02:03.700-16:00")),
            "2010-12-24 01:02:03"
        );
    }

    #[test]
    fn test_fmt_full() {
        assert_eq!(
            fmt_full(txt2ts("2010-01-02T00:00:00+00:00")),
            "2010-01-02 00:00:00.0"
        );
        assert_eq!(
            fmt_full(txt2ts("2010-12-24T13:14:15+16:00")),
            "2010-12-24 13:14:15.0"
        );
        assert_eq!(
            fmt_full(txt2ts("2010-12-24T01:02:03.456+16:00")),
            "2010-12-24 01:02:03.456"
        );
        assert_eq!(
            fmt_full(txt2ts("2010-12-24T01:02:03.456789+16:00")),
            "2010-12-24 01:02:03.456789"
        );
        assert_eq!(
            fmt_full(txt2ts("2020-12-24T23:58:59.123456789+00:00")),
            "2020-12-24 23:58:59.123456789"
        );
        assert_eq!(
            fmt_full(txt2ts("2010-12-24T01:02:03.700-16:00")),
            "2010-12-24 01:02:03.7"
        );
    }

    #[test]
    fn test_fmt_date() {
        assert_eq!(fmt_date(txt2ts("2010-01-02T00:00:00+00:00")), "2010-01-02");
        assert_eq!(fmt_date(txt2ts("2010-12-24T13:14:15+16:00")), "2010-12-24");
        assert_eq!(
            fmt_date(txt2ts("2010-12-24T01:02:03.456+16:00")),
            "2010-12-24"
        );
        assert_eq!(
            fmt_date(txt2ts("2010-12-24T01:02:03.456789+16:00")),
            "2010-12-24"
        );
        assert_eq!(
            fmt_date(txt2ts("2020-12-24T23:58:59.123456789+00:00")),
            "2020-12-24"
        );
        assert_eq!(
            fmt_date(txt2ts("2010-12-24T01:02:03.700-16:00")),
            "2010-12-24"
        );
    }

    #[test]
    fn test_fmt_month() {
        assert_eq!(fmt_month(txt2ts("2010-01-02T00:00:00+00:00")), "2010-01");
        assert_eq!(fmt_month(txt2ts("2010-12-24T13:14:15+16:00")), "2010-12");
        assert_eq!(
            fmt_month(txt2ts("2010-12-24T01:02:03.456+16:00")),
            "2010-12"
        );
        assert_eq!(
            fmt_month(txt2ts("2010-12-24T01:02:03.456789+16:00")),
            "2010-12"
        );
        assert_eq!(
            fmt_month(txt2ts("2020-12-24T23:58:59.123456789+00:00")),
            "2020-12"
        );
        assert_eq!(
            fmt_month(txt2ts("2010-12-24T01:02:03.700-16:00")),
            "2010-12"
        );
    }

    #[test]
    fn test_fmt_year() {
        assert_eq!(fmt_year(txt2ts("2010-01-02T00:00:00+00:00")), "2010");
        assert_eq!(fmt_year(txt2ts("2010-12-24T13:14:15+16:00")), "2010");
        assert_eq!(fmt_year(txt2ts("2010-12-24T01:02:03.456+16:00")), "2010");
        assert_eq!(fmt_year(txt2ts("2010-12-24T01:02:03.456789+16:00")), "2010");
        assert_eq!(
            fmt_year(txt2ts("2020-12-24T23:58:59.123456789+00:00")),
            "2020"
        );
        assert_eq!(fmt_year(txt2ts("2010-12-24T01:02:03.700-16:00")), "2010");
    }

    #[test]
    fn test_fmt_week() {
        assert_eq!(fmt_week(txt2ts("2010-01-03T00:00:00+00:00")), "2009-W53");
        assert_eq!(fmt_week(txt2ts("2010-01-04T00:00:00+00:00")), "2010-W01");
        assert_eq!(fmt_week(txt2ts("2017-01-01T00:00:00+00:00")), "2016-W52");
        assert_eq!(fmt_week(txt2ts("2017-01-02T00:00:00+00:00")), "2017-W01");
    }

    #[test]
    fn test_fmt_week_date() {
        assert_eq!(
            fmt_week_date(txt2ts("2010-01-03T00:00:00+00:00")),
            "2009-W53-7"
        );
        assert_eq!(
            fmt_week_date(txt2ts("2010-01-04T00:00:00+00:00")),
            "2010-W01-1"
        );
        assert_eq!(
            fmt_week_date(txt2ts("2017-01-01T00:00:00+00:00")),
            "2016-W52-7"
        );

        assert_eq!(fmt_week_date(txt2ts("2017-01-02T00:00:00Z")), "2017-W01-1");

        assert_eq!(fmt_week_date(txt2ts("2020-12-31T00:00:00Z")), "2020-W53-4");
        assert_eq!(fmt_week_date(txt2ts("2021-01-01T00:00:00Z")), "2020-W53-5");
    }

    #[test]
    fn test_utc_seconds() {
        assert_eq!(
            utc_seconds(txt2ts("2010-01-01T00:00:00+16:00")),
            "2009-12-31 08:00:00"
        );
        assert_eq!(
            utc_seconds(txt2ts("2010-01-02T14:15:16+00:00")),
            "2010-01-02 14:15:16"
        );
        assert_eq!(
            utc_seconds(txt2ts("2010-01-01T01:02:03.700-16:00")),
            "2010-01-01 17:02:03"
        );
    }

    #[test]
    fn test_utc_full() {
        assert_eq!(
            utc_full(txt2ts("2010-01-01T00:00:00+16:00")),
            "2009-12-31 08:00:00.0"
        );
        assert_eq!(
            utc_full(txt2ts("2010-01-02T14:15:16.456+00:00")),
            "2010-01-02 14:15:16.456"
        );
        assert_eq!(
            utc_full(txt2ts("2010-01-01T01:02:03.700-16:00")),
            "2010-01-01 17:02:03.7"
        );
        assert_eq!(
            utc_full(txt2ts("2020-12-31T23:58:59.123456789Z")),
            "2020-12-31 23:58:59.123456789"
        );
    }

    #[test]
    fn test_utc_week() {
        assert_eq!(
            utc_iso_week(txt2ts("2010-01-03T00:00:00+00:00")),
            "2009-W53"
        );
        assert_eq!(
            utc_iso_week(txt2ts("2017-01-02T00:00:00+00:00")),
            "2017-W01"
        );
        assert_eq!(
            utc_iso_week(txt2ts("2017-01-02T00:00:00+02:00")),
            "2016-W52"
        );
        assert_eq!(
            utc_iso_week(txt2ts("2017-01-02T00:00:00-02:00")),
            "2017-W01"
        );
    }

    #[test]
    fn test_utc_week_date() {
        assert_eq!(
            utc_iso_week_date(txt2ts("2010-01-03T00:00:00+00:00")),
            "2009-W53-7"
        );

        assert_eq!(
            utc_iso_week_date(txt2ts("2017-01-02T00:00:00Z")),
            "2017-W01-1"
        );
        assert_eq!(
            utc_iso_week_date(txt2ts("2017-01-02T00:00:00+02:00")),
            "2016-W52-7"
        );
        assert_eq!(
            utc_iso_week_date(txt2ts("2017-01-01T22:00:00Z")),
            "2016-W52-7"
        );
        assert_eq!(
            utc_iso_week_date(txt2ts("2017-01-01T22:00:00-02:00")),
            "2017-W01-1"
        );
    }

    #[test]
    fn test_utc_date() {
        assert_eq!(utc_date(txt2ts("2010-01-01T15:00:00+16:00")), "2009-12-31");
        assert_eq!(utc_date(txt2ts("2010-01-01T08:02:03-16:00")), "2010-01-02");
        assert_eq!(
            utc_date(txt2ts("2020-12-31T23:59:59.999999999+00:00")),
            "2020-12-31"
        );
    }

    #[test]
    fn test_utc_month() {
        assert_eq!(utc_month(txt2ts("2010-01-01T15:00:00+16:00")), "2009-12");
        assert_eq!(utc_month(txt2ts("2010-01-31T08:02:03-16:00")), "2010-02");
        assert_eq!(
            utc_month(txt2ts("2020-12-31T23:59:59.999999999+00:00")),
            "2020-12"
        );
    }

    #[test]
    fn test_utc_year() {
        assert_eq!(utc_year(txt2ts("2010-01-01T15:00:00+16:00")), "2009");
        assert_eq!(utc_year(txt2ts("2010-12-31T08:02:03-16:00")), "2011");
        assert_eq!(
            utc_year(txt2ts("2020-12-31T23:59:59.999999999+00:00")),
            "2020"
        );
    }

    #[test]
    fn test_zoned_seconds() {
        let utc_tz = timezones::get_by_name("UTC").unwrap();
        let helsinki_tz = timezones::get_by_name("Europe/Helsinki").unwrap();

        // standard time
        assert_eq!(
            zoned_seconds(txt2ts("2010-01-02T00:00:00+00:00"), utc_tz),
            "2010-01-02 00:00:00"
        );
        assert_eq!(
            zoned_seconds(txt2ts("2010-01-02T00:00:00+00:00"), helsinki_tz),
            "2010-01-02 02:00:00"
        );

        // daylight saving time
        assert_eq!(
            zoned_seconds(txt2ts("2022-06-24T00:00:00+00:00"), utc_tz),
            "2022-06-24 00:00:00"
        );
        assert_eq!(
            zoned_seconds(txt2ts("2022-06-24T00:00:00+00:00"), helsinki_tz),
            "2022-06-24 03:00:00"
        );

        assert_eq!(
            zoned_seconds(txt2ts("2010-01-02T00:00:00+16:00"), utc_tz),
            "2010-01-01 08:00:00"
        );
        assert_eq!(
            zoned_seconds(txt2ts("2010-01-02T00:00:00+16:00"), helsinki_tz),
            "2010-01-01 10:00:00"
        );
    }

    #[test]
    fn test_zoned_full() {
        let utc_tz = timezones::get_by_name("UTC").unwrap();
        let helsinki_tz = timezones::get_by_name("Europe/Helsinki").unwrap();

        // standard time
        assert_eq!(
            zoned_full(txt2ts("2010-01-02T00:00:00+00:00"), utc_tz),
            "2010-01-02 00:00:00.0"
        );
        assert_eq!(
            zoned_full(txt2ts("2010-01-02T00:00:00+00:00"), helsinki_tz),
            "2010-01-02 02:00:00.0"
        );

        // daylight saving time
        assert_eq!(
            zoned_full(txt2ts("2022-06-24T00:00:00+00:00"), utc_tz),
            "2022-06-24 00:00:00.0"
        );
        assert_eq!(
            zoned_full(txt2ts("2022-06-24T00:00:00+00:00"), helsinki_tz),
            "2022-06-24 03:00:00.0"
        );

        assert_eq!(
            zoned_full(txt2ts("2010-01-02T00:00:00.123+16:00"), utc_tz),
            "2010-01-01 08:00:00.123"
        );
        assert_eq!(
            zoned_full(txt2ts("2010-01-02T00:00:00.123+16:00"), helsinki_tz),
            "2010-01-01 10:00:00.123"
        );
    }

    #[test]
    fn test_zoned_date() {
        let utc_tz = timezones::get_by_name("UTC").unwrap();
        let helsinki_tz = timezones::get_by_name("Europe/Helsinki").unwrap();

        // standard time
        assert_eq!(
            zoned_date(txt2ts("2010-01-02T00:00:00+00:00"), utc_tz),
            "2010-01-02"
        );
        assert_eq!(
            zoned_date(txt2ts("2009-12-31T21:59:59.999999999+00:00"), helsinki_tz),
            "2009-12-31"
        );
        assert_eq!(
            zoned_date(txt2ts("2009-12-31T22:00:00+00:00"), helsinki_tz),
            "2010-01-01"
        );

        // daylight saving time
        assert_eq!(
            zoned_date(txt2ts("2022-06-24T00:00:00+00:00"), utc_tz),
            "2022-06-24"
        );
        assert_eq!(
            zoned_date(txt2ts("2022-06-23T21:00:00+00:00"), helsinki_tz),
            "2022-06-24"
        );
    }

    #[test]
    fn test_zoned_month() {
        let utc_tz = timezones::get_by_name("UTC").unwrap();
        let helsinki_tz = timezones::get_by_name("Europe/Helsinki").unwrap();

        // standard time
        assert_eq!(
            zoned_month(txt2ts("2010-01-02T00:00:00+00:00"), utc_tz),
            "2010-01"
        );
        assert_eq!(
            zoned_month(txt2ts("2009-12-31T21:59:59.999999999+00:00"), helsinki_tz),
            "2009-12"
        );
        assert_eq!(
            zoned_month(txt2ts("2009-12-31T22:00:00+00:00"), helsinki_tz),
            "2010-01"
        );

        // daylight saving time
        assert_eq!(
            zoned_month(txt2ts("2022-06-30T00:00:00+00:00"), utc_tz),
            "2022-06"
        );
        assert_eq!(
            zoned_month(txt2ts("2022-06-30T21:00:00+00:00"), helsinki_tz),
            "2022-07"
        );
    }

    #[test]
    fn test_zoned_year() {
        let utc_tz = timezones::get_by_name("UTC").unwrap();
        let helsinki_tz = timezones::get_by_name("Europe/Helsinki").unwrap();

        // standard time
        assert_eq!(
            zoned_year(txt2ts("2010-01-02T00:00:00+00:00"), utc_tz),
            "2010"
        );
        assert_eq!(
            zoned_year(txt2ts("2009-12-31T21:59:59.999999999+00:00"), helsinki_tz),
            "2009"
        );
        assert_eq!(
            zoned_year(txt2ts("2009-12-31T22:00:00+00:00"), helsinki_tz),
            "2010"
        );
    }

    #[test]
    fn test_zoned_week() {
        let utc_tz = timezones::get_by_name("UTC").unwrap();
        let helsinki_tz = timezones::get_by_name("Europe/Helsinki").unwrap();

        // standard time
        assert_eq!(
            zoned_iso_week(txt2ts("2010-01-03T00:00:00+00:00"), utc_tz),
            "2009-W53"
        );
        assert_eq!(
            zoned_iso_week(txt2ts("2010-01-03T21:59:59.999999999+00:00"), helsinki_tz),
            "2009-W53"
        );
        assert_eq!(
            zoned_iso_week(txt2ts("2010-01-04T00:00:00+00:00"), utc_tz),
            "2010-W01"
        );
        assert_eq!(
            zoned_iso_week(txt2ts("2010-01-03T22:00:00+00:00"), helsinki_tz),
            "2010-W01"
        );

        // daylight saving time
        assert_eq!(
            zoned_iso_week(txt2ts("2022-06-19T23:59:59.999999999+00:00"), utc_tz),
            "2022-W24"
        );
        assert_eq!(
            zoned_iso_week(txt2ts("2022-06-19T20:59:59.999999999+00:00"), helsinki_tz),
            "2022-W24"
        );
        assert_eq!(
            zoned_iso_week(txt2ts("2022-06-20T00:00:00+00:00"), utc_tz),
            "2022-W25"
        );
        assert_eq!(
            zoned_iso_week(txt2ts("2022-06-19T21:00:00+00:00"), helsinki_tz),
            "2022-W25"
        );
    }

    #[test]
    fn test_zoned_week_date() {
        let utc_tz = timezones::get_by_name("UTC").unwrap();
        let helsinki_tz = timezones::get_by_name("Europe/Helsinki").unwrap();

        // standard time
        assert_eq!(
            zoned_iso_week_date(txt2ts("2010-01-03T00:00:00+00:00"), utc_tz),
            "2009-W53-7"
        );
        assert_eq!(
            zoned_iso_week_date(txt2ts("2010-01-03T21:59:59.999999999+00:00"), helsinki_tz),
            "2009-W53-7"
        );
        assert_eq!(
            zoned_iso_week_date(txt2ts("2010-01-04T00:00:00+00:00"), utc_tz),
            "2010-W01-1"
        );
        assert_eq!(
            zoned_iso_week_date(txt2ts("2010-01-03T22:00:00+00:00"), helsinki_tz),
            "2010-W01-1"
        );

        // daylight saving time
        assert_eq!(
            zoned_iso_week_date(txt2ts("2022-06-19T23:59:59.999999999+00:00"), utc_tz),
            "2022-W24-7"
        );
        assert_eq!(
            zoned_iso_week_date(txt2ts("2022-06-19T20:59:59.999999999+00:00"), helsinki_tz),
            "2022-W24-7"
        );
        assert_eq!(
            zoned_iso_week_date(txt2ts("2022-06-20T00:00:00+00:00"), utc_tz),
            "2022-W25-1"
        );
        assert_eq!(
            zoned_iso_week_date(txt2ts("2022-06-19T21:00:00+00:00"), helsinki_tz),
            "2022-W25-1"
        );
    }
}
