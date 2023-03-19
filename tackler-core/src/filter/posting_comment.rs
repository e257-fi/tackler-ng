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
use tackler_api::filters::TxnFilterPostingComment;

use super::FilterTxn;

impl FilterTxn for TxnFilterPostingComment {
    fn filter(&self, txn: &Transaction) -> bool {
        txn.posts
            .iter()
            .any(|p| p.comment.as_ref().map_or(false, |c| self.regex.is_match(c)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filter::tests::make_default_txn;
    use crate::model::{AccountTreeNode, Posting, Transaction};
    use regex::Regex;
    use rust_decimal::Decimal;
    use tackler_api::filters::TxnFilter;
    use tackler_api::TxnHeader;

    pub(crate) fn make_posts_comment_txn(
        comment: Option<&str>,
        a: &str,
        a_value: i64,
        e: &str,
    ) -> Transaction {
        let e_v = Decimal::new(a_value, 0);
        let e_acctn = AccountTreeNode::from(e.to_string(), None).unwrap();
        let e_p =
            Posting::from(e_acctn, e_v, e_v, false, None, comment.map(str::to_string)).unwrap();

        let a_v = Decimal::new(-1 * a_value, 0);
        let a_acctn = AccountTreeNode::from(a.to_string(), None).unwrap();
        let a_p = Posting::from(a_acctn, a_v, a_v, false, None, None).unwrap();

        Transaction::from(TxnHeader::default(), vec![e_p, a_p]).unwrap()
    }

    #[test]
    // test: 0c1dcffe-152d-4959-89bb-2c48677ad171
    // desc: filter by posting comments
    fn posting_comment() {
        let tf = TxnFilterPostingComment {
            regex: Regex::new("abc.*").unwrap(),
        };

        let cases: Vec<(Transaction, bool)> = vec![
            (make_default_txn(None), false),
            (
                make_posts_comment_txn(Some(""), "a:the:abc", 123, "e:the:def"),
                false,
            ),
            (
                make_posts_comment_txn(None, "a:the:abc", 123, "e:the:def"),
                false,
            ),
            (make_posts_comment_txn(Some("abc"), "a", 123, "e"), true),
        ];

        for t in cases.iter() {
            assert_eq!(tf.filter(&t.0), t.1);
        }

        // test: e8517888-b459-4ecf-a622-592fd16aa067
        // desc: TxnFilter::TxnFilterPostingComment
        let filt = TxnFilter::TxnFilterPostingComment(tf);
        for t in cases {
            assert_eq!(filt.filter(&t.0), t.1);
        }
    }
}
