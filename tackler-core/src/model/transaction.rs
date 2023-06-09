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

use crate::model::{posting, Posts};
use std::cmp::Ordering;
use std::error::Error;
use std::fmt::{Display, Formatter};
use tackler_api::txn_header::TxnHeader;
use tackler_api::txn_ts;

#[derive(Debug, Default)]
pub struct Transaction {
    pub(crate) header: TxnHeader,
    pub(crate) posts: Posts,
}

impl Transaction {
    pub fn from(header: TxnHeader, posts: Posts) -> Result<Transaction, Box<dyn Error>> {
        let txn_sum = posting::txn_sum(&posts);
        if !txn_sum.is_zero() {
            let msg = format!("TXN postings do not zero: {txn_sum}");
            return Err(msg.into());
        }

        Ok(Transaction { header, posts })
    }
}

pub fn ord_by_txn(before: &Transaction, after: &Transaction) -> Ordering {
    before.cmp(after)
}

impl Eq for Transaction {}

impl Ord for Transaction {
    fn cmp(&self, other: &Self) -> Ordering {
        self.header.cmp(&other.header)
    }
}

impl PartialOrd for Transaction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq<Self> for Transaction {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header
    }
}

impl Display for Transaction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let indent = "   ";
        write!(
            f,
            "{}{}",
            self.header.to_string_with_indent(indent, txn_ts::rfc_3339),
            self.posts
                .iter()
                .map(|p| { format!("{indent}{p}\n") })
                .collect::<String>()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use rust_decimal::Decimal;
    use tackler_rs::IndocUtils;
    use time::macros::datetime;

    use crate::model::{AccountTreeNode, Posting};
    use tackler_api::txn_header::TxnHeader;

    #[test]
    fn txn_to_display() {
        let ts = datetime!(2023-02-04 14:03:05.047974 +02:00);

        let tnx_hdr = TxnHeader {
            timestamp: ts,
            code: None,
            description: Some("desc".to_string()),
            uuid: None,
            location: None,
            tags: None,
            comments: None,
        };

        let atn_ab = AccountTreeNode::from("a:b".to_string(), None).unwrap(/*:test:*/);
        let atn_cd = AccountTreeNode::from("c:d".to_string(), None).unwrap(/*:test:*/);
        let atn_ef = AccountTreeNode::from("e:f".to_string(), None).unwrap(/*:test:*/);

        let ef_post = Posting::from(
            atn_ef,
            Decimal::from_str_exact("1").unwrap(/*:test:*/),
            Decimal::from_str_exact("0").unwrap(/*:test:*/),
            false,
            None,
            None,
        )
        .unwrap(/*:test:*/);
        let cd_post = Posting::from(
            atn_cd,
            Decimal::from_str_exact("2").unwrap(/*:test:*/),
            Decimal::from_str_exact("0").unwrap(/*:test:*/),
            false,
            None,
            None,
        )
        .unwrap(/*:test:*/);
        let ab_post = Posting::from(
            atn_ab,
            Decimal::from_str_exact("-3").unwrap(/*:test:*/),
            Decimal::from_str_exact("0").unwrap(/*:test:*/),
            false,
            None,
            None,
        )
        .unwrap(/*:test:*/);

        let tests: Vec<(Transaction, String)> = vec![(
            Transaction {
                header: tnx_hdr,
                posts: vec![ef_post, cd_post, ab_post],
            },
            indoc!(
                "|2023-02-04T14:03:05.047974+02:00 'desc
                 |   e:f   1
                 |   c:d   2
                 |   a:b  -3
                 |"
            )
            .strip_margin(),
        )];

        let mut count = 0;
        let should_be_count = tests.len();
        for t in tests {
            let txn_hdr_str = format!("{}", t.0);
            assert_eq!(txn_hdr_str, t.1);
            count += 1;
        }
        assert_eq!(count, should_be_count);
    }
}
