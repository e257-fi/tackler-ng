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

use tackler_api::filters::TxnFilter;

use crate::model::Transaction;
mod filter_definition;
mod logic_and;
mod logic_not;
mod logic_or;
mod nullary_false;
mod nullary_true;
mod posting_account;
mod posting_amount_equal;
mod posting_amount_greater;
mod posting_amount_less;
mod posting_comment;
mod posting_commodity;
mod txn_bbox_lat_lon;
mod txn_bbox_lat_lon_alt;
mod txn_code;
mod txn_comments;
mod txn_description;
mod txn_tags;
mod txn_ts_begin;
mod txn_ts_end;
mod txn_uuid;

/// Actual filtering implementation for [`TxnFilter`]
///
/// [`TxnFilter`]: ../tackler_api/filters/index.html
pub trait FilterTxn {
    fn filter(&self, txn: &Transaction) -> bool;
}

impl FilterTxn for TxnFilter {
    fn filter(&self, txn: &Transaction) -> bool {
        match self {
            // special nullary filters
            TxnFilter::NullaryTRUE(tf) => tf.filter(txn),
            TxnFilter::NullaryFALSE(tf) => tf.filter(txn),

            // logic filters
            TxnFilter::TxnFilterAND(tf) => tf.filter(txn),
            TxnFilter::TxnFilterOR(tf) => tf.filter(txn),
            TxnFilter::TxnFilterNOT(tf) => tf.filter(txn),

            // txn header filters
            TxnFilter::TxnFilterTxnTSBegin(tf) => tf.filter(txn),
            TxnFilter::TxnFilterTxnTSEnd(tf) => tf.filter(txn),
            TxnFilter::TxnFilterTxnCode(tf) => tf.filter(txn),
            TxnFilter::TxnFilterTxnDescription(tf) => tf.filter(txn),
            TxnFilter::TxnFilterTxnUUID(tf) => tf.filter(txn),
            TxnFilter::TxnFilterBBoxLatLon(tf) => tf.filter(txn),
            TxnFilter::TxnFilterBBoxLatLonAlt(tf) => tf.filter(txn),
            TxnFilter::TxnFilterTxnTags(tf) => tf.filter(txn),
            TxnFilter::TxnFilterTxnComments(tf) => tf.filter(txn),

            // txn posting filters
            TxnFilter::TxnFilterPostingAccount(tf) => tf.filter(txn),
            TxnFilter::TxnFilterPostingComment(tf) => tf.filter(txn),
            TxnFilter::TxnFilterPostingAmountEqual(tf) => tf.filter(txn),
            TxnFilter::TxnFilterPostingAmountLess(tf) => tf.filter(txn),
            TxnFilter::TxnFilterPostingAmountGreater(tf) => tf.filter(txn),
            TxnFilter::TxnFilterPostingCommodity(tf) => tf.filter(txn),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::{AccountTreeNode, Commodity, Posting};
    use chrono::{DateTime, FixedOffset};
    use rust_decimal::Decimal;
    use tackler_api::filters::{
        NullaryFALSE, NullaryTRUE, TxnFilter, TxnFilterAND, TxnFilterNOT, TxnFilterOR,
    };
    use tackler_api::{GeoPoint, TxnHeader};
    use uuid::Uuid;

    use super::*;

    pub(crate) fn make_ts_txn(ts: DateTime<FixedOffset>) -> Transaction {
        Transaction {
            header: TxnHeader {
                timestamp: ts,
                code: None,
                description: None,
                uuid: None,
                location: None,
                tags: None,
                comments: None,
            },
            posts: vec![],
        }
    }

    pub(crate) fn make_default_txn(_str: Option<&str>) -> Transaction {
        Transaction::default()
    }

    pub(crate) fn make_default_v_txn(_str: Option<Vec<&str>>) -> Transaction {
        Transaction::default()
    }

    pub(crate) fn make_code_txn(code: Option<&str>) -> Transaction {
        Transaction {
            header: TxnHeader {
                timestamp: DateTime::default(),
                code: code.map(str::to_string),
                description: None,
                uuid: None,
                location: None,
                tags: None,
                comments: None,
            },
            posts: vec![],
        }
    }

    pub(crate) fn make_desc_txn(desc: Option<&str>) -> Transaction {
        Transaction {
            header: TxnHeader {
                timestamp: DateTime::default(),
                code: None,
                description: desc.map(str::to_string),
                uuid: None,
                location: None,
                tags: None,
                comments: None,
            },
            posts: vec![],
        }
    }

    pub(crate) fn make_uuid_txn(uuid: Option<&str>) -> Transaction {
        Transaction {
            header: TxnHeader {
                timestamp: DateTime::default(),
                code: None,
                description: None,
                uuid: uuid.map(|uuid_str| Uuid::parse_str(uuid_str).unwrap(/*:test:*/)),
                location: None,
                tags: None,
                comments: None,
            },
            posts: vec![],
        }
    }

    pub(crate) fn make_geo_txn(lat: f64, lon: f64, alt: Option<f64>) -> Transaction {
        Transaction {
            header: TxnHeader {
                timestamp: DateTime::default(),
                code: None,
                description: None,
                uuid: None,
                location: GeoPoint::from(lat, lon, alt).ok(),
                tags: None,
                comments: None,
            },
            posts: vec![],
        }
    }

    pub(crate) fn make_tags_txn(tags: Option<Vec<&str>>) -> Transaction {
        Transaction {
            header: TxnHeader {
                timestamp: DateTime::default(),
                code: None,
                description: None,
                uuid: None,
                location: None,
                tags: tags.map(|tags| tags.iter().map(|t| str::to_string(*t)).collect()),
                comments: None,
            },
            posts: vec![],
        }
    }

    pub(crate) fn make_comments_txn(cmts: Option<Vec<&str>>) -> Transaction {
        Transaction {
            header: TxnHeader {
                timestamp: DateTime::default(),
                code: None,
                description: None,
                uuid: None,
                location: None,
                tags: None,
                comments: cmts
                    .map(|comments| comments.iter().map(|t| str::to_string(*t)).collect()),
            },
            posts: vec![],
        }
    }

    pub(crate) fn make_posts_txn(e: &str, e_value: i64, a: &str) -> Transaction {
        let e_v = Decimal::new(e_value, 0);
        let e_acctn = AccountTreeNode::from(e.to_string(), None).unwrap(/*:test:*/);
        let e_p = Posting::from(e_acctn, e_v, e_v, false, None, Some("comment".to_string())).unwrap(/*:test:*/);

        let a_v = Decimal::new(-1 * e_value, 0);
        let a_acctn = AccountTreeNode::from(a.to_string(), None).unwrap(/*:test:*/);
        let a_p = Posting::from(a_acctn, a_v, a_v, false, None, Some("comment".to_string())).unwrap(/*:test:*/);

        Transaction::from(
            TxnHeader {
                timestamp: DateTime::default(),
                code: None,
                description: None,
                uuid: None,
                location: None,
                tags: None,
                comments: None,
            },
            vec![e_p, a_p],
        )
        .unwrap(/*:test:*/)
    }

    pub(crate) fn make_posts_commodity_txn(
        c: Option<&str>,
        a: &str,
        a_value: i64,
        e: &str,
    ) -> Transaction {
        fn make_commodity(c: Option<&str>) -> Option<Commodity> {
            c.map(|c| Commodity::from(c.to_string()).unwrap(/*:test:*/))
        }

        let e_v = Decimal::new(a_value, 0);
        let e_acctn = AccountTreeNode::from(e.to_string(), make_commodity(c)).unwrap(/*:test:*/);
        let e_p = Posting::from(
            e_acctn,
            e_v,
            e_v,
            false,
            make_commodity(Some("txn_comm")),
            None,
        )
        .unwrap(/*:test:*/);

        let a_v = Decimal::new(-1 * a_value, 0);
        let a_acctn = AccountTreeNode::from(a.to_string(), make_commodity(c)).unwrap(/*:test:*/);
        let a_p = Posting::from(
            a_acctn,
            a_v,
            a_v,
            false,
            make_commodity(Some("txn_comm")),
            None,
        )
        .unwrap(/*:test:*/);

        Transaction::from(TxnHeader::default(), vec![e_p, a_p]).unwrap(/*:test:*/)
    }

    #[test]
    fn complex_and() {
        let txn = Transaction::default();
        //
        // AND(AND(...) style tests are done by logic_and::tests
        //
        let filters: Vec<(TxnFilter, bool)> = vec![
            (
                // test: 54cbd549-5567-4b19-bc20-a3de146fff40
                // desc: "AND(filter, AND(...))"
                TxnFilter::TxnFilterAND(TxnFilterAND {
                    txn_filters: vec![
                        TxnFilter::NullaryTRUE(NullaryTRUE {}),
                        TxnFilter::TxnFilterAND(TxnFilterAND {
                            txn_filters: vec![
                                TxnFilter::NullaryFALSE(NullaryFALSE {}),
                                TxnFilter::NullaryTRUE(NullaryTRUE {}),
                            ],
                        }),
                    ],
                }),
                false,
            ),
            (
                // test: 6e544624-ad3e-4920-9946-7eaf94febfb5
                // desc: "AND(filter, OR(...))"
                TxnFilter::TxnFilterAND(TxnFilterAND {
                    txn_filters: vec![
                        TxnFilter::NullaryTRUE(NullaryTRUE {}),
                        TxnFilter::TxnFilterOR(TxnFilterOR {
                            txn_filters: vec![
                                TxnFilter::NullaryFALSE(NullaryFALSE {}),
                                TxnFilter::NullaryTRUE(NullaryTRUE {}),
                            ],
                        }),
                    ],
                }),
                true,
            ),
            (
                // test: ef81d4c1-9d5e-47f2-ab7c-646fbc49e268
                // desc: "AND(filter, NOT(...))"
                TxnFilter::TxnFilterAND(TxnFilterAND {
                    txn_filters: vec![
                        TxnFilter::NullaryTRUE(NullaryTRUE {}),
                        TxnFilter::TxnFilterNOT(TxnFilterNOT {
                            txn_filter: Box::new(TxnFilter::NullaryFALSE(NullaryFALSE {})),
                        }),
                    ],
                }),
                true,
            ),
            (
                // test: b2e5d857-e02c-4313-9ca7-9aa765033343
                // desc: "AND(AND(...), OR(...))"
                TxnFilter::TxnFilterAND(TxnFilterAND {
                    txn_filters: vec![
                        TxnFilter::TxnFilterAND(TxnFilterAND {
                            txn_filters: vec![
                                TxnFilter::NullaryTRUE(NullaryTRUE {}),
                                TxnFilter::NullaryTRUE(NullaryTRUE {}),
                            ],
                        }),
                        TxnFilter::TxnFilterOR(TxnFilterOR {
                            txn_filters: vec![
                                TxnFilter::NullaryFALSE(NullaryFALSE {}),
                                TxnFilter::NullaryTRUE(NullaryTRUE {}),
                            ],
                        }),
                    ],
                }),
                true,
            ),
            (
                // test: dab44c95-834c-438a-8543-a73547284f03
                // desc: "AND(filter, AND(...), OR(...), NOT(...))"
                TxnFilter::TxnFilterAND(TxnFilterAND {
                    txn_filters: vec![
                        TxnFilter::NullaryTRUE(NullaryTRUE {}),
                        TxnFilter::TxnFilterAND(TxnFilterAND {
                            txn_filters: vec![
                                TxnFilter::NullaryTRUE(NullaryTRUE {}),
                                TxnFilter::NullaryTRUE(NullaryTRUE {}),
                            ],
                        }),
                        TxnFilter::TxnFilterOR(TxnFilterOR {
                            txn_filters: vec![
                                TxnFilter::NullaryFALSE(NullaryFALSE {}),
                                TxnFilter::NullaryTRUE(NullaryTRUE {}),
                            ],
                        }),
                        TxnFilter::TxnFilterNOT(TxnFilterNOT {
                            txn_filter: Box::new(TxnFilter::NullaryFALSE(NullaryFALSE {})),
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

    #[test]
    fn complex_or() {
        let txn = Transaction::default();
        //
        // OR(OR(...) style tests are done by logic_or::tests
        //
        let filters: Vec<(TxnFilter, bool)> = vec![
            (
                // test: b75466f3-f7bf-4e7f-9865-e2937a5d968d
                // desc: "OR(filter, AND(...))"
                TxnFilter::TxnFilterOR(TxnFilterOR {
                    txn_filters: vec![
                        TxnFilter::NullaryFALSE(NullaryFALSE {}),
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
            (
                // test: 9029ad79-bbea-4c0c-a0e0-09c8b1b04188
                // desc: "OR(filter, OR(...))"
                TxnFilter::TxnFilterOR(TxnFilterOR {
                    txn_filters: vec![
                        TxnFilter::NullaryFALSE(NullaryFALSE {}),
                        TxnFilter::TxnFilterOR(TxnFilterOR {
                            txn_filters: vec![
                                TxnFilter::NullaryFALSE(NullaryFALSE {}),
                                TxnFilter::NullaryTRUE(NullaryTRUE {}),
                            ],
                        }),
                    ],
                }),
                true,
            ),
            (
                // test: b01bfc0d-0f6d-409b-8101-4647c70d1409
                // desc: "OR(filter, NOT(...))"
                TxnFilter::TxnFilterOR(TxnFilterOR {
                    txn_filters: vec![
                        TxnFilter::NullaryFALSE(NullaryFALSE {}),
                        TxnFilter::TxnFilterNOT(TxnFilterNOT {
                            txn_filter: Box::new(TxnFilter::NullaryFALSE(NullaryFALSE {})),
                        }),
                    ],
                }),
                true,
            ),
            (
                // test: e8c40011-4aef-4639-98e2-1362a0961db8
                // desc: "OR(AND(...), OR(...))"
                TxnFilter::TxnFilterOR(TxnFilterOR {
                    txn_filters: vec![
                        TxnFilter::TxnFilterAND(TxnFilterAND {
                            txn_filters: vec![
                                TxnFilter::NullaryTRUE(NullaryTRUE {}),
                                TxnFilter::NullaryTRUE(NullaryTRUE {}),
                            ],
                        }),
                        TxnFilter::TxnFilterOR(TxnFilterOR {
                            txn_filters: vec![
                                TxnFilter::NullaryFALSE(NullaryFALSE {}),
                                TxnFilter::NullaryFALSE(NullaryFALSE {}),
                            ],
                        }),
                    ],
                }),
                true,
            ),
            (
                // test: 4b127707-c83b-418b-9703-849ee304a19c
                // desc: "OR(filter, AND(...), OR(...), NOT(...))"
                TxnFilter::TxnFilterOR(TxnFilterOR {
                    txn_filters: vec![
                        TxnFilter::NullaryFALSE(NullaryFALSE {}),
                        TxnFilter::TxnFilterAND(TxnFilterAND {
                            txn_filters: vec![
                                TxnFilter::NullaryFALSE(NullaryFALSE {}),
                                TxnFilter::NullaryTRUE(NullaryTRUE {}),
                            ],
                        }),
                        TxnFilter::TxnFilterOR(TxnFilterOR {
                            txn_filters: vec![
                                TxnFilter::NullaryFALSE(NullaryFALSE {}),
                                TxnFilter::NullaryTRUE(NullaryTRUE {}),
                            ],
                        }),
                        TxnFilter::TxnFilterNOT(TxnFilterNOT {
                            txn_filter: Box::new(TxnFilter::NullaryTRUE(NullaryTRUE {})),
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

    #[test]
    fn complex_not() {
        let txn = Transaction::default();
        //
        // NOT(NOT(...) style tests are done by logic_not::tests
        //
        let filters: Vec<(TxnFilter, bool)> = vec![
            (
                // test: 3e03d091-4f06-44d3-8bf5-285c85178ff9
                // desc: "NOT(OR(...))"
                TxnFilter::TxnFilterNOT(TxnFilterNOT {
                    txn_filter: Box::new(TxnFilter::TxnFilterOR(TxnFilterOR {
                        txn_filters: vec![
                            TxnFilter::NullaryFALSE(NullaryFALSE {}),
                            TxnFilter::NullaryTRUE(NullaryTRUE {}),
                        ],
                    })),
                }),
                false,
            ),
            (
                // test: 0c549c6e-f3b7-4614-b874-31db1110c41c
                // desc: "NOT(AND(...))"
                TxnFilter::TxnFilterNOT(TxnFilterNOT {
                    txn_filter: Box::new(TxnFilter::TxnFilterAND(TxnFilterAND {
                        txn_filters: vec![
                            TxnFilter::NullaryFALSE(NullaryFALSE {}),
                            TxnFilter::NullaryTRUE(NullaryTRUE {}),
                        ],
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
