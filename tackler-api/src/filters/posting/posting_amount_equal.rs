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

use regex::Regex;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

use crate::filters::{posting_filter_indent_fmt, IndentDisplay};

/// Txn Posting "Amount is equal" filter
///
/// Select the transaction, if its posting match `regex` with exact `amount`
///
/// Q: Why is there also account regex as parameter?
///
/// A: For consistency with less and greater, where it's mandatory.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TxnFilterPostingAmountEqual {
    #[doc(hidden)]
    #[serde(with = "serde_regex")]
    pub regex: Regex,
    #[doc(hidden)]
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub amount: Decimal,
}

impl IndentDisplay for TxnFilterPostingAmountEqual {
    fn i_fmt(&self, indent: &str, f: &mut Formatter<'_>) -> std::fmt::Result {
        posting_filter_indent_fmt(
            indent,
            "Posting Amount",
            self.regex.as_str(),
            "==",
            &self.amount,
            f,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filters::{logic::TxnFilterAND, FilterDefinition, NullaryTRUE, TxnFilter};
    use indoc::indoc;
    use regex::Regex;
    use tackler_rs::IndocWithMarker;

    #[test]
    // test: b7b4543d-2ffa-488f-b251-af5a7ba7204f
    // desc: PostingAmountEqual, JSON
    fn posting_amount_equal_json() {
        let filter_json_str = r#"{"txnFilter":{"TxnFilterPostingAmountEqual":{"regex":"(abc.*)|(def.*)","amount":1}}}"#;

        let filter_text_str = indoc! {
        r#"|Filter:
           |  Posting Amount
           |    account: "(abc.*)|(def.*)"
           |    amount == 1
           |"#}
        .strip_margin();

        let tf_res = serde_json::from_str::<FilterDefinition>(filter_json_str);
        assert!(tf_res.is_ok());
        let tf = tf_res.unwrap(/*:test:*/);

        match tf.txn_filter {
            TxnFilter::TxnFilterPostingAmountEqual(_) => assert!(true),
            _ => assert!(false),
        }

        assert_eq!(format!("{tf}"), filter_text_str);
        assert_eq!(
            serde_json::to_string(&tf).unwrap(/*:test:*/),
            filter_json_str
        );
    }

    #[test]
    // test: c0f88d70-c388-4c4f-9cca-f29b921dbc41
    // desc: PostingAmountEqual, Text
    fn posting_amount_equal_text() {
        let filter_text_str = indoc! {
        r#"|Filter:
           |  AND
           |    Posting Amount
           |      account: "(abc.*)|(def.*)"
           |      amount == 1
           |    AND
           |      Posting Amount
           |        account: "xyz"
           |        amount == 2
           |      All pass
           |"#}
        .strip_margin();

        let tfd = FilterDefinition {
            txn_filter: TxnFilter::TxnFilterAND(TxnFilterAND {
                txn_filters: vec![
                    TxnFilter::TxnFilterPostingAmountEqual(TxnFilterPostingAmountEqual {
                        regex: Regex::new("(abc.*)|(def.*)").unwrap(/*:test:*/),
                        amount: Decimal::from(1),
                    }),
                    TxnFilter::TxnFilterAND(TxnFilterAND {
                        txn_filters: vec![
                            TxnFilter::TxnFilterPostingAmountEqual(TxnFilterPostingAmountEqual {
                                regex: Regex::new("xyz").unwrap(/*:test:*/),
                                amount: Decimal::from(2),
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
