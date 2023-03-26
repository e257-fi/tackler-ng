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

use crate::model::Transaction;
use tackler_api::filters::TxnFilterTxnTags;

use super::FilterTxn;

impl FilterTxn for TxnFilterTxnTags {
    fn filter(&self, txn: &Transaction) -> bool {
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
    use tackler_api::filters::{TxnFilter, TxnFilterTxnTags};

    #[test]
    // test: f3d05712-3c6e-482c-bfb9-8b559b8f6eb9
    // desc: filter by txn tags
    fn txn_tags() {
        let tf = TxnFilterTxnTags {
            regex: Regex::new("ab.*").unwrap(/*:test:*/),
        };

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
            assert_eq!(tf.filter(&txn), t.2);
        }

        // test: 82ff060b-11c4-4b91-8e3f-2298f0a201af
        // desc: TxnFilter::TxnFilterTxnTags
        let filt = TxnFilter::TxnFilterTxnTags(tf);
        for t in cases {
            let txn = t.0(t.1);
            assert_eq!(filt.filter(&txn), t.2);
        }
    }
}
