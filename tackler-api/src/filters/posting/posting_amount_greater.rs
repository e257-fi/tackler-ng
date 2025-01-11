/*
 * Tackler-NG 2023-2024
 *
 * SPDX-License-Identifier: Apache-2.0
 */

use regex::Regex;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
use tackler_rs::regex::peeled_pattern;
use tackler_rs::regex::serde::full_haystack_matcher;

use crate::filters::{posting_filter_indent_fmt, IndentDisplay};

/// Txn Posting "Amount is Greater than" filter
///
/// Select the transaction, if its posting match `regex` with amount greater than `amount`
///
/// Q: Why there is also account regex as parameter?
///
/// A: To support negative amounts as an argument.
///    Sum of all postings inside transaction must be zero.
///    If you select "more than some negative amount",
///    then all transactions will match, because there must
///    be postings with positive amounts in every transaction
///    to zero out the whole transaction.
///    Hence the filter would be useless without account selector.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TxnFilterPostingAmountGreater {
    #[doc(hidden)]
    #[serde(with = "full_haystack_matcher")]
    pub regex: Regex,
    #[doc(hidden)]
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub amount: Decimal,
}

impl IndentDisplay for TxnFilterPostingAmountGreater {
    fn i_fmt(&self, indent: &str, f: &mut Formatter<'_>) -> std::fmt::Result {
        posting_filter_indent_fmt(
            indent,
            "Posting Amount",
            peeled_pattern(&self.regex),
            ">",
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

    use tackler_rs::regex::new_full_haystack_regex;
    use tackler_rs::IndocUtils;

    #[test]
    // test: 8609eb58-f600-42d1-a20a-c7de2c57e6e2
    // desc: PostingAmountGreater, full haystack match
    fn posting_amount_greater_full_haystack() {
        let filter_json_str =
            r#"{"txnFilter":{"TxnFilterPostingAmountGreater":{"regex":"o.a","amount":1}}}"#;

        let tf_res = serde_json::from_str::<FilterDefinition>(filter_json_str);
        assert!(tf_res.is_ok());
        let tf = tf_res.unwrap(/*:test:*/);

        match &tf.txn_filter {
            TxnFilter::TxnFilterPostingAmountGreater(f) => {
                assert!(!f.regex.is_match("foobar"));
                assert!(!f.regex.is_match("obar"));
                assert!(!f.regex.is_match("ooba"));

                assert!(f.regex.is_match("oba"));
            }
            _ => panic!(/*:test:*/),
        }
    }

    #[test]
    // test: 66d6ee10-a18e-4615-9e7a-1569c793fe46
    // desc: PostingAmountGreater, JSON
    fn posting_amount_greater_json() {
        let filter_json_str = r#"{"txnFilter":{"TxnFilterPostingAmountGreater":{"regex":"(abc.*)|(def.*)","amount":1}}}"#;

        let filter_text_str = indoc! {
        r#"|Filter
           |  Posting Amount
           |    account: "(abc.*)|(def.*)"
           |    amount > 1
           |"#}
        .strip_margin();

        let tf_res = serde_json::from_str::<FilterDefinition>(filter_json_str);
        assert!(tf_res.is_ok());
        let tf = tf_res.unwrap(/*:test:*/);

        match tf.txn_filter {
            TxnFilter::TxnFilterPostingAmountGreater(_) => (),
            _ => panic!(/*:test:*/),
        }

        assert_eq!(format!("{tf}"), filter_text_str);
        assert_eq!(
            serde_json::to_string(&tf).unwrap(/*:test:*/),
            filter_json_str
        );
    }

    #[test]
    // test: f940a623-f4b6-4937-86ff-c05ddc1921d6
    // desc: PostingAmountGreater, Text
    fn posting_amount_greater_text() {
        let filter_text_str = indoc! {
        r#"|Filter
           |  AND
           |    Posting Amount
           |      account: "(abc.*)|(def.*)"
           |      amount > 1
           |    AND
           |      Posting Amount
           |        account: "xyz"
           |        amount > 2
           |      All pass
           |"#}
        .strip_margin();

        let tfd = FilterDefinition {
            txn_filter: TxnFilter::TxnFilterAND(TxnFilterAND {
                txn_filters: vec![
                    TxnFilter::TxnFilterPostingAmountGreater(TxnFilterPostingAmountGreater {
                        regex: new_full_haystack_regex("(abc.*)|(def.*)").unwrap(/*:test:*/),
                        amount: Decimal::from(1),
                    }),
                    TxnFilter::TxnFilterAND(TxnFilterAND {
                        txn_filters: vec![
                            TxnFilter::TxnFilterPostingAmountGreater(
                                TxnFilterPostingAmountGreater {
                                    regex: new_full_haystack_regex("xyz").unwrap(/*:test:*/),
                                    amount: Decimal::from(2),
                                },
                            ),
                            TxnFilter::NullaryTRUE(NullaryTRUE {}),
                        ],
                    }),
                ],
            }),
        };

        assert_eq!(format!("{tfd}"), filter_text_str);
    }
}
