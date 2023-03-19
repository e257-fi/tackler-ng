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

use crate::model::Transaction;
use tackler_api::filters::TxnFilterBBoxLatLon;

use super::FilterTxn;

impl FilterTxn for TxnFilterBBoxLatLon {
    fn filter(&self, txn: &Transaction) -> bool {
        txn.header.location.as_ref().map_or(false, |point| {
            if self.west < self.east {
                self.south <= point.lat
                    && point.lat <= self.north
                    && self.west <= point.lon
                    && point.lon <= self.east
            } else {
                // (west > east) => BBox is over 180th meridian (over antimeridian):
                // 1.1 The left (west) hand side of BBox is actually the longitude of East (+deg)
                // 1.2 The right (east) hand side of BBox is actually the longitude of West (-deg)
                // 2. (+deg) <----- meri_|_dian ----> (-deg)
                // 3. Valid points are
                // 3.1   from left (west) hand to the meridian (180deg)
                // 3.2   from right (east) hand to the meridian (-180deg)
                // 3.3   This is true also if both edges (left and rights) are on
                //       the same sign of Longitude (e.g. box is super slide (>180deg))
                self.south <= point.lat
                    && point.lat <= self.north
                    && (self.west <= point.lon || point.lon <= self.east)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filter::tests::make_geo_txn;
    use crate::model::Transaction;
    use tackler_api::filters::{TxnFilter, TxnFilterBBoxLatLon};

    #[test]
    // test: 320d92b9-f8e7-4283-9296-74ff4340fff1
    // desc: Filter 2D Txns
    fn txn_bbox_lat_lon() {
        let tf = TxnFilterBBoxLatLon {
            south: 40.0,
            west: 20.0,
            north: 65.0,
            east: 26.0,
        };

        let cases: Vec<(Transaction, bool)> = vec![
            (make_geo_txn(0.0, 0.0, None), false),
            (make_geo_txn(60.170833, 24.9375, None), true),
            (make_geo_txn(39.0, 23.0, None), false),
            (make_geo_txn(66.0, 23.0, None), false),
            (make_geo_txn(50.0, 19.0, None), false),
            (make_geo_txn(50.0, 27.0, None), false),
        ];

        for t in cases.iter() {
            assert_eq!(tf.filter(&t.0), t.1);
        }

        // test: 78412bff-ef54-41e5-aed8-c1bf6965a4e6
        // desc: TxnFilter::TxnFilterBBoxLatLon
        let filt = TxnFilter::TxnFilterBBoxLatLon(tf);
        for t in cases {
            assert_eq!(filt.filter(&t.0), t.1);
        }
    }
}
