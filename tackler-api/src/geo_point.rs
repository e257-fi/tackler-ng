/*
 * Copyright 2022 E257.FI
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

use rust_decimal::Decimal;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct GeoPoint {
    pub lat: Decimal,
    pub lon: Decimal,
    pub alt: Option<Decimal>,
}

impl Display for GeoPoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        //"geo:" + GeoPoint.frmt(lat) + "," + GeoPoint.frmt(lon) + alt.map("," + GeoPoint.frmt(_)).getOrElse("")
        // todo: check the scale behaviour of Decimal
        let alt = match &self.alt {
            Some(a) => format!(",{a}"),
            None => "".to_string(),
        };
        write!(f, "geo:{},{}{}", self.lat, self.lon, alt)
    }
}

impl GeoPoint {
    pub fn from(
        lat: Decimal,
        lon: Decimal,
        alt: Option<Decimal>,
    ) -> Result<GeoPoint, Box<dyn Error>> {
        if lat < Decimal::from(-90) || Decimal::from(90) < lat {
            let msg = format!("Value out of specification for Latitude: {lat}");
            return Err(msg.into());
        }
        if lon < Decimal::from(-180) || Decimal::from(180) < lon {
            let msg = format!("Value out of specification for Longitude: {lon}");
            return Err(msg.into());
        }
        if let Some(z) = alt.as_ref() {
            if z < &Decimal::from(-6378137) {
                // Jules Verne: Voyage au centre de la Terre
                let msg = format!("Value Out of specification for Altitude: {z}");
                return Err(msg.into());
            }
        }
        Ok(GeoPoint { lat, lon, alt })
    }
}

// todo: GeoPoint::from + checks
