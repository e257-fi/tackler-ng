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
use tackler_api::filters::TxnFilterTxnDescription;

use super::FilterTxn;

impl FilterTxn for TxnFilterTxnDescription {
    fn filter(&self, txn: &Transaction) -> bool {
        txn.header
            .description
            .as_ref()
            .map_or(false, |desc| self.regex.is_match(desc))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filter::tests::make_code_txn;
    use crate::filter::tests::make_default_txn;
    use crate::filter::tests::make_desc_txn;
    use crate::model::Transaction;
    use regex::Regex;
    use tackler_api::filters::{TxnFilter, TxnFilterTxnDescription};

    #[test]
    // test: 59157c61-0ced-4b3a-ab8d-ec5edf7aafb4
    // desc: filter by txn description
    fn txn_description() {
        let tf = TxnFilterTxnDescription {
            regex: Regex::new("ab.*").unwrap(),
        };

        let cases: Vec<(fn(Option<&str>) -> Transaction, Option<&str>, bool)> = vec![
            (make_default_txn, None, false),
            (make_desc_txn, Some(""), false),
            (make_desc_txn, Some("abc"), true),
            (make_desc_txn, Some("foo"), false),
            (make_code_txn, Some("abc"), false),
        ];

        for t in cases.iter() {
            let txn = t.0(t.1);
            assert_eq!(tf.filter(&txn), t.2);
        }

        // test: 3bca6d7b-e515-42d8-b65b-2780b0c0d7e0
        // desc: TxnFilter::TxnFilterTxnCode
        let filt = TxnFilter::TxnFilterTxnDescription(tf);
        for t in cases {
            let txn = t.0(t.1);
            assert_eq!(filt.filter(&txn), t.2);
        }
    }
}
