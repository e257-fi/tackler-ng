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

//! Txn Set and Report metadata
//!
pub mod items;

use items::MetadataItem;
use items::MetadataItems;
use items::Text;

/// Metadata of Inputs, Txn Set, Reporting parameters, etc.
///
#[derive(Debug, Clone, Default)]
pub struct Metadata {
    // todo: fix pub access
    #[doc(hidden)]
    pub items: MetadataItems,
}

impl Metadata {
    /// Get new empty `metadata`
    #[must_use]
    pub fn new() -> Metadata {
        Metadata { items: Vec::new() }
    }

    /// Get new metadata with existing Metadata item
    #[must_use]
    pub fn from_mdi(mdi: MetadataItem) -> Metadata {
        let items = vec![mdi];

        Metadata { items }
    }

    /// Get new metadata from existing Metadata.
    ///
    /// If there is an existing [`TxnSetChecksum`](items::TxnSetChecksum) metadata item,
    /// it will be removed from the new set.
    #[must_use]
    pub fn from_metadata(md: &Metadata) -> Metadata {
        let mut metadata = Metadata::new();
        for mdi in &md.items {
            match mdi {
                MetadataItem::TxnSetChecksum(_) => (),
                _ => metadata.push(mdi.clone()),
            }
        }
        metadata
    }

    /// Add metadata item into metadata
    pub fn push(&mut self, mdi: MetadataItem) {
        self.items.push(mdi);
    }

    /// Get textual representation of Metadata
    #[must_use]
    pub fn text(&self) -> String {
        let ts = self
            .items
            .iter()
            .flat_map(|item| {
                let mut vs = item.text();
                // put a newline between metadata items
                vs.push(String::default());
                vs
            })
            .collect::<Vec<String>>();
        ts.join("\n")
    }
}

/// Generic checksum value
#[derive(Debug, Clone)]
pub struct Checksum {
    /// used hash algorithm
    pub algorithm: String,
    /// hexadecimal hash value
    pub value: String,
}
