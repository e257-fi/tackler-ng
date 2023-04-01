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

use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
use uuid::Uuid;

use crate::filters::IndentDisplay;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TxnFilterTxnUUID {
    pub uuid: Uuid,
}

impl IndentDisplay for TxnFilterTxnUUID {
    fn i_fmt(&self, indent: &str, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{indent}Txn UUID: {}", self.uuid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filters::{FilterDefinition, NullaryTRUE, TxnFilter, TxnFilterAND};
    use crate::tests::IndocWithMarker;
    use indoc::indoc;

    #[test]
    // test: 9ad41df9-c153-458b-a941-3b4763c25548
    // desc: TxnUUID, JSON
    fn txn_uuid_json() {
        let filter_json_str =
            r#"{"txnFilter":{"TxnFilterTxnUUID":{"uuid":"8c913372-48e9-466c-a897-11b151548a19"}}}"#;

        let filter_text_str = indoc! {
        "|Filter:
         |  Txn UUID: 8c913372-48e9-466c-a897-11b151548a19
         |"}
        .strip_margin();

        let tf_res = serde_json::from_str::<FilterDefinition>(filter_json_str);
        assert!(tf_res.is_ok());
        let tf = tf_res.unwrap(/*:test:*/);

        match tf.txn_filter {
            TxnFilter::TxnFilterTxnUUID(_) => assert!(true),
            _ => assert!(false),
        }

        assert_eq!(format!("{tf}"), filter_text_str);
        assert_eq!(
            serde_json::to_string(&tf).unwrap(/*:test:*/),
            filter_json_str
        );
    }

    #[test]
    // test: e388aecd-8500-4f89-98c6-9588199c104f
    // desc: TxnUUID, Text
    fn txn_uuid_text() {
        let filter_text_str = indoc! {
        "|Filter:
         |  AND
         |    Txn UUID: 76a0f143-d64e-4497-b357-5ae2eb092219
         |    AND
         |      Txn UUID: f01df5b5-18e2-477c-aaac-3e0b672b2729
         |      All pass
         |"}
        .strip_margin();

        let tfd = FilterDefinition {
            txn_filter: TxnFilter::TxnFilterAND(TxnFilterAND {
                txn_filters: vec![
                    TxnFilter::TxnFilterTxnUUID(TxnFilterTxnUUID {
                        uuid: Uuid::parse_str("76a0f143-d64e-4497-b357-5ae2eb092219").unwrap(/*:test:*/),
                    }),
                    TxnFilter::TxnFilterAND(TxnFilterAND {
                        txn_filters: vec![
                            TxnFilter::TxnFilterTxnUUID(TxnFilterTxnUUID {
                                uuid: Uuid::parse_str("f01df5b5-18e2-477c-aaac-3e0b672b2729")
                                    .unwrap(/*:test:*/),
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
