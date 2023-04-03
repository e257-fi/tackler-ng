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

/// Txn Posting Commodity filter
///
/// Select the transaction, if any of its postings' commodity match `regex`
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TxnFilterPostingCommodity {
    #[doc(hidden)]
    #[serde(with = "serde_regex")]
    pub regex: Regex,
}

impl IndentDisplay for TxnFilterPostingCommodity {
    fn i_fmt(&self, indent: &str, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{indent}Posting Commodity: \"{}\"", self.regex.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filters::{logic::TxnFilterAND, FilterDefinition, NullaryTRUE, TxnFilter};
    use indoc::indoc;
    use regex::Regex;
    use tackler_rs::IndocWithMarker;

    #[test]
    // test: b7b43b0f-0046-4d25-8f61-2ef419b84f0b
    // desc: PostingCommodity, JSON
    fn posting_commodity_json() {
        let filter_json_str =
            r#"{"txnFilter":{"TxnFilterPostingCommodity":{"regex":"(abc.*)|(def.*)"}}}"#;

        let filter_text_str = indoc! {
        r#"|Filter:
           |  Posting Commodity: "(abc.*)|(def.*)"
           |"#}
        .strip_margin();

        let tf_res = serde_json::from_str::<FilterDefinition>(filter_json_str);
        assert!(tf_res.is_ok());
        let tf = tf_res.unwrap(/*:test:*/);

        match tf.txn_filter {
            TxnFilter::TxnFilterPostingCommodity(_) => assert!(true),
            _ => assert!(false),
        }

        assert_eq!(format!("{tf}"), filter_text_str);
        assert_eq!(
            serde_json::to_string(&tf).unwrap(/*:test:*/),
            filter_json_str
        );
    }

    #[test]
    // test: 15d83e84-11a6-4ec2-a458-82fea493f10f
    // desc: PostingCommodity, Text
    fn posting_commodity_text() {
        let filter_text_str = indoc! {
        r#"|Filter:
           |  AND
           |    Posting Commodity: "(abc.*)|(def.*)"
           |    AND
           |      Posting Commodity: "xyz"
           |      All pass
           |"#}
        .strip_margin();

        let tfd = FilterDefinition {
            txn_filter: TxnFilter::TxnFilterAND(TxnFilterAND {
                txn_filters: vec![
                    TxnFilter::TxnFilterPostingCommodity(TxnFilterPostingCommodity {
                        regex: Regex::new("(abc.*)|(def.*)").unwrap(/*:test:*/),
                    }),
                    TxnFilter::TxnFilterAND(TxnFilterAND {
                        txn_filters: vec![
                            TxnFilter::TxnFilterPostingCommodity(TxnFilterPostingCommodity {
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
