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
use tackler_api::filters::txn::TxnFilterTxnTSEnd;

use crate::kernel::Predicate;

impl Predicate<Transaction> for TxnFilterTxnTSEnd {
    fn eval(&self, txn: &Transaction) -> bool {
        match txn.header.timestamp.cmp(&self.end) {
            Ordering::Less => true,
            Ordering::Equal => false,
            Ordering::Greater => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filter::tests::make_ts_txn;
    use tackler_api::filters::TxnFilter;
    use time::format_description::well_known::Rfc3339;
    use time::macros::datetime;
    use time::OffsetDateTime;

    #[test]
    // test: 42a42f07-dea5-45ee-b563-187f9121e1e1
    // desc: filter by date
    fn filter_by_date() {
        let tf = TxnFilterTxnTSEnd {
            end: datetime!(2018-02-01 00:00:00 UTC),
        };

        let cases: Vec<(&str, bool)> = vec![
            ("2018-01-01T00:00:00Z", true),
            ("2018-02-01T00:00:00Z", false),
            ("2018-03-01T00:00:00Z", false),
        ];

        for t in cases.iter() {
            let txn = make_ts_txn(OffsetDateTime::parse(t.0, &Rfc3339).unwrap(/*:test:*/));
            assert_eq!(tf.eval(&txn), t.1);
        }

        // test: 3d3d06cb-fcf2-422a-b57d-1bdb08f7d65e
        // desc: TxnFilter::TxnFilterTxnTSEnd
        let filt = TxnFilter::TxnFilterTxnTSEnd(tf);
        for t in cases {
            let txn = make_ts_txn(OffsetDateTime::parse(t.0, &Rfc3339).unwrap(/*:test:*/));
            assert_eq!(filt.eval(&txn), t.1);
        }
    }

    #[test]
    // test: 4e566d2b-da32-4336-9b7f-d7c4a59658d2
    // desc: filter by time
    fn filter_by_time() {
        let tf = TxnFilterTxnTSEnd {
            end: datetime!(2018-01-01 23:00:00 UTC),
        };

        let cases: Vec<(&str, bool)> = vec![
            ("2018-01-01T11:00:00Z", true),
            ("2018-01-01T23:00:00Z", false),
            ("2018-01-02T00:00:00Z", false),
        ];

        for t in cases {
            let txn = make_ts_txn(OffsetDateTime::parse(t.0, &Rfc3339).unwrap(/*:test:*/));
            assert_eq!(tf.eval(&txn), t.1);
        }
    }

    #[test]
    // test: f6081a60-92a9-4051-85d7-c993e3cc03be
    // desc: filter by nanoseconds
    fn filter_by_nanosecond() {
        let tf = TxnFilterTxnTSEnd {
            end: datetime!(2018-01-01 14:00:00.123456788 UTC),
        };

        let cases: Vec<(&str, bool)> = vec![
            ("2018-01-01T14:00:00.123456787Z", true),
            ("2018-01-01T14:00:00.123456788Z", false),
            ("2018-01-01T14:00:00.123456789Z", false),
        ];

        for t in cases {
            let txn = make_ts_txn(OffsetDateTime::parse(t.0, &Rfc3339).unwrap(/*:test:*/));
            assert_eq!(tf.eval(&txn), t.1);
        }
    }

    #[test]
    // test: ab53df34-d22a-4256-9c4d-6d1ccf0ef32e
    // desc: filter by timezone
    fn filter_by_timezone() {
        let tf = TxnFilterTxnTSEnd {
            end: datetime!(2018-01-04 00:00:00 UTC),
        };

        let cases: Vec<(&str, bool)> = vec![
            ("2018-01-04T09:00:00+10:00", true),
            ("2018-01-03T18:00:00-06:00", false),
            ("2018-01-04T00:00:00+00:00", false),
        ];

        for t in cases {
            let txn = make_ts_txn(OffsetDateTime::parse(t.0, &Rfc3339).unwrap(/*:test:*/));
            assert_eq!(tf.eval(&txn), t.1);
        }
    }
}
