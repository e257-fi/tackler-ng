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

use crate::filters;
use crate::filters::IndentDisplay;
use filters::TxnFilter;

use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

/// Data model for logical AND-filter
///
/// Actual filtering implementation is done by Trait [`FilterTxn`]
///
/// [`FilterTxn`]: ../tackler_core/filter/index.html
#[derive(Serialize, Deserialize, Debug)]
pub struct TxnFilterAND {
    // todo: functionality, test
    // todo-test: aa8aa459-b100-403e-98ea-7381ca58727d
    // desc: "reject AND filter with only one filter"
    #[serde(rename = "txnFilters")]
    pub txn_filters: Vec<TxnFilter>,
}

impl IndentDisplay for TxnFilterAND {
    fn i_fmt(&self, indent: &str, f: &mut Formatter<'_>) -> std::fmt::Result {
        filters::logic_filter_indent_fmt("AND", indent, &self.txn_filters, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filters::{FilterDefinition, NullaryFALSE, NullaryTRUE};
    use indoc::indoc;

    #[test]
    // test: caa264f6-719f-49e9-9b56-3bdf0b0941ec
    // desc: AND, JSON
    fn and_json() {
        let filter_json_str = r#"{"txnFilter":{"TxnFilterAND":{"txnFilters":[{"NullaryTRUE":{}},{"NullaryFALSE":{}}]}}}"#;

        let filter_text_str = indoc! {
            "Filter:
               AND
                 All pass
                 None pass
            "};

        let tf_res = serde_json::from_str::<FilterDefinition>(filter_json_str);
        assert!(tf_res.is_ok());
        let tf = tf_res.unwrap();

        match tf.txn_filter {
            TxnFilter::TxnFilterAND(_) => assert!(true),
            _ => assert!(false),
        }

        assert_eq!(format!("{tf}"), filter_text_str);
        assert_eq!(serde_json::to_string(&tf).unwrap(), filter_json_str);
    }

    #[test]
    // test: deda9918-cba5-4b3d-85db-61a3a7e1128f
    // desc: AND, Text
    fn and_filt_text() {
        let filter_text_str = indoc! {
            "Filter:
               AND
                 All pass
                 AND
                   All pass
                   None pass
            "};

        let tfd = FilterDefinition {
            txn_filter: TxnFilter::TxnFilterAND(TxnFilterAND {
                txn_filters: vec![
                    TxnFilter::NullaryTRUE(NullaryTRUE {}),
                    TxnFilter::TxnFilterAND(TxnFilterAND {
                        txn_filters: vec![
                            TxnFilter::NullaryTRUE(NullaryTRUE {}),
                            TxnFilter::NullaryFALSE(NullaryFALSE {}),
                        ],
                    }),
                ],
            }),
        };

        assert_eq!(format!("{tfd}"), filter_text_str);
    }
}
