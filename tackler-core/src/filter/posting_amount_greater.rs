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
use tackler_api::filters::TxnFilterPostingAmountGreater;

use super::FilterTxn;

impl FilterTxn for TxnFilterPostingAmountGreater {
    fn filter(&self, txn: &Transaction) -> bool {
        txn.posts
            .iter()
            .any(|p| p.amount > self.amount && self.regex.is_match(&p.acctn.account))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filter::tests::make_default_txn;
    use crate::filter::tests::make_posts_commodity_txn;
    use crate::filter::tests::make_posts_txn;
    use crate::model::Transaction;
    use regex::Regex;
    use rust_decimal::Decimal;
    use tackler_api::filters::{TxnFilter, TxnFilterPostingAccount};

    #[test]
    // test: b94b99d7-acfa-4a4b-871f-c1b6282738ff
    // desc: filter by posting amount (greater)
    fn posting_amount_greater() {
        let tf = TxnFilterPostingAmountGreater {
            regex: Regex::new("e:.*:abc").unwrap(),
            amount: Decimal::new(3, 0),
        };

        let cases: Vec<(Transaction, bool)> = vec![
            (make_default_txn(None), false),
            (make_posts_txn("e:the:abc", -5, "a:the:def"), false),
            (make_posts_txn("e:the:abc", 2, "a:the:def"), false),
            (make_posts_txn("e:the:abc", 3, "a:the:def"), false),
            (make_posts_txn("e:the:abc", 4, "a:the:def"), true),
            (make_posts_txn("e:not:b:c", 4, "a:the:def"), false),
        ];

        for t in cases.iter() {
            assert_eq!(tf.filter(&t.0), t.1);
        }

        // test: dde614b5-d368-4550-98bd-dc2e2e36aa9e
        // desc: TxnFilter::TxnFilterPostingAmountGreater
        let filt = TxnFilter::TxnFilterPostingAmountGreater(tf);
        for t in cases {
            assert_eq!(filt.filter(&t.0), t.1);
        }
    }
}
