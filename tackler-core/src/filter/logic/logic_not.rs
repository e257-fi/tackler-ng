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
use tackler_api::filters::logic::TxnFilterNOT;

use crate::filter::FilterTxn;

impl FilterTxn for TxnFilterNOT {
    fn filter(&self, txn: &Transaction) -> bool {
        !self.txn_filter.filter(txn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tackler_api::filters::{NullaryFALSE, NullaryTRUE, TxnFilter};

    #[test]
    fn permutations() {
        let txn = Transaction::default();

        let filters: Vec<(TxnFilter, bool)> = vec![
            (
                // test: 08126158-2262-41f4-aa34-5695023d7a9b
                // desc: NOT(true)
                TxnFilter::TxnFilterNOT(TxnFilterNOT {
                    txn_filter: Box::new(TxnFilter::NullaryTRUE(NullaryTRUE {})),
                }),
                false,
            ),
            (
                // test: 32aa1190-d5f2-40eb-a494-3cb7969ab65a
                // desc: NOT(false)
                TxnFilter::TxnFilterNOT(TxnFilterNOT {
                    txn_filter: Box::new(TxnFilter::NullaryFALSE(NullaryFALSE {})),
                }),
                true,
            ),
            (
                // test: b280271f-a0a6-41e1-aa49-305b9f4a791e
                // desc: NOT(NOT(false))
                TxnFilter::TxnFilterNOT(TxnFilterNOT {
                    txn_filter: Box::new(TxnFilter::TxnFilterNOT(TxnFilterNOT {
                        txn_filter: Box::new(TxnFilter::NullaryFALSE(NullaryFALSE {})),
                    })),
                }),
                false,
            ),
            (
                // todo: new, record
                // test: dac1a5b1-9128-452f-94d9-06ab163b0a02
                // desc: NOT(NOT(true)
                TxnFilter::TxnFilterNOT(TxnFilterNOT {
                    txn_filter: Box::new(TxnFilter::TxnFilterNOT(TxnFilterNOT {
                        txn_filter: Box::new(TxnFilter::NullaryTRUE(NullaryTRUE {})),
                    })),
                }),
                true,
            ),
        ];

        let mut test_count = 0;
        let ref_count = filters.len();
        for tf in filters {
            assert_eq!(tf.0.filter(&txn), tf.1);
            test_count += 1;
        }
        assert_eq!(test_count, ref_count);
    }
}
