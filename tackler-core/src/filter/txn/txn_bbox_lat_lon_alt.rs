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
use tackler_api::filters::txn::TxnFilterBBoxLatLonAlt;

use crate::kernel::Predicate;

impl Predicate<Transaction> for TxnFilterBBoxLatLonAlt {
    fn eval(&self, txn: &Transaction) -> bool {
        txn.header.location.as_ref().map_or(false, |point| {
            let res2d = if self.west < self.east {
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
            };
            if res2d {
                match point.alt {
                    Some(z) => self.depth <= z && z <= self.height,
                    None => {
                        // 3d filter, but point has no altitude
                        false
                    }
                }
            } else {
                false
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filter::tests::make_geo_txn;
    use crate::model::Transaction;
    use tackler_api::filters::TxnFilter;

    #[test]
    // test: 607d4e0e-e05b-43cf-87b6-d3cad309be73
    // desc: Filter 3D Txns
    fn txn_bbox_lat_lon() {
        let tf = TxnFilterBBoxLatLonAlt {
            south: 40.0,
            west: 20.0,
            depth: -2000.0,
            north: 65.0,
            east: 26.0,
            height: 14000.0,
        };

        let cases: Vec<(Transaction, bool)> = vec![
            (make_geo_txn(60.0, 24.0, None), false),
            (make_geo_txn(60.0, 24.0, Some(-2001.0)), false),
            (make_geo_txn(60.0, 24.0, Some(-2000.0)), true),
            (make_geo_txn(60.0, 24.0, Some(0.0)), true),
            (make_geo_txn(60.0, 24.0, Some(1.0)), true),
            (make_geo_txn(60.0, 24.0, Some(14000.0)), true),
            (make_geo_txn(60.0, 24.0, Some(14001.0)), false),
        ];

        for t in cases.iter() {
            assert_eq!(tf.eval(&t.0), t.1);
        }

        // test: 5405a3cd-504f-4668-af57-563cbbe10298
        // desc: TxnFilter::TxnFilterBBoxLatLonAlt
        let filt = TxnFilter::TxnFilterBBoxLatLonAlt(tf);
        for t in cases {
            assert_eq!(filt.eval(&t.0), t.1);
        }
    }
}
