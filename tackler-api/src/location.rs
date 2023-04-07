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

//! Transaction Geo location
//!
use std::error::Error;
use std::fmt::{Display, Formatter};

/// Geo Point
///
#[derive(Debug, Clone)]
pub struct GeoPoint {
    /// Latitude in decimal format
    pub lat: f64,
    /// Longitude in decimal format
    pub lon: f64,
    /// optional depth/altitude, in meters
    pub alt: Option<f64>,
}

impl Display for GeoPoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        //"geo:" + GeoPoint.frmt(lat) + "," + GeoPoint.frmt(lon) + alt.map("," + GeoPoint.frmt(_)).getOrElse("")
        // todo: check the scale behaviour of Decimal
        let alt = match &self.alt {
            Some(a) => format!(",{a}"),
            None => String::new(),
        };
        write!(f, "geo:{},{}{}", self.lat, self.lon, alt)
    }
}

#[allow(clippy::manual_range_contains)]
impl GeoPoint {
    /// Make Geo point from given coordinates.
    ///
    /// * `lat` in decimals, must be inclusive -90 -- 90
    /// * `lon` in decimals, must be inclusive -180 -- 180
    /// * `alt` in meters, must be more than -6378137 meters
    pub fn from(lat: f64, lon: f64, alt: Option<f64>) -> Result<GeoPoint, Box<dyn Error>> {
        if lat < -90.0 || 90.0 < lat {
            let msg = format!("Value out of specification for Latitude: {lat}");
            return Err(msg.into());
        }
        if lon < -180.0 || 180.0 < lon {
            let msg = format!("Value out of specification for Longitude: {lon}");
            return Err(msg.into());
        }
        if let Some(z) = alt.as_ref() {
            if z < &-6_378_137.0 {
                // Jules Verne: Voyage au centre de la Terre
                let msg = format!("Value Out of specification for Altitude: {z}");
                return Err(msg.into());
            }
        }
        Ok(GeoPoint { lat, lon, alt })
    }
}

// todo: GeoPoint::from + checks
