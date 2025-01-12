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
use tackler_api::filters::txn::TxnFilterTxnComments;

use crate::kernel::Predicate;

impl Predicate<Transaction> for TxnFilterTxnComments {
    fn eval(&self, txn: &Transaction) -> bool {
        txn.header
            .comments
            .as_ref()
            .is_some_and(|tags| tags.iter().any(|t| self.regex.is_match(t)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filter::tests::make_comments_txn;
    use crate::filter::tests::make_default_v_txn;
    use crate::model::Transaction;
    use regex::Regex;
    use tackler_api::filters::TxnFilter;

    #[test]
    // test: 8bad2776-51fa-4766-839a-1bb99df44f5c
    // desc: filter by txn comments
    fn txn_comments() {
        let tf = TxnFilterTxnComments {
            regex: Regex::new("ab.*").unwrap(/*:test:*/),
        };

        #[allow(clippy::type_complexity)]
        let cases: Vec<(
            fn(Option<Vec<&str>>) -> Transaction,
            Option<Vec<&str>>,
            bool,
        )> = vec![
            (make_default_v_txn, None, false),
            (make_comments_txn, Some(vec![""]), false),
            (make_comments_txn, Some(vec!["", "a", "abc", "b"]), true),
            (make_comments_txn, Some(vec!["abc", "abcdef"]), true),
            (make_comments_txn, Some(vec!["", "a", "def", "b"]), false),
        ];

        for t in cases.iter().cloned() {
            let txn = t.0(t.1);
            assert_eq!(tf.eval(&txn), t.2);
        }

        // test: 593648ab-8973-4c80-b4e9-82e4011f9e32
        // desc: TxnFilter::TxnFilterTxnComments
        let filt = TxnFilter::TxnFilterTxnComments(tf);
        for t in cases {
            let txn = t.0(t.1);
            assert_eq!(filt.eval(&txn), t.2);
        }
    }
}
