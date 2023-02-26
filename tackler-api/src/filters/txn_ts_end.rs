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

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

use crate::filters::IndentDisplay;

#[derive(Serialize, Deserialize, Debug)]
pub struct TxnFilterTxnTSEnd {
    pub end: DateTime<FixedOffset>,
}

impl IndentDisplay for TxnFilterTxnTSEnd {
    fn i_fmt(&self, indent: &str, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{indent}Txn TS: end   {}", self.end.to_rfc3339())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filters::{FilterDefinition, NullaryTRUE, TxnFilter, TxnFilterAND};
    use crate::tests::IndocWithMarker;
    use indoc::indoc;

    #[test]
    // test: db171b86-7435-4e9b-bfa0-4288c720289c
    // desc: TxnTSEnd, JSON
    fn txn_ts_end_json() {
        let filter_json_str =
            r#"{"txnFilter":{"TxnFilterTxnTSEnd":{"end":"2023-02-25T10:11:22.345+02:00"}}}"#;

        let filter_text_str = indoc! {
        "|Filter:
         |  Txn TS: end   2023-02-25T10:11:22.345+02:00
         |"}
        .strip_margin();

        let tf_res = serde_json::from_str::<FilterDefinition>(filter_json_str);
        assert!(tf_res.is_ok());
        let tf = tf_res.unwrap();

        match tf.txn_filter {
            TxnFilter::TxnFilterTxnTSEnd(_) => assert!(true),
            _ => assert!(false),
        }

        assert_eq!(format!("{tf}"), filter_text_str);
        assert_eq!(serde_json::to_string(&tf).unwrap(), filter_json_str);
    }

    #[test]
    // test: ef2348e6-3684-4a13-85e9-5aec89a9e3bb
    // desc: TxnTSEnd, Text
    fn txn_ts_end_text() {
        let filter_text_str = indoc! {
        "|Filter:
         |  AND
         |    Txn TS: end   2023-02-25T10:11:22.345+02:00
         |    AND
         |      Txn TS: end   2023-02-25T20:11:22.345+02:00
         |      All pass
         |"}
        .strip_margin();

        let tfd = FilterDefinition {
            txn_filter: TxnFilter::TxnFilterAND(TxnFilterAND {
                txn_filters: vec![
                    TxnFilter::TxnFilterTxnTSEnd(TxnFilterTxnTSEnd {
                        end: "2023-02-25T10:11:22.345+02:00"
                            .parse::<DateTime<FixedOffset>>()
                            .unwrap(),
                    }),
                    TxnFilter::TxnFilterAND(TxnFilterAND {
                        txn_filters: vec![
                            TxnFilter::TxnFilterTxnTSEnd(TxnFilterTxnTSEnd {
                                end: "2023-02-25T20:11:22.345+02:00"
                                    .parse::<DateTime<FixedOffset>>()
                                    .unwrap(),
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
