/*
 * Copyright 2022-2023 E257.FI
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
#![forbid(unsafe_code)]

//! Tackler API components

pub use crate::txn_header::TxnHeader;

pub use crate::geo_point::GeoPoint;
pub use crate::metadata::Checksum;
pub use crate::metadata::GitInputReference;
pub use crate::metadata::Metadata;
pub use crate::metadata::MetadataItem;
pub use crate::metadata::TxnSetChecksum;

mod metadata;

pub type Tags = Vec<String>;
pub type Tag = String;

pub mod filters;
mod geo_point;
mod txn_header;
pub mod txn_ts;

#[cfg(test)]
mod tests {
    // todo: fixt this into common place with tackler-core

    pub(crate) trait IndocWithMarker {
        fn strip_margin(&self) -> String;
    }

    impl IndocWithMarker for str {
        fn strip_margin(&self) -> String {
            match self.strip_prefix('|') {
                Some(s) => s.to_string().replace("\n|", "\n"),
                None => self.replace("\n|", "\n"),
            }
        }
    }
}
