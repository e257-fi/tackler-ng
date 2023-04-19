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
use chrono::{DateTime, FixedOffset, SecondsFormat};

/// ISO-8601 Timestamp with offset.
///
/// Generates ISO-8601 date-time with offset `2016-12-17T12:31:12+03:00`
#[must_use]
pub fn iso_zoned_ts(ts: DateTime<FixedOffset>) -> String {
    ts.to_rfc3339_opts(SecondsFormat::AutoSi, true)
}

//   /**
//    * Zoned timestamp (seconds)
//    *
//    * @param ts timestamp
//    * @return date time (s) with offset: 2016-12-17 12:31:12 +03:00
//    */
//   def tzSeconds(ts: ZonedDateTime): String = {
//     ts.format(frmtZonedSeconds)
//   }

//   /**
//    * Zoned timestamp (nanoseconds)
//    *
//    * @param ts timestamp
//    * @return date time (ns) with offset: 2016-12-17 12:31:12.123456789 +03:00
//    */
//   def tzFull(ts: ZonedDateTime): String = {
//     ts.format(frmtZonedFullTs)
//   }

//   /**
//    * Zoned timestamp (date)
//    *
//    * @param ts timestamp
//    * @return  date with offset: 2016-12-17 +03:00
//    */
//   def tzDate(ts: ZonedDateTime): String = {
//     ts.format(DateTimeFormatter.ofPattern("yyyy-MM-dd XXX"))
//   }

//   /**
//    * Zoned timestamp (month)
//    *
//    * @param ts timestamp
//    * @return year-month with offset: 2016-12 +03:00
//    */
//   def tzMonth(ts: ZonedDateTime): String = {
//     ts.format(DateTimeFormatter.ofPattern("yyyy'-'MM XXX"))
//   }

//   /**
//    * Zoned timestamp (year)
//    *
//    * @param ts timestamp
//    * @return year with offset: 2016 +03:00
//    */
//   def tzYear(ts: ZonedDateTime): String = {
//     ts.format(DateTimeFormatter.ofPattern("yyyy XXX"))
//   }

//   /**
//    * Zoned timestamp (year-week with ISO-8601 rules)
//    *
//    * @param ts timestamp
//    * @return year-week with offset: 2009-W53 +03:00
//    */
//   def tzWeek(ts: ZonedDateTime): String = {
//     ts.format(frmtTzWeek)
//   }

//   /**
//    * Zoned timestamp (year-week-day with ISO-8601 rules)
//    *
//    * @param ts timestamp
//    * @return year-week-date and offset: 2009-W53-5 +03:00
//    */
//   def tzWeekDate(ts: ZonedDateTime): String = {
//     ts.format(frmtTzWeekDate)
//   }

//   /**
//    * Local timestamp (seconds)
//    *
//    * @param ts timestamp
//    * @param localTZ local time zone
//    * @return local ts (seconds): 2016-12-17 12:31:12
//    */
//   def localSeconds(ts: ZonedDateTime, localTZ: ZoneId): String = {
//     ts.withZoneSameInstant(localTZ).format(frmtLocalSeconds)
//   }

//   /**
//    * Local timestamp (nanoseconds)
//    *
//    * @param ts timestamp
//    * @param localTZ local time zone
//    * @return local ts (nanoseconds): 2016-12-17 12:31:12.123456789
//    */
//   def localFull(ts: ZonedDateTime, localTZ: ZoneId): String = {
//     ts.withZoneSameInstant(localTZ).format(frmtLocalFullTs)
//   }

//   /**
//    * Local timestamp (date)
//    *
//    * @param ts timestamp
//    * @param localTZ local time zone
//    * @return local ts (date): 2016-12-17
//    */
//   def localDate(ts: ZonedDateTime, localTZ: ZoneId): String = {
//     ts.withZoneSameInstant(localTZ).format(DateTimeFormatter.ofPattern("yyyy-MM-dd"))
//   }

//   /**
//    * Local timestamp (month)
//    *
//    * @param ts timestamp
//    * @param localTZ local time zone
//    * @return local ts (months): 2016-12
//    */
//   def localMonth(ts: ZonedDateTime, localTZ: ZoneId): String = {
//     ts.withZoneSameInstant(localTZ).format(DateTimeFormatter.ofPattern("yyyy'-'MM"))
//   }

//   /**
//    * Local timestamp (year)
//    *
//    * @param ts timestamp
//    * @param localTZ local time zone
//    * @return Local ts (year): 2016
//    */
//   def localYear(ts: ZonedDateTime, localTZ: ZoneId): String = {
//     ts.withZoneSameInstant(localTZ).format(DateTimeFormatter.ofPattern("yyyy"))
//   }

//   /**
//    *  Local timestamp (year-week with ISO-8601 rules)
//    *
//    * @param ts timestamp
//    * @param localTZ local time zone
//    * @return local year-week: 2009-W53
//    */
//   def localWeek(ts: ZonedDateTime, localTZ: ZoneId): String = {
//     ts.withZoneSameInstant(localTZ).format(frmtLocalWeek)
//   }

//   /**
//    * Local timestamp (week-date with ISO-8601 rules)
//    *
//    * @param ts timestamp
//    * @param localTZ local time zone
//    * @return local week date: 2009-W53-5
//    */
//   def localWeekDate(ts: ZonedDateTime, localTZ: ZoneId): String = {
//     ts.withZoneSameInstant(localTZ).format(frmtLocalWeekDate)
//   }
