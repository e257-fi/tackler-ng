/*
 * Copyright 2016-2023 E257.FI
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
//
// This is tackler test:
//    - https://gitlab.com/e257/accounting/tackler
//    - https://github.com/e257-fi/tackler
// * core/src/test/scala/fi/e257/tackler/parser/TacklerTxnsTest.scala
//
use indoc::indoc;
use crate::kernel::Settings;
use crate::parser;
use super::*;
use tackler_rs::IndocUtils;

//
// describe("TacklerTxns with String") {
//
    #[test]
    //desc: "tackler_txns with string input: notice unbalanced transaction"
    #[allow(non_snake_case)]
    fn id_52836ff9_94de_4575_bfae_6b5afa971351__err_unbalanced_txn() {
      let txns_str: Vec<(String, &str)> = vec![
        (indoc!(
         "|2017-01-01
          | e  1
          | a  1
          |").strip_margin(),
         "TXN postings do not zero: 2"),
        ];
      let mut count = 0;
      let should_be_count = txns_str.len();
      for t in txns_str {
        let res = parser::string_to_txns(&t.0, &Settings::default());
        assert!(res.is_err(),
                "Testing Error: Offending test vector item: {}", count);
        assert!(res.err().unwrap(/*:test:*/).to_string().contains(t.1),
                "Testing Line: Offending test vector item: {}", count);
        // todo: parser error messages, error position
        //assert(ex.getMessage.contains(perrStr._3))
        count = count + 1;
      }
      assert_eq!(count, should_be_count);
    }

    #[test]
    //desc: "tackler_txns with string input: txns_data is sorted"
    #[allow(non_snake_case)]
    fn id_200aad57_9275_4d16_bdad_2f1c484bcf17__ok_string_txns_are_sorted() {
      let txns_str =
          indoc!(
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
          |").strip_margin();

      let res = parser::string_to_txns(&txns_str, &Settings::default());
      assert!(res.is_ok());
      let txn_data = &res.unwrap(/*:test:*/);
      assert_eq!(txn_data.len(), 3);
      let txns = txn_data.get_all().unwrap(/*:test:*/);
      let txn_1: &Transaction = &txns.txns[0];
      let txn_3: &Transaction = &txns.txns[2];

      assert_eq!(txn_desc_to_string(txn_1), "txn-1 by str");
      assert_eq!(txn_desc_to_string(txn_3), "txn-3 by str");
    }
