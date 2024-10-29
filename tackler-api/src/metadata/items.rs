/*
 * Copyright 2022-2024 E257.FI
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

//! This module contains various Metadata items
//!
use crate::filters::FilterDefinition;
use crate::metadata::Checksum;

#[doc(hidden)]
pub type MetadataItems = Vec<MetadataItem>;

#[doc(hidden)]
pub trait Text: std::fmt::Debug {
    /// Get metadata item as text
    #[must_use]
    fn text(&self) -> Vec<String>;
}

#[doc(hidden)]
#[derive(Debug, Clone)]
pub enum MetadataItem {
    #[doc(hidden)]
    TxnSetChecksum(TxnSetChecksum),
    #[doc(hidden)]
    AccountSelectorChecksum(AccountSelectorChecksum),
    #[doc(hidden)]
    GitInputReference(GitInputReference),
    #[doc(hidden)]
    TxnFilterDescription(TxnFilterDescription),
}

impl Text for MetadataItem {
    fn text(&self) -> Vec<String> {
        match self {
            Self::GitInputReference(gif) => gif.text(),
            Self::TxnSetChecksum(tscs) => tscs.text(),
            Self::AccountSelectorChecksum(asc) => asc.text(),
            Self::TxnFilterDescription(tfd) => tfd.text(),
        }
    }
}

/// Txn Set Checksum metadata item
#[derive(Debug, Clone)]
pub struct TxnSetChecksum {
    /// size of transaction set
    pub size: usize,
    /// hash of Txn Set Checksum
    pub hash: Checksum,
}
impl Text for TxnSetChecksum {
    fn text(&self) -> Vec<String> {
        // echo -n "SHA-512/256" | wc -c => 11
        let pad = 15;
        vec![
            format!("Txn Set Checksum"),
            format!("{:>pad$} : {}", self.hash.algorithm, &self.hash.value),
            format!("{:>pad$} : {}", "Set size", self.size),
        ]
    }
}

/// Account Selector Checksum item
#[derive(Debug, Clone)]
pub struct AccountSelectorChecksum {
    /// Hash of selector Checksum
    pub hash: Checksum,
}
impl Text for AccountSelectorChecksum {
    fn text(&self) -> Vec<String> {
        // echo -n "SHA-512/256" | wc -c => 11
        let pad = 15;
        vec![
            format!("Account Selector Checksum"),
            format!("{:>pad$} : {}", self.hash.algorithm, &self.hash.value),
        ]
    }
}

/// Metadata information about active Txn Filters
///
#[derive(Debug, Clone)]
pub struct TxnFilterDescription {
    #[doc(hidden)]
    txn_filter_def: FilterDefinition,
}

impl TxnFilterDescription {
    /// Make Txn filter Description from Filter Definition
    ///
    #[must_use]
    pub fn from(tf: FilterDefinition) -> TxnFilterDescription {
        TxnFilterDescription { txn_filter_def: tf }
    }
}
impl Text for TxnFilterDescription {
    fn text(&self) -> Vec<String> {
        vec![String::from(format!("{}", self.txn_filter_def).trim_end())]
    }
}

/// Metadata information about Git Txn input
///
#[derive(Debug, Clone)]
pub struct GitInputReference {
    /// commit id
    pub commit: String,
    /// git symbolic reference `main`, `Y2023`, etc.
    pub reference: Option<String>,
    /// Git directory inside repository
    pub dir: String,
    /// filename suffix of journal files
    pub suffix: String,
    /// Git commit message of selected commit
    pub message: String,
}

impl Text for GitInputReference {
    /*
       Seq(
         "Git storage:",
         "  commit:  " + commit,
         "  ref:     " + ref.getOrElse("FIXED by commit"),
         "  dir:     " + dir,
         "  suffix:  " + suffix,
         "  message: " + message,
       )
    */
    fn text(&self) -> Vec<String> {
        let pad = 15;
        vec![
            format!("Git Storage"),
            format!("{:>pad$} : {}", "commit", self.commit),
            format!(
                "{:>pad$} : {}",
                "reference",
                self.reference
                    .as_ref()
                    .unwrap_or(&"FIXED by commit - no ref!".to_string())
            ),
            format!("{:>pad$} : {}", "directory", self.dir),
            format!("{:>pad$} : .{}", "suffix", self.suffix),
            format!("{:>pad$} : {}", "message", self.message.trim()),
        ]
    }
}
