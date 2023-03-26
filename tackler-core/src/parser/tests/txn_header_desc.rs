/*
 * Copyright 2019-2023 E257.FI
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
// * core/src/test/scala/fi/e257/tackler/parser/TacklerParserHeaderDescriptionTest.scala
//
use indoc::indoc;
use crate::kernel::Settings;
use crate::parser;
use super::*;
use crate::tests::IndocWithMarker;



      #[test]
      //desc: "check invalid description constructs"
      #[allow(non_snake_case)]
      fn id_03d3df34_e68a_4104_b8ab_be06d36bf189__err_description_parse() {
        let  perr_strings: Vec<(String, &str, &str)> = vec![
        (indoc!(
           "|
            |2017-01-01 (123) abc
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 2",
          r#"at input ' abc'"#
        ),
        (indoc!(
           "|
            |2017-01-01 (123) (abc
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 2",
          r#"at input ' ('"#
        ),
        (indoc!(
           "|
            |2017-01-01 )abc
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 2",
          r#"at input ' )'"#
        ),
        (indoc!(
           "|
            |2017-01-01 +02:00
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 2",
          r#"at input ' +'"#
        ),
        (indoc!(
           "|
            |2017-01-01 -02:00
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 2",
          r#"at input ' -02'"#
        ),
        (indoc!(
           "|
            |2017-01-01 Z
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 2",
          r#"at input ' Z'"#
        ),

        (indoc!(
           "|
            |2017-01-01 T 00:00:00Z
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 2",
          r#"at input ' T'"#
        ),

        (indoc!(
           "|
            |2017-01-01 T 00:00:00 Z
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 2",
          r#"at input ' T'"#
        ),

        (indoc!(
           "|
            |2017-01-01 (123) )abc
            | a 1
            | e 1
            |
            |").strip_margin(),
          "line: 2",
          r#"at input ' )'"#
        ),
      ];
        let mut count = 0;
        for t in perr_strings {
          let res = parser::string_to_txns(&t.0, &Settings::default());
          assert!(res.is_err(),
                  "Testing Error: Offending test vector item: {}", count);
          assert!(res.err().unwrap(/*:test:*/).to_string().contains(t.1),
                  "Testing Line: Offending test vector item: {}", count);
          // todo: parser error messages, error position
          //assert(ex.getMessage.contains(perrStr._3))
          count = count + 1;
        }
        assert_eq!(count, 9);
    }

    #[test]
    //desc: "accept valid description constructs"
    #[allow(non_snake_case)]
    fn id_58d08778_10ee_489c_bb91_7059b9ba0cca__ok_description() {
      let pok_strings: Vec<(String, &str)> = vec![
        (indoc!(
           "|
            |2017-01-01 'abc
            | a 1
            | e -1
            |
            |").strip_margin(),
          "abc"
        ),
        (indoc!(
           "|
            |2017-01-01   'abc
            | a 1
            | e -1
            |
            |").strip_margin(),
          "abc"
        ),
        (indoc!(
           "|
            |2017-01-01 \t \t   'abc
            | a 1
            | e -1
            |
            |").strip_margin(),
          "abc"
        ),
        (indoc!(
            "|
             |2017-01-01 'abc   \n\
             | a 1
             | e -1
             |
             |").strip_margin(),
          "abc"
        ),
        (indoc!(
            "|
             |2017-01-01 'abc \t \n\
             | a 1
             | e -1
             |
             |").strip_margin(),
          "abc"
        ),
        (indoc!(
           "|
            |2017-01-01 '123
            | a 1
            | e -1
            |
            |").strip_margin(),
          "123"
        ),
        (indoc!(
           "|
            |2017-01-01 '1.23
            | a 1
            | e -1
            |
            |").strip_margin(),
          "1.23"
        ),
        (indoc!(
           "|
            |2017-01-01 '(abc
            | a 1
            | e -1
            |
            |").strip_margin(),
          "(abc"
        ),
        (indoc!(
           "|
            |2017-01-01   '
            | a 1
            | e -1
            |
            |").strip_margin(),
          ""
        ),
        (indoc!(
           "|
            |2017-01-01  '   a
            | a 1
            | e -1
            |
            |").strip_margin(),
          "   a"
        ),
        (indoc!(
           "|
            |2017-01-01 'abc'
            | a 1
            | e -1
            |
            |").strip_margin(),
          "abc'"
        ),
        (indoc!(
           "|
            |2017-01-01 ''
            | a 1
            | e -1
            |
            |").strip_margin(),
          "'"
        ),
        (indoc!(
           "|
            |2017-01-01  '  '
            | a 1
            | e -1
            |
            |").strip_margin(),
          "  '"
        ),
        (indoc!(
           "|
            |2017-01-01  '''
            | a 1
            | e -1
            |
            |").strip_margin(),
          "''"
        ),
        (indoc!(
           "|
            |2017-01-01  ''''
            | a 1
            | e -1
            |
            |").strip_margin(),
          "'''"
        ),
        (indoc!(
           "|
            |2017-01-01 'a'b'
            | a 1
            | e -1
            |
            |").strip_margin(),
          "a'b'"
        ),
        (indoc!(
           "|
            |2017-01-01 'a'b''
            | a 1
            | e -1
            |
            |").strip_margin(),
          "a'b''"
        ),
      ];

        let mut count = 0;
        for t in pok_strings {
            let res = parser::string_to_txns(&t.0, &Settings::default());
            assert!(res.is_ok(), "Offending test vector item: {}", count);
            let txn: &Transaction = &res.unwrap(/*:test:*/).txns[0];
            assert_eq!(txn_desc_to_string(txn), t.1.to_string());
            count = count + 1;
        }
        assert_eq!(count, 17);
    }


    #[test]
    //desc: "accept valid code + description constructs"
    #[allow(non_snake_case)]
    fn id_5081594a_ecaf_4232_9c93_1d84ea7600eb__ok_code_and_description() {
      let  pok_strings: Vec<(String, &str, &str)> = vec![
        (indoc!(
           "|
            |2017-01-01 (123) 'abc
            | a 1
            | e -1
            |
            |").strip_margin(),
          "123",
          "abc"
        ),
        (indoc!(
           "|
            |2017-01-01 (123)  \t 'abc
            | a 1
            | e -1
            |
            |").strip_margin(),
          "123",
          "abc"
        ),
        (indoc!(
           "|
             |2017-01-01 \t (123) \t 'abc
             | a 1
             | e -1
             |
             |").strip_margin(),
          "123",
          "abc"
        ),

        (indoc!(
           "|
            |2017-01-01 (123)  \t '(abc
            | a 1
            | e -1
            |
            |").strip_margin(),
          "123",
          "(abc"
        ),
        (indoc!(
           "|
            |2017-01-01 (123) '
            | a 1
            | e -1
            |
            |").strip_margin(),
          "123",
          ""
        ),
        (indoc!(
           "|
             |2017-01-01 (123) ' \t \n\
             | a 1
             | e -1
             |
             |").strip_margin(),
          "123",
          ""
        ),
        (indoc!(
           "|
            |2017-01-01 (123) '   a
            | a 1
            | e -1
            |
            |").strip_margin(),
          "123",
          "   a"
        ),
        (indoc!(
           "|
            |2017-01-01 (123) 'abc'
            | a 1
            | e -1
            |
            |").strip_margin(),
          "123",
          "abc'"
        ),
        (indoc!(
           "|
            |2017-01-01 (123) ''
            | a 1
            | e -1
            |
            |").strip_margin(),
          "123",
          "'"
        ),
        (indoc!(
           "|
            |2017-01-01 (123) '  '
            | a 1
            | e -1
            |
            |").strip_margin(),
          "123",
          "  '"
        ),
        (indoc!(
           "|
            |2017-01-01 (123) '''
            | a 1
            | e -1
            |
            |").strip_margin(),
          "123",
          "''"
        ),
        (indoc!(
           "|
            |2017-01-01 (123) ''''
            | a 1
            | e -1
            |
            |").strip_margin(),
          "123",
          "'''"
        ),
        (indoc!(
           "|
            |2017-01-01 (123) 'a'b'
            | a 1
            | e -1
            |
            |").strip_margin(),
          "123",
          "a'b'"
        ),
        (indoc!(
           "|
            |2017-01-01 (123) 'a'b''
            | a 1
            | e -1
            |
            |").strip_margin(),
          "123",
          "a'b''"
        ),
      ];

        let mut count = 0;
        for t in pok_strings {
            let res = parser::string_to_txns(&t.0, &Settings::default());
            assert!(res.is_ok(), "Offending test vector item: {}", count);
            let txn: &Transaction = &res.unwrap(/*:test:*/).txns[0];
            assert_eq!(&txn.header.code.as_ref().unwrap(/*:test:*/).to_string(), &t.1.to_string());
            assert_eq!(&txn.header.description.as_ref().unwrap(/*:test:*/).to_string(), &t.2.to_string());
            count = count + 1;
        }
        assert_eq!(count, 14);
    }
