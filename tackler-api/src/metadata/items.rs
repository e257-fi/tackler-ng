/*
 * Tackler-NG 2022-2024
 *
 * SPDX-License-Identifier: Apache-2.0
 */

//! This module contains various Metadata items
//!

use crate::filters::{FilterDefZoned, FilterDefinition};
use crate::metadata::Checksum;
use jiff::tz::TimeZone;

#[doc(hidden)]
pub type MetadataItems = Vec<MetadataItem>;

#[doc(hidden)]
pub trait Text: std::fmt::Debug {
    /// Get metadata item as text
    #[must_use]
    fn text(&self, tz: TimeZone) -> Vec<String>;
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

impl MetadataItem {
    pub const ITEM_PAD: usize = 15;
}

impl Text for MetadataItem {
    fn text(&self, tz: TimeZone) -> Vec<String> {
        match self {
            Self::GitInputReference(gif) => gif.text(tz),
            Self::TxnSetChecksum(tscs) => tscs.text(tz),
            Self::AccountSelectorChecksum(asc) => asc.text(tz),
            Self::TxnFilterDescription(tfd) => tfd.text(tz),
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
    fn text(&self, _tz: TimeZone) -> Vec<String> {
        // echo -n "SHA-512/256" | wc -c => 11
        let pad = MetadataItem::ITEM_PAD;
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
    fn text(&self, _tz: TimeZone) -> Vec<String> {
        // echo -n "SHA-512/256" | wc -c => 11
        let pad = MetadataItem::ITEM_PAD;
        vec![
            format!("Account Selector Checksum"),
            format!("{:>pad$} : {}", self.hash.algorithm, &self.hash.value),
        ]
    }
}

/// Report timezone item
#[derive(Debug, Clone)]
pub struct ReportTimezone {
    /// Timezone name
    pub timezone: String,
}
impl Text for ReportTimezone {
    fn text(&self, _tz: TimeZone) -> Vec<String> {
        let pad = MetadataItem::ITEM_PAD;
        vec![
            "Report Time Zone".to_string(),
            format!("{:>pad$} : {}", "TZ name", &self.timezone),
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
    fn text(&self, tz: TimeZone) -> Vec<String> {
        // todo: TxnFilterDescription needs proper implementation for Text
        //       See equity_exporter::write_export
        format!(
            "{}",
            FilterDefZoned {
                filt_def: &self.txn_filter_def,
                tz
            }
        )
        .trim_end()
        .split("\n")
        .map(String::from)
        .collect::<Vec<String>>()
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
    fn text(&self, _tz: TimeZone) -> Vec<String> {
        let pad = MetadataItem::ITEM_PAD;
        vec![
            format!("Git Storage"),
            format!("{:>pad$} : {}", "commit", self.commit),
            format!(
                "{:>pad$} : {}",
                "reference",
                self.reference
                    .as_ref()
                    .unwrap_or(&"FIXED by commit".to_string())
            ),
            format!("{:>pad$} : {}", "directory", self.dir),
            format!("{:>pad$} : .{}", "suffix", self.suffix),
            format!("{:>pad$} : {}", "message", self.message.trim()),
        ]
    }
}
