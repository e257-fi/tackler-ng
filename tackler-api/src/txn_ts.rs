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
