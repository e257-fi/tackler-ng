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
use tackler_api::filters::TxnFilterTxnUUID;

use super::FilterTxn;

impl FilterTxn for TxnFilterTxnUUID {
    fn filter(&self, txn: &Transaction) -> bool {
        txn.header
            .uuid
            .as_ref()
            .map_or(false, |uuid| uuid == &self.uuid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filter::tests::make_default_txn;
    use crate::filter::tests::make_uuid_txn;
    use crate::model::Transaction;
    use tackler_api::filters::{TxnFilter, TxnFilterTxnUUID};
    use uuid::Uuid;

    #[test]
    // test: f6f2853b-fce4-4577-8fc3-3089e717de0b
    // desc: filter by txn uuid
    fn txn_uuid() {
        let tf = TxnFilterTxnUUID {
            uuid: Uuid::parse_str("842ded5c-e176-4e59-85a7-af2ded001d55").unwrap(/*:test:*/),
        };

        let cases: Vec<(fn(Option<&str>) -> Transaction, Option<&str>, bool)> = vec![
            (
                // test: 6bf82dff-374a-4bf2-bdad-a882b59df932
                // desc: check filter for txns without no UUID
                make_default_txn,
                None,
                false,
            ),
            (
                make_uuid_txn,
                Some("842ded5c-e176-4e59-85a7-af2ded001d55"),
                true,
            ),
            (
                make_uuid_txn,
                Some("02aa0341-f07a-4125-bf16-3b9d44beb37c"),
                false,
            ),
        ];

        for t in cases.iter() {
            let txn = t.0(t.1);
            assert_eq!(tf.filter(&txn), t.2);
        }

        // test: 3e461f5b-d1fe-4e2e-aca3-c205f64befd7
        // desc: TxnFilter::TxnFilterTxnUUID
        let filt = TxnFilter::TxnFilterTxnUUID(tf);
        for t in cases {
            let txn = t.0(t.1);
            assert_eq!(filt.filter(&txn), t.2);
        }
    }
}
