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
use std::cmp::Ordering;
use tackler_api::filters::TxnFilterTxnTSBegin;

use super::FilterTxn;

impl FilterTxn for TxnFilterTxnTSBegin {
    fn filter(&self, txn: &Transaction) -> bool {
        match self.begin.cmp(&txn.header.timestamp) {
            Ordering::Less => true,
            Ordering::Equal => true,
            Ordering::Greater => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filter::tests::make_ts_txn;
    use chrono::{DateTime, FixedOffset, TimeZone, Timelike};
    use tackler_api::filters::{TxnFilter, TxnFilterTxnTSBegin};

    #[test]
    // test: 701b2c27-d33c-4460-9a5e-64316c6ed946
    // desc: filter by date
    fn filter_by_date() {
        let tf = TxnFilterTxnTSBegin {
            begin: FixedOffset::east_opt(0)
                .unwrap(/*:test:*/)
                .with_ymd_and_hms(2018, 2, 1, 0, 0, 0)
                .unwrap(/*:test:*/),
        };

        let cases: Vec<(DateTime<FixedOffset>, bool)> = vec![
            (
                "2018-01-01T00:00:00Z"
                    .parse::<DateTime<FixedOffset>>()
                    .unwrap(/*:test:*/),
                false,
            ),
            (
                "2018-02-01T00:00:00Z"
                    .parse::<DateTime<FixedOffset>>()
                    .unwrap(/*:test:*/),
                true,
            ),
            (
                "2018-03-01T00:00:00Z"
                    .parse::<DateTime<FixedOffset>>()
                    .unwrap(/*:test:*/),
                true,
            ),
        ];

        for t in cases.iter() {
            let txn = make_ts_txn(t.0);
            assert_eq!(tf.filter(&txn), t.1);
        }

        // test: 42dfcaca-b407-437e-9bc0-7f9618c1636e
        // desc: TxnFilter::TxnFilterTxnTSBegin
        let filt = TxnFilter::TxnFilterTxnTSBegin(tf);
        for t in cases {
            let txn = make_ts_txn(t.0);
            assert_eq!(filt.filter(&txn), t.1);
        }
    }

    #[test]
    // test: ec7cf2bd-e10e-4f46-9baa-4096881a5fbb
    // desc: filter by time
    fn filter_by_time() {
        let tf = TxnFilterTxnTSBegin {
            begin: FixedOffset::east_opt(0)
                .unwrap(/*:test:*/)
                .with_ymd_and_hms(2018, 1, 1, 23, 0, 0)
                .unwrap(/*:test:*/),
        };

        let cases: Vec<(DateTime<FixedOffset>, bool)> = vec![
            (
                "2018-01-01T11:00:00Z"
                    .parse::<DateTime<FixedOffset>>()
                    .unwrap(/*:test:*/),
                false,
            ),
            (
                "2018-01-01T23:00:00Z"
                    .parse::<DateTime<FixedOffset>>()
                    .unwrap(/*:test:*/),
                true,
            ),
            (
                "2018-01-02T00:00:00Z"
                    .parse::<DateTime<FixedOffset>>()
                    .unwrap(/*:test:*/),
                true,
            ),
        ];

        for t in cases {
            let txn = make_ts_txn(t.0);
            assert_eq!(tf.filter(&txn), t.1);
        }
    }

    #[test]
    // test: f1623bd0-f767-458e-bc68-6eadfa113fd1
    // desc: filter by nanoseconds
    fn filter_by_nanosecond() {
        let tf = TxnFilterTxnTSBegin {
            begin: FixedOffset::east_opt(0)
                .unwrap(/*:test:*/)
                .with_ymd_and_hms(2018, 1, 1, 14, 0, 0)
                .unwrap(/*:test:*/)
                .with_nanosecond(123456788)
                .unwrap(/*:test:*/),
        };

        let cases: Vec<(DateTime<FixedOffset>, bool)> = vec![
            (
                "2018-01-01T14:00:00.123456787Z"
                    .parse::<DateTime<FixedOffset>>()
                    .unwrap(/*:test:*/),
                false,
            ),
            (
                "2018-01-01T14:00:00.123456788Z"
                    .parse::<DateTime<FixedOffset>>()
                    .unwrap(/*:test:*/),
                true,
            ),
            (
                "2018-01-01T14:00:00.123456789Z"
                    .parse::<DateTime<FixedOffset>>()
                    .unwrap(/*:test:*/),
                true,
            ),
        ];

        for t in cases {
            let txn = make_ts_txn(t.0);
            assert_eq!(tf.filter(&txn), t.1);
        }
    }

    #[test]
    // test: 960cb7e7-b180-4276-a43b-714e53e1789b
    // desc: filter by timezone
    fn filter_by_timezone() {
        let tf = TxnFilterTxnTSBegin {
            begin: FixedOffset::east_opt(0)
                .unwrap(/*:test:*/)
                .with_ymd_and_hms(2018, 1, 4, 0, 0, 0)
                .unwrap(/*:test:*/),
        };

        let cases: Vec<(DateTime<FixedOffset>, bool)> = vec![
            (
                "2018-01-04T09:00:00+10:00"
                    .parse::<DateTime<FixedOffset>>()
                    .unwrap(/*:test:*/),
                false,
            ),
            (
                "2018-01-03T18:00:00-06:00"
                    .parse::<DateTime<FixedOffset>>()
                    .unwrap(/*:test:*/),
                true,
            ),
            (
                "2018-01-04T00:00:00+00:00"
                    .parse::<DateTime<FixedOffset>>()
                    .unwrap(/*:test:*/),
                true,
            ),
        ];

        for t in cases {
            let txn = make_ts_txn(t.0);
            assert_eq!(tf.filter(&txn), t.1);
        }
    }
}
