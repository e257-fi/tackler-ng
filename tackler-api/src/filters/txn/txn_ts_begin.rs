/*
 * Copyright 2023-2024 E257.FI
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

use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

use crate::filters::IndentDisplay;

/// Txn TS Begin filter
///
/// Select transaction if its timestamp is on or after
/// specified `begin` time.
///
/// `begin` is inclusive timestamp in ISO 8601 format with zone
///
/// Time is expressed as ISO 8601 format with zone,
/// e.g. 2018-01-01T10:11:22.345+02:00
/// or 2018-01-01T08:11:22.345Z.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TxnFilterTxnTSBegin {
    #[doc(hidden)]
    #[serde(with = "time::serde::rfc3339")]
    pub begin: OffsetDateTime,
}

impl IndentDisplay for TxnFilterTxnTSBegin {
    fn i_fmt(&self, indent: &str, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{indent}Txn TS: begin {}",
            self.begin
                .format(&Rfc3339)
                .unwrap_or_else(|_| { "IE: ts frmt error".to_string() })
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filters::{logic::TxnFilterAND, FilterDefinition, NullaryTRUE, TxnFilter};
    use indoc::indoc;
    use tackler_rs::IndocUtils;

    #[test]
    // test: baa0038e-45b7-4911-a647-859de2da4716
    // desc: TxnTSBegin, JSON
    fn txn_ts_begin_json() {
        let filter_json_str =
            r#"{"txnFilter":{"TxnFilterTxnTSBegin":{"begin":"2023-02-25T10:11:22.345+02:00"}}}"#;

        let filter_text_str = indoc! {
        "|Filter
         |  Txn TS: begin 2023-02-25T10:11:22.345+02:00
         |"}
        .strip_margin();

        let tf_res = serde_json::from_str::<FilterDefinition>(filter_json_str);
        assert!(tf_res.is_ok());
        let tf = tf_res.unwrap(/*:test:*/);

        match tf.txn_filter {
            TxnFilter::TxnFilterTxnTSBegin(_) => (),
            _ => panic!(/*:test:*/),
        }

        assert_eq!(format!("{tf}"), filter_text_str);
        assert_eq!(
            serde_json::to_string(&tf).unwrap(/*:test:*/),
            filter_json_str
        );
    }

    #[test]
    // test: 7f2a2dd7-6a5c-4f59-8000-1ee451e1540f
    // desc: TxnTSBegin, JSON with Zulu zone
    fn txn_ts_begin_json_with_zulu() {
        let filter_json_str =
            r#"{"txnFilter":{"TxnFilterTxnTSBegin":{"begin":"2023-02-25T10:11:22.345Z"}}}"#;

        let filter_text_str = indoc! {
        "|Filter
         |  Txn TS: begin 2023-02-25T10:11:22.345Z
         |"}
        .strip_margin();

        let tf_res = serde_json::from_str::<FilterDefinition>(filter_json_str);
        assert!(tf_res.is_ok());
        let tf = tf_res.unwrap(/*:test:*/);

        match tf.txn_filter {
            TxnFilter::TxnFilterTxnTSBegin(_) => (),
            _ => panic!(/*:test:*/),
        }

        assert_eq!(format!("{tf}"), filter_text_str);
    }

    #[test]
    // test: 9002e6e1-cee5-4751-a3e0-c64cea0091e6
    // desc: TxnTSBegin, JSON with nanoseconds
    fn txn_ts_begin_json_with_nano() {
        let filter_json_str = r#"{"txnFilter":{"TxnFilterTxnTSBegin":{"begin":"2023-02-25T10:11:22.123456789-05:00"}}}"#;

        let filter_text_str = indoc! {
        "|Filter
         |  Txn TS: begin 2023-02-25T10:11:22.123456789-05:00
         |"}
        .strip_margin();

        let tf_res = serde_json::from_str::<FilterDefinition>(filter_json_str);
        assert!(tf_res.is_ok());
        let tf = tf_res.unwrap(/*:test:*/);

        match tf.txn_filter {
            TxnFilter::TxnFilterTxnTSBegin(_) => (),
            _ => panic!(/*:test:*/),
        }

        assert_eq!(format!("{tf}"), filter_text_str);
    }

    #[test]
    // test: c01de4f4-0e07-4d8d-a4c8-2d1ad28df264
    // desc: TxnTSBegin, Text
    fn txn_ts_begin_text() {
        let filter_text_str = indoc! {
        "|Filter
         |  AND
         |    Txn TS: begin 2023-02-25T10:11:22.345+02:00
         |    AND
         |      Txn TS: begin 2023-02-25T20:11:22.345+02:00
         |      All pass
         |"}
        .strip_margin();

        let tfd = FilterDefinition {
            txn_filter: TxnFilter::TxnFilterAND(TxnFilterAND {
                txn_filters: vec![
                    TxnFilter::TxnFilterTxnTSBegin(TxnFilterTxnTSBegin {
                        begin: OffsetDateTime::parse("2023-02-25T10:11:22.345+02:00", &Rfc3339)
                            .unwrap(/*:test:*/),
                    }),
                    TxnFilter::TxnFilterAND(TxnFilterAND {
                        txn_filters: vec![
                            TxnFilter::TxnFilterTxnTSBegin(TxnFilterTxnTSBegin {
                                begin: OffsetDateTime::parse("2023-02-25T20:11:22.345+02:00", &Rfc3339)
                                    .unwrap(/*:test:*/),
                            }),
                            TxnFilter::NullaryTRUE(NullaryTRUE {}),
                        ],
                    }),
                ],
            }),
        };

        assert_eq!(format!("{tfd}"), filter_text_str);
    }
}
