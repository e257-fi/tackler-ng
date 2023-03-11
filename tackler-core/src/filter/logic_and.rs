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
use tackler_api::filters::TxnFilterAND;

use super::FilterTxn;

impl FilterTxn for TxnFilterAND {
    fn filter(&self, txn: &Transaction) -> bool {
        self.txn_filters.iter().all(|f| f.filter(txn))
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
                // test: 2bd7fa78-adda-4f35-93eb-9b602bb3667e
                // desc: AND(false, false)
                TxnFilter::TxnFilterAND(TxnFilterAND {
                    txn_filters: vec![
                        TxnFilter::NullaryFALSE(NullaryFALSE {}),
                        TxnFilter::NullaryFALSE(NullaryFALSE {}),
                    ],
                }),
                false,
            ),
            (
                // test: 11d4409c-93e2-4670-b2d5-65073980ba2d
                // desc: AND(false, true)
                TxnFilter::TxnFilterAND(TxnFilterAND {
                    txn_filters: vec![
                        TxnFilter::NullaryFALSE(NullaryFALSE {}),
                        TxnFilter::NullaryTRUE(NullaryTRUE {}),
                    ],
                }),
                false,
            ),
            (
                // test: 7635059e-1828-48f7-9799-5bb0d327f446
                // desc: AND(true, false)
                TxnFilter::TxnFilterAND(TxnFilterAND {
                    txn_filters: vec![
                        TxnFilter::NullaryTRUE(NullaryTRUE {}),
                        TxnFilter::NullaryFALSE(NullaryFALSE {}),
                    ],
                }),
                false,
            ),
            (
                // test: bd589c45-4c80-4ccd-9f2f-49caf964d2a5
                // desc: AND(true, true)
                TxnFilter::TxnFilterAND(TxnFilterAND {
                    txn_filters: vec![
                        TxnFilter::NullaryTRUE(NullaryTRUE {}),
                        TxnFilter::NullaryTRUE(NullaryTRUE {}),
                    ],
                }),
                true,
            ),
            (
                // todo: new, record
                // test: 20cb5b36-d9fb-4c63-bd68-37394f2c0524
                // desc: AND(true, true, true)
                TxnFilter::TxnFilterAND(TxnFilterAND {
                    txn_filters: vec![
                        TxnFilter::NullaryTRUE(NullaryTRUE {}),
                        TxnFilter::NullaryTRUE(NullaryTRUE {}),
                        TxnFilter::NullaryTRUE(NullaryTRUE {}),
                    ],
                }),
                true,
            ),
            (
                // todo: new, record
                // test: 80b9bcbc-1274-440b-8e63-4be23bc6caa2
                // desc: AND(false, true, true)
                TxnFilter::TxnFilterAND(TxnFilterAND {
                    txn_filters: vec![
                        TxnFilter::NullaryFALSE(NullaryFALSE {}),
                        TxnFilter::NullaryTRUE(NullaryTRUE {}),
                        TxnFilter::NullaryTRUE(NullaryTRUE {}),
                    ],
                }),
                false,
            ),
            (
                // test: feb1a75c-cea8-40db-b4bf-ef4d59d49c9e
                // desc: AND(true, false, true)
                TxnFilter::TxnFilterAND(TxnFilterAND {
                    txn_filters: vec![
                        TxnFilter::NullaryTRUE(NullaryTRUE {}),
                        TxnFilter::NullaryFALSE(NullaryFALSE {}),
                        TxnFilter::NullaryTRUE(NullaryTRUE {}),
                    ],
                }),
                false,
            ),
            (
                // test: 456c6b08-7e61-410b-8a36-c3c47d6355b0
                // desc: AND(true, true, false)
                TxnFilter::TxnFilterAND(TxnFilterAND {
                    txn_filters: vec![
                        TxnFilter::NullaryTRUE(NullaryTRUE {}),
                        TxnFilter::NullaryTRUE(NullaryTRUE {}),
                        TxnFilter::NullaryFALSE(NullaryFALSE {}),
                    ],
                }),
                false,
            ),
            (
                // todo: new, record
                // test: 87107bc2-3c6d-435c-ac05-9ddade8352be
                // desc: AND(AND(true,false), AND(true,true)
                TxnFilter::TxnFilterAND(TxnFilterAND {
                    txn_filters: vec![
                        TxnFilter::TxnFilterAND(TxnFilterAND {
                            txn_filters: vec![
                                TxnFilter::NullaryTRUE(NullaryTRUE {}),
                                TxnFilter::NullaryFALSE(NullaryFALSE {}),
                            ],
                        }),
                        TxnFilter::TxnFilterAND(TxnFilterAND {
                            txn_filters: vec![
                                TxnFilter::NullaryTRUE(NullaryTRUE {}),
                                TxnFilter::NullaryTRUE(NullaryTRUE {}),
                            ],
                        }),
                    ],
                }),
                false,
            ),
            (
                // todo: new, record
                // test: d7c618df-3840-4cb3-b703-0896168ab448
                // desc: AND(AND(true,true),  AND(true,false)
                TxnFilter::TxnFilterAND(TxnFilterAND {
                    txn_filters: vec![
                        TxnFilter::TxnFilterAND(TxnFilterAND {
                            txn_filters: vec![
                                TxnFilter::NullaryTRUE(NullaryTRUE {}),
                                TxnFilter::NullaryTRUE(NullaryTRUE {}),
                            ],
                        }),
                        TxnFilter::TxnFilterAND(TxnFilterAND {
                            txn_filters: vec![
                                TxnFilter::NullaryTRUE(NullaryTRUE {}),
                                TxnFilter::NullaryFALSE(NullaryFALSE {}),
                            ],
                        }),
                    ],
                }),
                false,
            ),
            (
                // todo: new, record
                // test: b48c2765-12a7-4679-82e9-263f023fe731
                // desc: AND(AND(true,true),  AND(true,true)
                TxnFilter::TxnFilterAND(TxnFilterAND {
                    txn_filters: vec![
                        TxnFilter::TxnFilterAND(TxnFilterAND {
                            txn_filters: vec![
                                TxnFilter::NullaryTRUE(NullaryTRUE {}),
                                TxnFilter::NullaryTRUE(NullaryTRUE {}),
                            ],
                        }),
                        TxnFilter::TxnFilterAND(TxnFilterAND {
                            txn_filters: vec![
                                TxnFilter::NullaryTRUE(NullaryTRUE {}),
                                TxnFilter::NullaryTRUE(NullaryTRUE {}),
                            ],
                        }),
                    ],
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
