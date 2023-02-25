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

/// Data model for logical NOT-filter
///
/// Actual filtering implementation is done by Trait [`FilterTxn`]
///
/// [`FilterTxn`]: ../tackler_core/filter/index.html
#[derive(Serialize, Deserialize, Debug)]
pub struct TxnFilterNOT {
    #[serde(rename = "txnFilter")]
    pub txn_filter: Box<TxnFilter>,
}

impl IndentDisplay for TxnFilterNOT {
    fn i_fmt(&self, indent: &str, f: &mut Formatter<'_>) -> std::fmt::Result {
        let new_ident = format!("{indent}  ");

        writeln!(f, "{indent}NOT")?;
        self.txn_filter.i_fmt(&new_ident, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filters::{FilterDefinition, NullaryTRUE};
    use indoc::indoc;

    #[test]
    // test: 8416ffe5-f07b-4304-85ca-be3a3e15f5e7
    // desc: NOT, JSON
    fn not_filt_json() {
        let filter_json_str = r#"{"txnFilter":{"TxnFilterNOT":{"txnFilter":{"NullaryFALSE":{}}}}}"#;

        let filter_text_str = indoc! {
            "Filter:
               NOT
                 None pass
            "};

        let tf_res = serde_json::from_str::<FilterDefinition>(filter_json_str);
        assert!(tf_res.is_ok());
        let tf = tf_res.unwrap();

        match tf.txn_filter {
            TxnFilter::TxnFilterNOT(_) => assert!(true),
            _ => assert!(false),
        }

        assert_eq!(format!("{tf}"), filter_text_str);
        assert_eq!(serde_json::to_string(&tf).unwrap(), filter_json_str);
    }

    #[test]
    // test: 22482f84-2d21-48eb-8161-c16dfa8f9920
    // desc: NOT, Text
    fn not_filt_text() {
        let filter_text_str = indoc! {
            "Filter:
               NOT
                 NOT
                   All pass
            "};

        let tfd = FilterDefinition {
            txn_filter: TxnFilter::TxnFilterNOT(TxnFilterNOT {
                txn_filter: Box::new(TxnFilter::TxnFilterNOT(TxnFilterNOT {
                    txn_filter: Box::new(TxnFilter::NullaryTRUE(NullaryTRUE {})),
                })),
            }),
        };

        assert_eq!(format!("{tfd}"), filter_text_str);
    }
}
