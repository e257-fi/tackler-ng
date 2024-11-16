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

use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

use crate::filters::IndentDisplay;

/// Txn Geo Location (2D) filter
///
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TxnFilterBBoxLatLon {
    /// min latitude
    pub south: f64,
    /// min longitude
    pub west: f64,
    /// max latitude
    pub north: f64,
    /// max longitude
    pub east: f64,
}

impl IndentDisplay for TxnFilterBBoxLatLon {
    fn i_fmt(&self, indent: &str, f: &mut Formatter<'_>) -> std::fmt::Result {
        let my_indent = format!("{indent}  ");
        writeln!(f, "{indent}Txn Bounding Box 2D")?;
        writeln!(
            f,
            "{my_indent}North, East: geo:{},{}",
            self.north, self.east
        )?;
        writeln!(
            f,
            "{my_indent}South, West: geo:{},{}",
            self.south, self.west
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filters::{logic::TxnFilterAND, FilterDefinition, NullaryTRUE, TxnFilter};
    use indoc::indoc;
    use tackler_rs::IndocUtils;

    #[test]
    // test: 05bfe9c0-0dc1-462a-b452-39c2eaf55d02
    // desc: BBoxLatLon, JSON
    fn txn_bbox_lat_lon_json() {
        let filter_json_str = r#"{"txnFilter":{"TxnFilterBBoxLatLon":{"south":59.85,"west":24.0,"north":60.8,"east":27.5}}}"#;

        let filter_text_str = indoc! {
        "|Filter
         |  Txn Bounding Box 2D
         |    North, East: geo:60.8,27.5
         |    South, West: geo:59.85,24
         |"}
        .strip_margin();

        let tf_res = serde_json::from_str::<FilterDefinition>(filter_json_str);
        assert!(tf_res.is_ok());
        let tf = tf_res.unwrap(/*:test:*/);

        match tf.txn_filter {
            TxnFilter::TxnFilterBBoxLatLon(_) => (),
            _ => panic!(/*:test:*/),
        }

        assert_eq!(format!("{tf}"), filter_text_str);
        assert_eq!(
            serde_json::to_string(&tf).unwrap(/*:test:*/),
            filter_json_str
        );
    }

    #[test]
    // test: 89d31f9c-029f-47ce-acb9-ddfaaa089782
    // desc: BBoxLatLon, Text
    fn txn_bbox_lat_lon_text() {
        let filter_text_str = indoc! {
        "|Filter
         |  AND
         |    Txn Bounding Box 2D
         |      North, East: geo:60.8,27.5
         |      South, West: geo:59.85,24
         |    AND
         |      Txn Bounding Box 2D
         |        North, East: geo:60.8,27.5
         |        South, West: geo:59.85,24
         |      All pass
         |"}
        .strip_margin();

        let tfd = FilterDefinition {
            txn_filter: TxnFilter::TxnFilterAND(TxnFilterAND {
                txn_filters: vec![
                    TxnFilter::TxnFilterBBoxLatLon(TxnFilterBBoxLatLon {
                        south: 59.85,
                        west: 24.0,
                        north: 60.8,
                        east: 27.5,
                    }),
                    TxnFilter::TxnFilterAND(TxnFilterAND {
                        txn_filters: vec![
                            TxnFilter::TxnFilterBBoxLatLon(TxnFilterBBoxLatLon {
                                south: 59.85,
                                west: 24.0,
                                north: 60.8,
                                east: 27.5,
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
