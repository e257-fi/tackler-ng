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

/// Txn Posting "Amount is Less than" filter
///
/// Select the transaction, if its posting match `regex` with amount less than `amount`
///
/// Q: Why there is also account regex as parameter?
///
/// A: To support positive amounts as an argument.
///    Sum of all postings inside transaction must be zero.
///    If you select "less than some positive amount",
///    then all transactions will match, because there must
///    be postings with negative amounts in every transaction
///    to zero out the whole transaction.
///    Hence the filter would be useless without account selector.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TxnFilterPostingAmountLess {
    #[doc(hidden)]
    #[serde(with = "serde_regex")]
    pub regex: Regex,
    #[doc(hidden)]
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub amount: Decimal,
}

impl IndentDisplay for TxnFilterPostingAmountLess {
    fn i_fmt(&self, indent: &str, f: &mut Formatter<'_>) -> std::fmt::Result {
        posting_filter_indent_fmt(
            indent,
            "Posting Amount",
            self.regex.as_str(),
            "<",
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
    use tackler_rs::IndocUtils;

    #[test]
    // test: 3dbd4103-66ee-4747-8eae-75d6b13bdb29
    // desc: PostingAmountLess, JSON
    fn posting_amount_less_json() {
        let filter_json_str = r#"{"txnFilter":{"TxnFilterPostingAmountLess":{"regex":"(abc.*)|(def.*)","amount":1}}}"#;

        let filter_text_str = indoc! {
        r#"|Filter:
           |  Posting Amount
           |    account: "(abc.*)|(def.*)"
           |    amount < 1
           |"#}
        .strip_margin();

        let tf_res = serde_json::from_str::<FilterDefinition>(filter_json_str);
        assert!(tf_res.is_ok());
        let tf = tf_res.unwrap(/*:test:*/);

        match tf.txn_filter {
            TxnFilter::TxnFilterPostingAmountLess(_) => assert!(true),
            _ => assert!(false),
        }

        assert_eq!(format!("{tf}"), filter_text_str);
        assert_eq!(
            serde_json::to_string(&tf).unwrap(/*:test:*/),
            filter_json_str
        );
    }

    #[test]
    // test: c0725d0c-2261-4a98-982f-4a62c4f9c7da
    // desc: PostingAmountLess, Text
    fn posting_amount_less_text() {
        let filter_text_str = indoc! {
        r#"|Filter:
           |  AND
           |    Posting Amount
           |      account: "(abc.*)|(def.*)"
           |      amount < 1
           |    AND
           |      Posting Amount
           |        account: "xyz"
           |        amount < 2
           |      All pass
           |"#}
        .strip_margin();

        let tfd = FilterDefinition {
            txn_filter: TxnFilter::TxnFilterAND(TxnFilterAND {
                txn_filters: vec![
                    TxnFilter::TxnFilterPostingAmountLess(TxnFilterPostingAmountLess {
                        regex: Regex::new("(abc.*)|(def.*)").unwrap(/*:test:*/),
                        amount: Decimal::from(1),
                    }),
                    TxnFilter::TxnFilterAND(TxnFilterAND {
                        txn_filters: vec![
                            TxnFilter::TxnFilterPostingAmountLess(TxnFilterPostingAmountLess {
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
