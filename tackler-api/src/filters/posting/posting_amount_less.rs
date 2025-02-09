/*
 * Tackler-NG 2023-2024
 * SPDX-License-Identifier: Apache-2.0
 */

use jiff::tz::TimeZone;
use regex::Regex;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
use tackler_rs::regex::peeled_pattern;
use tackler_rs::regex::serde::full_haystack_matcher;

use crate::filters::{IndentDisplay, posting_filter_indent_fmt};

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
    #[serde(with = "full_haystack_matcher")]
    pub regex: Regex,
    #[doc(hidden)]
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub amount: Decimal,
}

impl IndentDisplay for TxnFilterPostingAmountLess {
    fn i_fmt(&self, indent: &str, _tz: TimeZone, f: &mut Formatter<'_>) -> std::fmt::Result {
        posting_filter_indent_fmt(
            indent,
            "Posting Amount",
            peeled_pattern(&self.regex),
            "<",
            &self.amount,
            f,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filters::{
        FilterDefZoned, FilterDefinition, NullaryTRUE, TxnFilter, logic::TxnFilterAND,
    };
    use indoc::indoc;
    use jiff::tz;
    use tackler_rs::IndocUtils;
    use tackler_rs::regex::new_full_haystack_regex;

    #[test]
    // test: 2d01669b-b051-4550-9436-ac31e84dd892
    // desc: PostingAmountLess, full haystack match
    fn posting_amount_less_full_haystack() {
        let filter_json_str =
            r#"{"txnFilter":{"TxnFilterPostingAmountLess":{"regex":"o.a","amount":1}}}"#;

        let tf_res = serde_json::from_str::<FilterDefinition>(filter_json_str);
        assert!(tf_res.is_ok());
        let tf = tf_res.unwrap(/*:test:*/);

        match &tf.txn_filter {
            TxnFilter::TxnFilterPostingAmountLess(f) => {
                assert!(!f.regex.is_match("foobar"));
                assert!(!f.regex.is_match("obar"));
                assert!(!f.regex.is_match("ooba"));

                assert!(f.regex.is_match("oba"));
            }
            _ => panic!(/*:test:*/),
        }
    }

    #[test]
    // test: 3dbd4103-66ee-4747-8eae-75d6b13bdb29
    // desc: PostingAmountLess, JSON
    fn posting_amount_less_json() {
        let filter_json_str = r#"{"txnFilter":{"TxnFilterPostingAmountLess":{"regex":"(abc.*)|(def.*)","amount":1}}}"#;

        let filter_text_str = indoc! {
        r#"|Filter
           |  Posting Amount
           |    account: "(abc.*)|(def.*)"
           |    amount < 1
           |"#}
        .strip_margin();

        let tf_res = serde_json::from_str::<FilterDefinition>(filter_json_str);
        assert!(tf_res.is_ok());
        let tf = tf_res.unwrap(/*:test:*/);

        match tf.txn_filter {
            TxnFilter::TxnFilterPostingAmountLess(_) => (),
            _ => panic!(/*:test:*/),
        }

        assert_eq!(
            format!(
                "{}",
                FilterDefZoned {
                    filt_def: &tf,
                    tz: tz::TimeZone::UTC
                }
            ),
            filter_text_str
        );
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
        r#"|Filter
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

        let tf = FilterDefinition {
            txn_filter: TxnFilter::TxnFilterAND(TxnFilterAND {
                txn_filters: vec![
                    TxnFilter::TxnFilterPostingAmountLess(TxnFilterPostingAmountLess {
                        regex: new_full_haystack_regex("(abc.*)|(def.*)").unwrap(/*:test:*/),
                        amount: Decimal::from(1),
                    }),
                    TxnFilter::TxnFilterAND(TxnFilterAND {
                        txn_filters: vec![
                            TxnFilter::TxnFilterPostingAmountLess(TxnFilterPostingAmountLess {
                                regex: new_full_haystack_regex("xyz").unwrap(/*:test:*/),
                                amount: Decimal::from(2),
                            }),
                            TxnFilter::NullaryTRUE(NullaryTRUE {}),
                        ],
                    }),
                ],
            }),
        };

        assert_eq!(
            format!(
                "{}",
                FilterDefZoned {
                    filt_def: &tf,
                    tz: tz::TimeZone::UTC
                }
            ),
            filter_text_str
        );
    }
}
