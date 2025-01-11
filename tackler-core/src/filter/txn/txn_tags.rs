/*
 * Tackler-NG 2023
 *
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::model::Transaction;
use tackler_api::filters::txn::TxnFilterTxnTags;

use crate::kernel::Predicate;

impl Predicate<Transaction> for TxnFilterTxnTags {
    fn eval(&self, txn: &Transaction) -> bool {
        txn.header
            .tags
            .as_ref()
            .map_or(false, |tags| tags.iter().any(|t| self.regex.is_match(t)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filter::tests::make_default_v_txn;
    use crate::filter::tests::make_tags_txn;
    use crate::model::Transaction;
    use regex::Regex;
    use tackler_api::filters::TxnFilter;

    #[test]
    // test: f3d05712-3c6e-482c-bfb9-8b559b8f6eb9
    // desc: filter by txn tags
    fn txn_tags() {
        let tf = TxnFilterTxnTags {
            regex: Regex::new("ab.*").unwrap(/*:test:*/),
        };

        #[allow(clippy::type_complexity)]
        let cases: Vec<(
            fn(Option<Vec<&str>>) -> Transaction,
            Option<Vec<&str>>,
            bool,
        )> = vec![
            (make_default_v_txn, None, false),
            (make_tags_txn, Some(vec![""]), false),
            (make_tags_txn, Some(vec!["", "a", "abc", "b"]), true),
            (make_tags_txn, Some(vec!["abc", "abcdef"]), true),
            (make_tags_txn, Some(vec!["", "a", "def", "b"]), false),
        ];

        for t in cases.iter().cloned() {
            let txn = t.0(t.1);
            assert_eq!(tf.eval(&txn), t.2);
        }

        // test: 82ff060b-11c4-4b91-8e3f-2298f0a201af
        // desc: TxnFilter::TxnFilterTxnTags
        let filt = TxnFilter::TxnFilterTxnTags(tf);
        for t in cases {
            let txn = t.0(t.1);
            assert_eq!(filt.eval(&txn), t.2);
        }
    }
}
