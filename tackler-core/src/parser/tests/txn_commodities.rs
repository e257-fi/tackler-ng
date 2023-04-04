/*
 * Copyright 2017-2023 E257.FI
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
// * core/src/test/scala/fi/e257/tackler/parser/TacklerParserCommoditiesTest.scala
//
use indoc::indoc;
use crate::kernel::Settings;
use crate::parser;
use tackler_rs::IndocUtils;


//
// "Units and Commodities") {
//
    #[test]
    //desc: "accept commodity names"
    #[allow(non_snake_case)]
    fn id_aadbdf7c_c1d0_4e1e_a02f_9ca1b5ab2afc__ok_uncommon_accounts() {
      let  txns_str =
  indoc!("|
          |2017-01-01
          | e   1 USD
          | a
          |
          |2019-01-01
          | e   1 €
          | a
          |
          |2019-01-01
          | e   1 ¢
          | a
          |
          |2019-01-01
          | e   1 $
          | a
          |
          |2019-01-01
          | e   1 £
          | a
          |
          |2019-01-01
          | e   1 ¥
          | a
          |
          |2019-01-01
          | e   1 ¤
          | a
          |
          |2019-01-01
          | e   1 Au·µg
          | a
          |
          |").strip_margin();

        let res = parser::string_to_txns(&txns_str, &Settings::default());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(/*:test:*/).len(), 8);
      }

    #[test]
    //desc: "uac ; comment"
    #[allow(non_snake_case)]
    fn ok_commodity_and_comment_parse() {
      let  txns_str =
  indoc!("|
          |2017-01-01
          | e   1 USD; comment
          | a
          |
          |2017-01-01
          | e   1 USD ; comment
          | a
          |
          |").strip_margin();

        let res = parser::string_to_txns(&txns_str, &Settings::default());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(/*:test:*/).len(), 2);
    }

    #[test]
    //desc: "accepts closing position"
    #[allow(non_snake_case)]
    fn id_5f5dcb57_792d_49df_a491_2923612a0e2f__ok_closing_position() {
      let  txns_str =
  indoc!("|
          |2017-01-01
          | e   1 USD @ 1.20 EUR
          | a
          |
          |2019-01-01
          | e   1 USD @ 1 €
          | a
          |
          |2019-01-01
          | e   1 € @ 1 $
          | a
          |
          |2019-01-01
          | e   1 $ @ 1 £
          | a
          |
          |2019-01-01
          | e   1 £ @ 1 ¥
          | a
          |
          |2019-01-01
          | e   1 ¥ @ 1 ¢
          | a
          |
          |2019-01-01
          | e   1 ¢ @ 1 Au·µg
          | a
          |
          |2019-01-01
          | e   1 Au·µg @ 1 EUR
          | a
          |
          |").strip_margin();

        let res = parser::string_to_txns(&txns_str, &Settings::default());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(/*:test:*/).len(), 8);
    }

    #[test]
    //desc: "uac closing position ; comment"
    #[allow(non_snake_case)]
    fn ok_commodity_and_comment_closing_pos_parse() {
      let  txns_str =
  indoc!("|
          |2017-01-01
          | e   1 USD @ 1.20 EUR; comment
          | a
          |
          |2017-01-01
          | e   1 USD @ 1.20 EUR ; comment
          | a
          |
          |").strip_margin();

        let res = parser::string_to_txns(&txns_str, &Settings::default());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(/*:test:*/).len(), 2);
    }
