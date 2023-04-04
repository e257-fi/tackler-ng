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

use crate::filters::IndentDisplay;
use crate::filters::TxnFilter;
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::from_utf8;

/// The main filter definition
///
/// This is the main handle for Txn Filter  definition, and this can be used to serialize
/// and deserialize filters from JSON.
///
/// # Examples
///
/// ```
/// # use std::error::Error;
/// # use tackler_api::filters::FilterDefinition;
/// # use tackler_api::filters::TxnFilter;
///
/// let filter_json_str = r#"{"txnFilter":{"NullaryTRUE":{}}}"#;
///
/// let tf = serde_json::from_str::<FilterDefinition>(filter_json_str)?;
///
/// match tf.txn_filter {
///      TxnFilter::NullaryTRUE(_) => assert!(true),
///      _ => assert!(false),
/// }
///
/// assert_eq!(serde_json::to_string(&tf)?, filter_json_str);
/// # Ok::<(), Box<dyn Error>>(())
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FilterDefinition {
    #[doc(hidden)]
    #[serde(rename = "txnFilter")]
    pub txn_filter: TxnFilter,
}

impl FilterDefinition {
    const FILTER_ARMOR: &'static str = "base64:";

    /// Generate filter from JSON String
    ///
    /// # Examples
    /// ```
    /// # use std::error::Error;
    /// # use tackler_api::filters::FilterDefinition;
    /// # use tackler_api::filters::TxnFilter;
    ///
    /// let filter_json_str = r#"{"txnFilter":{"NullaryTRUE":{}}}"#;
    ///
    /// let tf = FilterDefinition::from_json_str(filter_json_str)?;
    ///
    /// match tf.txn_filter {
    ///      TxnFilter::NullaryTRUE(_) => assert!(true),
    ///      _ => assert!(false),
    /// }
    ///
    /// # Ok::<(), Box<dyn Error>>(())
    /// ```
    pub fn from_json_str(filt_str: &str) -> Result<FilterDefinition, Box<dyn std::error::Error>> {
        Ok(serde_json::from_str::<FilterDefinition>(filt_str)?)
    }

    /// Test if filter string is ascii armored
    ///
    pub fn is_armored(filt: &str) -> bool {
        filt.starts_with(FilterDefinition::FILTER_ARMOR)
    }

    /// Generate filter from ascii armor JSON String
    ///
    /// # Examples
    /// ```
    /// # use std::error::Error;
    /// # use tackler_api::filters::FilterDefinition;
    /// # use tackler_api::filters::TxnFilter;
    ///
    /// let filter_ascii_armor = "base64:eyJ0eG5GaWx0ZXIiOnsiTnVsbGFyeVRSVUUiOnt9fX0K";
    ///
    /// let tf = FilterDefinition::from_armor(filter_ascii_armor)?;
    ///
    /// match tf.txn_filter {
    ///      TxnFilter::NullaryTRUE(_) => assert!(true),
    ///      _ => assert!(false),
    /// }
    ///
    /// # Ok::<(), Box<dyn Error>>(())
    /// ```
    pub fn from_armor(
        filt_armor_str: &str,
    ) -> Result<FilterDefinition, Box<dyn std::error::Error>> {
        let filt_armor = if FilterDefinition::is_armored(filt_armor_str) {
            filt_armor_str.trim_start_matches(FilterDefinition::FILTER_ARMOR)
        } else {
            let filt_begin = match filt_armor_str.char_indices().nth(10) {
                None => filt_armor_str,
                Some((idx, _)) => &filt_armor_str[..idx],
            };
            let msg = format!(
                "Unknown filter encoding, supported armor is: {}, (first 10 chars are): [{}]",
                FilterDefinition::FILTER_ARMOR,
                filt_begin
            );
            return Err(msg.into());
        };
        let data = &general_purpose::STANDARD.decode(filt_armor)?;

        FilterDefinition::from_json_str(from_utf8(data)?)
    }
}

impl Display for FilterDefinition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Filter:")?;
        self.txn_filter.i_fmt("  ", f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filters::NullaryTRUE;
    use indoc::indoc;
    use tackler_rs::IndocUtils;

    #[test]
    // test: c6fe4f86-1daa-4e29-b327-467aed6dc5bb
    // desc: filter definition, JSON
    fn filter_definition_json() {
        let filter_json_str = r#"{"txnFilter":{"NullaryTRUE":{}}}"#;

        let filter_text_str = indoc! {
        "|Filter:
         |  All pass
         |"}
        .strip_margin();

        let tf_res = serde_json::from_str::<FilterDefinition>(filter_json_str);
        assert!(tf_res.is_ok());
        let tf = tf_res.unwrap(/*:test:*/);

        match tf.txn_filter {
            TxnFilter::NullaryTRUE(_) => assert!(true),
            _ => assert!(false),
        }

        assert_eq!(format!("{tf}"), filter_text_str);
        assert_eq!(
            serde_json::to_string(&tf).unwrap(/*:test:*/),
            filter_json_str
        );
    }

    #[test]
    // test: 5e90f6cb-4414-4d4e-a496-1bb26abb9ba1
    // desc: filter definition, Text
    fn filter_definition_text() {
        let filter_text_str = indoc! {
        "|Filter:
         |  All pass
         |"}
        .strip_margin();

        let tfd = FilterDefinition {
            txn_filter: TxnFilter::NullaryTRUE(NullaryTRUE {}),
        };

        assert_eq!(format!("{tfd}"), filter_text_str);
    }

    #[test]
    fn filter_definition_is_encoded() {
        assert_eq!(
            FilterDefinition::is_armored(FilterDefinition::FILTER_ARMOR),
            true
        );
        assert_eq!(FilterDefinition::is_armored("hello there"), false);
    }

    #[test]
    // test: 939516a3-3c7a-4af8-b8fc-bcec2839965d
    // desc: decode txn filter from base64 armored JSON
    fn filter_definition_from_decoded() {
        let filters = vec![
            "base64:eyJ0eG5GaWx0ZXIiOnsiTnVsbGFyeVRSVUUiOnt9fX0K",
            "base64:IHsgInR4bkZpbHRlciI6eyJOdWxsYXJ5VFJVRSI6e30gfSB9Cg==",
        ];

        for s in filters {
            let tf_res = FilterDefinition::from_armor(s);
            assert!(tf_res.is_ok());

            let tf = tf_res.unwrap(/*:test:*/);
            match tf.txn_filter {
                TxnFilter::NullaryTRUE(_) => assert!(true),
                _ => assert!(false),
            }
        }
    }

    #[test]
    fn filter_definition_check_err_msg() {
        let s_err = "eyJ0eG5GaWx0ZXIiOnsiTnVsbGFyeVRSVUUiOnt9fX0K";

        let tf_res = FilterDefinition::from_armor(s_err);
        assert!(tf_res.is_err());

        let msg = tf_res.err().unwrap(/*:test:*/).to_string();

        assert!(msg.contains(FilterDefinition::FILTER_ARMOR));
        // test malformed cut-off
        assert!(msg.contains("eyJ0eG5GaW"));
        assert_eq!(msg.contains("eyJ0eG5GaWx"), false);
    }
}
