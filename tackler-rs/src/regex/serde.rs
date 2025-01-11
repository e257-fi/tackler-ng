/*
 * Tackler-NG 2024
 *
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

/// Full Haystack matcher serializer and deserializer
///
/// # Example
///
/// ```rust
/// use regex::Regex;
/// use serde::{Deserialize, Serialize};
/// use tackler_rs::regex::serde::full_haystack_matcher;
///
/// #[derive(Serialize, Deserialize)]
/// struct Account {
///     #[serde(with = "full_haystack_matcher")]
///     regex: Regex,
/// }
///
/// #
/// # fn main() {}
/// ```
pub mod full_haystack_matcher;
