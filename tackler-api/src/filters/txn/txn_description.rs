/*
 * Copyright 2023-2024 E257.FI
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
use tackler_rs::regex::peeled_pattern;
use tackler_rs::regex::serde::full_haystack_matcher;

use crate::filters::IndentDisplay;

/// Txn Description filter
///
/// Select transaction if its description matches specified `regex`
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TxnFilterTxnDescription {
    #[doc(hidden)]
    #[serde(with = "full_haystack_matcher")]
    pub regex: Regex,
}

impl IndentDisplay for TxnFilterTxnDescription {
    fn i_fmt(&self, indent: &str, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{indent}Txn Description: \"{}\"",
            peeled_pattern(&self.regex)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filters::{logic::TxnFilterAND, FilterDefinition, NullaryTRUE, TxnFilter};
    use indoc::indoc;

    use tackler_rs::regex::new_full_haystack_regex;
    use tackler_rs::IndocUtils;

    #[test]
    // test: f2a52f9d-1fd6-428d-9bb4-7821d1f15ce3
    // desc: TxnDescription, full haystack match
    fn txn_description_full_haystack() {
        let filter_json_str = r#"{"txnFilter":{"TxnFilterTxnDescription":{"regex":"o.a"}}}"#;

        let tf_res = serde_json::from_str::<FilterDefinition>(filter_json_str);
        assert!(tf_res.is_ok());
        let tf = tf_res.unwrap(/*:test:*/);

        match &tf.txn_filter {
            TxnFilter::TxnFilterTxnDescription(f) => {
                assert!(!f.regex.is_match("foobar"));
                assert!(!f.regex.is_match("obar"));
                assert!(!f.regex.is_match("ooba"));

                assert!(f.regex.is_match("oba"));
            }
            _ => panic!(/*:test:*/),
        }
    }

    #[test]
    // test: 9cb8321a-0c43-4a24-b21e-0286dbe503cd
    // desc: TxnDescription, JSON
    fn txn_description_json() {
        let filter_json_str =
            r#"{"txnFilter":{"TxnFilterTxnDescription":{"regex":"(abc.*)|(def.*)"}}}"#;

        let filter_text_str = indoc! {
        r#"|Filter
           |  Txn Description: "(abc.*)|(def.*)"
           |"#}
        .strip_margin();

        let tf_res = serde_json::from_str::<FilterDefinition>(filter_json_str);
        assert!(tf_res.is_ok());
        let tf = tf_res.unwrap(/*:test:*/);

        match tf.txn_filter {
            TxnFilter::TxnFilterTxnDescription(_) => (),
            _ => panic!(/*:test:*/),
        }

        assert_eq!(format!("{tf}"), filter_text_str);
        assert_eq!(
            serde_json::to_string(&tf).unwrap(/*:test:*/),
            filter_json_str
        );
    }

    #[test]
    // test: ea88d0cf-2c60-45ac-835d-6f2f18a2c10d
    // desc: TxnDescription, Text
    fn txn_description_text() {
        let filter_text_str = indoc! {
        r#"|Filter
           |  AND
           |    Txn Description: "(abc.*)|(def.*)"
           |    AND
           |      Txn Description: "xyz"
           |      All pass
           |"#}
        .strip_margin();

        let tfd = FilterDefinition {
            txn_filter: TxnFilter::TxnFilterAND(TxnFilterAND {
                txn_filters: vec![
                    TxnFilter::TxnFilterTxnDescription(TxnFilterTxnDescription {
                        regex: new_full_haystack_regex("(abc.*)|(def.*)").unwrap(/*:test:*/),
                    }),
                    TxnFilter::TxnFilterAND(TxnFilterAND {
                        txn_filters: vec![
                            TxnFilter::TxnFilterTxnDescription(TxnFilterTxnDescription {
                                regex: new_full_haystack_regex("xyz").unwrap(/*:test:*/),
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
