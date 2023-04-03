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

use crate::filters::IndentDisplay;

/// Txn Geo Location (3D) filter
///
/// BBoxLatLonAlt will select only 3D transactions with altitude,
/// e.g. it will not select any 2D txn.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TxnFilterBBoxLatLonAlt {
    /// min latitude
    pub south: f64,
    /// min longitude
    pub west: f64,
    /// max depth
    pub depth: f64,
    /// max latitude
    pub north: f64,
    /// max longitude
    pub east: f64,
    /// max height
    pub height: f64,
}

impl IndentDisplay for TxnFilterBBoxLatLonAlt {
    fn i_fmt(&self, indent: &str, f: &mut Formatter<'_>) -> std::fmt::Result {
        let my_indent = format!("{indent}  ");
        writeln!(f, "{indent}Txn Bounding Box 3D")?;
        writeln!(
            f,
            "{my_indent}North, East, Height: geo:{},{},{}",
            self.north, self.east, self.height
        )?;
        writeln!(
            f,
            "{my_indent}South, West, Depth:  geo:{},{},{}",
            self.south, self.west, self.depth
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filters::{logic::TxnFilterAND, FilterDefinition, NullaryTRUE, TxnFilter};
    use indoc::indoc;
    use tackler_rs::IndocWithMarker;

    #[test]
    // test: c027ef27-3287-411f-aad9-8185f1b55380
    // desc: BBoxLatLonAlt, JSON
    fn txn_bbox_lat_lon_alt_json() {
        let filter_json_str = r#"{"txnFilter":{"TxnFilterBBoxLatLonAlt":{"south":-1.0,"west":-2.0,"depth":-3.0,"north":1.0,"east":2.0,"height":3.0}}}"#;

        let filter_text_str = indoc! {
        "|Filter:
         |  Txn Bounding Box 3D
         |    North, East, Height: geo:1,2,3
         |    South, West, Depth:  geo:-1,-2,-3
         |"}
        .strip_margin();

        let tf_res = serde_json::from_str::<FilterDefinition>(filter_json_str);
        assert!(tf_res.is_ok());
        let tf = tf_res.unwrap(/*:test:*/);

        match tf.txn_filter {
            TxnFilter::TxnFilterBBoxLatLonAlt(_) => assert!(true),
            _ => assert!(false),
        }

        assert_eq!(format!("{tf}"), filter_text_str);
        assert_eq!(
            serde_json::to_string(&tf).unwrap(/*:test:*/),
            filter_json_str
        );
    }

    #[test]
    // test: 54528f53-49fc-43cf-b3a2-221e02e87bcc
    // desc: BBoxLatLonAlt, Text
    fn txn_bbox_lat_lon_alt_text() {
        let filter_text_str = indoc! {
        "|Filter:
         |  AND
         |    Txn Bounding Box 3D
         |      North, East, Height: geo:1,2,3
         |      South, West, Depth:  geo:-1,-2,-3
         |    AND
         |      Txn Bounding Box 3D
         |        North, East, Height: geo:1,2,3
         |        South, West, Depth:  geo:-1,-2,-3
         |      All pass
         |"}
        .strip_margin();

        let tfd = FilterDefinition {
            txn_filter: TxnFilter::TxnFilterAND(TxnFilterAND {
                txn_filters: vec![
                    TxnFilter::TxnFilterBBoxLatLonAlt(TxnFilterBBoxLatLonAlt {
                        south: -1 as f64,
                        west: -2 as f64,
                        depth: -3 as f64,
                        north: 1 as f64,
                        east: 2 as f64,
                        height: 3 as f64,
                    }),
                    TxnFilter::TxnFilterAND(TxnFilterAND {
                        txn_filters: vec![
                            TxnFilter::TxnFilterBBoxLatLonAlt(TxnFilterBBoxLatLonAlt {
                                south: -1 as f64,
                                west: -2 as f64,
                                depth: -3 as f64,
                                north: 1 as f64,
                                east: 2 as f64,
                                height: 3 as f64,
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