//
//  describe("Profit and Loss parsing") {
//
    #[test]
    //desc: "opening with PnL"
    #[allow(non_snake_case)]
    fn id_9f711991_c9ae_4558_923c_95a69faff8bc__ok_opening_with_pnl() {
      let  txns_str =
  indoc!("|
          |2017-01-01
          | e   1 USD {1.20 EUR}
          | a
          |
          |2017-01-01
          | e   -1 USD {1.20 EUR}
          | a
          |
          |2019-01-01
          | e   1 USD {1 €}
          | a
          |
          |2019-01-01
          | e   1 € { 1 $ }
          | a
          |
          |2019-01-01
          | e   1 $ {1 £ }
          | a
          |
          |2019-01-01
          | e   1 £ { 1 ¥}
          | a
          |
          |2019-01-01
          | e   1 ¥ {1 ¢}
          | a
          |
          |2019-01-01
          | e   1 ¢ {1 Au·µg}
          | a
          |
          |2019-01-01
          | e   1 Au·µg {1 EUR}
          | a
          |
          |").strip_margin();

        let res = parser::string_to_txns(&txns_str, &Settings::default());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(/*:test:*/).len(), 9);
    }

    #[test]
    //desc: "opening with PnL ; comment"
    #[allow(non_snake_case)]
    fn id_92f75975_061b_4867_87f5_e25cf5b13d40__ok_closing_position() {
      let  txns_str =
  indoc!("|
          |2017-01-01
          | e   1 USD {1.20 EUR}; comment
          | a
          |
          |2017-01-01
          | e   1 USD {1.20 EUR} ; comment
          | a
          |
          |").strip_margin();

        let res = parser::string_to_txns(&txns_str, &Settings::default());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(/*:test:*/).len(), 2);
    }

    #[test]
    //desc: "closing position with PnL"
    #[allow(non_snake_case)]
    fn id_84d81380_8664_45d7_a9e1_523c38c7a963__ok_closing_position() {
      let  txns_str =
  indoc!("|
          |2017-01-01
          | e   1 USD {1.20 EUR} @ 1.09 EUR
          | a
          |
          |2017-01-01
          | e   -1 USD {1.20 EUR} @ 1.09 EUR
          | a
          |
          |2019-01-01
          | e   1 USD {1 €} @ 1.09 €
          | a
          |
          |2019-01-01
          | e   1 € { 1 $ } @ 1.09 $
          | a
          |
          |2019-01-01
          | e   1 $ {1 £ } @ 1.09 £
          | a
          |
          |2019-01-01
          | e   1 £ { 1 ¥} @ 1.09  ¥
          | a
          |
          |2019-01-01
          | e   1 ¥ {1 ¢} @ 1.09 ¢
          | a
          |
          |2019-01-01
          | e   1 ¢ {1 Au·µg} @ 1.09 Au·µg
          | a
          |
          |2019-01-01
          | e   1 ⁴ {1 EUR} @ 1.09 EUR
          | a
          |
          |").strip_margin();

        let res = parser::string_to_txns(&txns_str, &Settings::default());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(/*:test:*/).len(), 9);
    }

    #[test]
    //desc: "closing position with PnL ; comment"
    #[allow(non_snake_case)]
    fn id_c1fbac7b_e924_4eee_aed3_b11b51116f1a__ok_closing_position() {
      let  txns_str =
  indoc!("|
          |2017-01-01
          | e   1 USD {1.20 EUR} @ 1.09 EUR; comment
          | a
          |
          |2017-01-01
          | e   1 USD {1.20 EUR} @ 1.09 EUR ; comment
          | a
          |
          |").strip_margin();

        let res = parser::string_to_txns(&txns_str, &Settings::default());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(/*:test:*/).len(), 2);
    }


