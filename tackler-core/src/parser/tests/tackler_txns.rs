/*
 * Tackler-NG 2016-2025
 * SPDX-License-Identifier: Apache-2.0
 */
use super::*;
use crate::kernel::Settings;
use crate::parser;
use indoc::indoc;
use tackler_rs::IndocUtils;

#[test]
fn txn_data_errors() {
    #[rustfmt::skip]
    let txns_str: Vec<(String, &str)> = vec![
        (
            // test: 52836ff9-94de-4575-bfae-6b5afa971351
            // desc: "tackler_txns with string input: notice unbalanced transaction"
            indoc!(
               "|2017-01-01
                | e  1
                | a  1
                |"
            ).strip_margin(),
            "TXN postings do not zero: 2",
        )
    ];
    let mut count = 0;
    let should_be_count = txns_str.len();
    for t in txns_str {
        let res = parser::string_to_txns(&mut t.0.as_ref(), &mut Settings::default());
        assert!(
            res.is_err(),
            "Testing Error: Offending test vector item: {}",
            count
        );
        assert!(
            res.err().unwrap(/*:test:*/).to_string().contains(t.1),
            "Testing Line: Offending test vector item: {}",
            count
        );
        count += 1;
    }
    assert_eq!(count, should_be_count);
}

#[test]
fn test_string_txns_are_sorted() {
    // test: 200aad57-9275-4d16-bdad-2f1c484bcf17
    // desc: "tackler_txns with string input: txns_data is sorted"
    #[rustfmt::skip]
    let txns_str = indoc!(
         "|2017-01-03 'txn-3 by str
          | e  1
          | a
          |
          |2017-01-01 'txn-1 by str
          | e  1
          | a
          |
          |2017-01-02 'txn-2 by str
          | e  1
          | a
          |
          |"
    )
    .strip_margin();

    let res = parser::string_to_txns(&mut txns_str.as_ref(), &mut Settings::default());
    assert!(res.is_ok());
    let txn_data = &res.unwrap(/*:test:*/);
    assert_eq!(txn_data.len(), 3);
    let txns = txn_data.get_all().unwrap(/*:test:*/);
    let txn_1: &Transaction = txns.txns[0];
    let txn_3: &Transaction = txns.txns[2];

    assert_eq!(txn_desc_to_string(txn_1), "txn-1 by str");
    assert_eq!(txn_desc_to_string(txn_3), "txn-3 by str");
}
