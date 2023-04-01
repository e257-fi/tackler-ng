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

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

use crate::filters::IndentDisplay;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TxnFilterPostingComment {
    #[serde(with = "serde_regex")]
    pub regex: Regex,
}

impl IndentDisplay for TxnFilterPostingComment {
    fn i_fmt(&self, indent: &str, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{indent}Posting Comment: \"{}\"", self.regex.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filters::{FilterDefinition, NullaryTRUE, TxnFilter, TxnFilterAND};
    use indoc::indoc;
    use regex::Regex;
    use tackler_rs::IndocWithMarker;

    #[test]
    // test: 55401f74-0054-42ec-ab0b-17d4c9cda0be
    // desc: PostingComment, JSON
    fn posting_comment_json() {
        let filter_json_str =
            r#"{"txnFilter":{"TxnFilterPostingComment":{"regex":"(abc.*)|(def.*)"}}}"#;

        let filter_text_str = indoc! {
        r#"|Filter:
           |  Posting Comment: "(abc.*)|(def.*)"
           |"#}
        .strip_margin();

        let tf_res = serde_json::from_str::<FilterDefinition>(filter_json_str);
        assert!(tf_res.is_ok());
        let tf = tf_res.unwrap(/*:test:*/);

        match tf.txn_filter {
            TxnFilter::TxnFilterPostingComment(_) => assert!(true),
            _ => assert!(false),
        }

        assert_eq!(format!("{tf}"), filter_text_str);
        assert_eq!(
            serde_json::to_string(&tf).unwrap(/*:test:*/),
            filter_json_str
        );
    }

    #[test]
    // test: a1b05b26-3cca-4e56-925d-7ae7602f941a
    // desc: PostingComment, Text
    fn posting_comment_text() {
        let filter_text_str = indoc! {
        r#"|Filter:
           |  AND
           |    Posting Comment: "(abc.*)|(def.*)"
           |    AND
           |      Posting Comment: "xyz"
           |      All pass
           |"#}
        .strip_margin();

        let tfd = FilterDefinition {
            txn_filter: TxnFilter::TxnFilterAND(TxnFilterAND {
                txn_filters: vec![
                    TxnFilter::TxnFilterPostingComment(TxnFilterPostingComment {
                        regex: Regex::new("(abc.*)|(def.*)").unwrap(/*:test:*/),
                    }),
                    TxnFilter::TxnFilterAND(TxnFilterAND {
                        txn_filters: vec![
                            TxnFilter::TxnFilterPostingComment(TxnFilterPostingComment {
                                regex: Regex::new("xyz").unwrap(/*:test:*/),
                            }),
                            TxnFilter::NullaryTRUE(NullaryTRUE {}),
                        ],
                    }),
                ],
            }),
        };

        assert_eq!(format!("{tfd}"), filter_text_str);
    }
}