//
//  describe("Invalid inputs and errors") {
//
//    describe("Logical errors") {
//

      #[test]
      //desc: "Unit cost '{ ... }' with negative value"
      #[allow(non_snake_case)]
      fn id_5af5d0d8_ca6e_4a03_a939_99d9d2a4ec43__err() {
        let  txns_str =
    indoc!("|
            |2017-01-01
            | e   1.12 USD {-1.00 EUR}
            | a
            |
            |").strip_margin();

          let res = parser::string_to_txns(&txns_str, &Settings::default());
          assert!(res.is_err());
          let msg = res.err().unwrap(/*:test:*/).to_string();
          assert!(msg.contains("Unit cost"));
          assert!(msg.contains("is negative"));
      }

      #[test]
      //desc: "Unit price '@' with negative value"
      #[allow(non_snake_case)]
      fn id_a27b166c_e9c9_432c_bb9d_91915b51d76b__err() {
        let  txns_str =
    indoc!("|
            |2019-01-01
            | e 1 € @ -1.2 $
            | a 1.2 $
            |
            |").strip_margin();

          let res = parser::string_to_txns(&txns_str, &Settings::default());
          assert!(res.is_err());
          let msg = res.err().unwrap(/*:test:*/).to_string();
          assert!(msg.contains("Unit price"));
          assert!(msg.contains("is negative"));
      }

      #[test]
      //desc: "Unit price '@' with same primary and secondary commodity"
      #[allow(non_snake_case)]
      fn id_6d1868da_3b9f_45e4_a2c6_db003da4c720__err() {
        let  txns_str =
    indoc!("|
            |2019-01-01
            | e 1 € @ 1 €
            | a
            |
            |").strip_margin();

          let res = parser::string_to_txns(&txns_str, &Settings::default());
          assert!(res.is_err());
          let msg = res.err().unwrap(/*:test:*/).to_string();
          assert!(msg.contains("Both commodities are same for value position [€]"));
      }

      #[test]
      //desc: "Unit price '@' with discrepancy of commodities"
      #[allow(non_snake_case)]
      fn id_fe246259_2280_4d42_8360_6dd3e280b30a__err() {
        let  txns_str =
    indoc!("|
            |2019-01-01
            | e 1 € @ 1 $
            | a 1 € @ 1 £
            |
            |").strip_margin();

          let res = parser::string_to_txns(&txns_str, &Settings::default());
          assert!(res.is_err());
          let msg = res.err().unwrap(/*:test:*/).to_string();
          assert!(msg.contains("Different commodities without"));
      }

      #[test]
      //desc: "Total cost '=' with different sign (-1st vs. +2nd)"
      #[allow(non_snake_case)]
      fn id_6f45f594_c4e6_449a_b6d2_7f25e9479bd5__err() {
        let  txns_str =
    indoc!("|
            |2019-01-01
            | e -1 $ = 1 €
            | a
            |
            |").strip_margin();

          let res = parser::string_to_txns(&txns_str, &Settings::default());
          assert!(res.is_err());
          let msg = res.err().unwrap(/*:test:*/).to_string();
          assert!(msg.contains("Total cost"));
          assert!(msg.contains("different sign"));
      }

      #[test]
      //desc: "Total cost '=' with different sign (+1st vs. -2nd)"
      #[allow(non_snake_case)]
      fn id_aaf50217_1d04_49bd_a873_43a53be1c99f__err() {
        let  txns_str =
    indoc!("|
            |2019-01-01
            | e 1 $ = -1 €
            | a
            |
            |").strip_margin();

          let res = parser::string_to_txns(&txns_str, &Settings::default());
          assert!(res.is_err());
          let msg = res.err().unwrap(/*:test:*/).to_string();
          assert!(msg.contains("Total cost"));
          assert!(msg.contains("different sign"));
      }


      #[test]
      //desc: "Total cost '=' with same primary and secondary commodity"
      #[allow(non_snake_case)]
      fn id_aa52ac0a_278a_49e4_abad_fc2f00416a41__err() {
        let  txns_str =
    indoc!("|
            |2019-01-01
            | e 1 € = 1 €
            | a
            |
            |").strip_margin();

          let res = parser::string_to_txns(&txns_str, &Settings::default());
          assert!(res.is_err());
          assert!(res.err().unwrap(/*:test:*/).to_string().contains("Both commodities are same for value position [€]"));
      }

      #[test]
      //desc: "Total cost '=' with discrepancy of commodities"
      #[allow(non_snake_case)]
      fn id_20b89e3e_a987_4e83_bd89_2cbf288caecc__err() {
        let  txns_str =
    indoc!("|
            |2019-01-01
            | e 1 € = 1 $
            | a 1 € = 1 £
            |
            |").strip_margin();

          let res = parser::string_to_txns(&txns_str, &Settings::default());
          assert!(res.is_err());
          assert!(res.err().unwrap(/*:test:*/).to_string().contains("Different commodities without"));
      }

    #[test]
    //desc: "perr: duplicate commodity"
    #[allow(non_snake_case)]
    fn id_4babf379_9d88_49f3_8158_b9b7ff4e6eed__err_parse() {
      let  txns_str =
  indoc!("|
          |2017-01-01
          | e   1 USD EUR
          | a
          |
          |").strip_margin();

        let res = parser::string_to_txns(&txns_str, &Settings::default());
        assert!(res.is_err());
        assert!(res.err().unwrap(/*:test:*/).to_string().contains("line: 3"));
    }

    #[test]
    //desc: "perr: value position, no primary commodity"
    #[allow(non_snake_case)]
    fn id_e24aacdf_fba2_4dc7_8165_4270c8822559__err_parse() {
      let  txns_str =
  indoc!("|
          |2017-01-01
          | e   1 @ 1 EUR
          | a
          |
          |").strip_margin();

        let res = parser::string_to_txns(&txns_str, &Settings::default());
        assert!(res.is_err());
        assert!(res.err().unwrap(/*:test:*/).to_string().contains("line: 3"));
    }

    #[test]
    //desc: "perr: value position, no secondary commodity"
    #[allow(non_snake_case)]
    fn id_0d1beaf2_c30c_4008_943f_46aaf44e4f76__err_parse() {
      let  txns_str =
  indoc!("|
          |2017-01-01
          | e   1 USD @ 2
          | a
          |
          |").strip_margin();

        let res = parser::string_to_txns(&txns_str, &Settings::default());
        assert!(res.is_err());
        assert!(res.err().unwrap(/*:test:*/).to_string().contains("line: 3"));
    }

    #[test]
    //desc: "perr: missing value pos value"
    #[allow(non_snake_case)]
    fn id_3152ec2f_4d5f_4a0a_b88c_68f17bccf7c6__err_parse() {
      let  txns_str =
  indoc!("|
          |2017-01-01
          | e   1 USD @ EUR
          | a
          |
          |").strip_margin();

        let res = parser::string_to_txns(&txns_str, &Settings::default());
        assert!(res.is_err());
        assert!(res.err().unwrap(/*:test:*/).to_string().contains("line: 3"));
    }

    #[test]
    //desc: "perr: with opening (comm)"
    #[allow(non_snake_case)]
    fn id_bed02ea9_4191_4c98_b847_6b4e2a0fcb2d__err_parse() {
      let  txns_str =
  indoc!("|
          |2017-01-01
          | e   1 USD {1.00} @ 1.20 EUR
          | a
          |
          |").strip_margin();

        let res = parser::string_to_txns(&txns_str, &Settings::default());
        assert!(res.is_err());
        assert!(res.err().unwrap(/*:test:*/).to_string().contains("line: 3"));
    }

    #[test]
    //desc: "perr: with opening (value)"
    #[allow(non_snake_case)]
    fn id_ac4a6183_fb21_4847_8b3e_912f21fe5a6b__err_parse() {
      let  txns_str =
  indoc!("|
          |2017-01-01
          | e   1 USD {EUR} @ 1.20 EUR
          | a
          |
          |").strip_margin();

        let res = parser::string_to_txns(&txns_str, &Settings::default());
        assert!(res.is_err());
        assert!(res.err().unwrap(/*:test:*/).to_string().contains("line: 3"));
    }

    #[test]
    //desc: "perr: with missing @"
    #[allow(non_snake_case)]
    fn id_436d9ed5_b7a0_4e37_a7b4_86b00eb60e83__err_parse() {
      let  txns_str =
  indoc!("|
          |2017-01-01
          | e   1 USD {1.00 EUR}  1.20 EUR
          | a
          |
          |").strip_margin();

        let res = parser::string_to_txns(&txns_str, &Settings::default());
        assert!(res.is_err());
        assert!(res.err().unwrap(/*:test:*/).to_string().contains("line: 3"));
    }
