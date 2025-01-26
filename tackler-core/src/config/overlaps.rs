/*
 * Tackler-NG 2025
 * SPDX-License-Identifier: Apache-2.0
 */
use crate::config::PriceLookupType;
use jiff::Zoned;
use std::path::PathBuf;

/// This module contains the overlap
/// configuration items to be used e.g. with CLI

#[derive(Debug, Clone)]
pub struct PriceOverlap {
    pub db_path: Option<PathBuf>,
    pub lookup_type: Option<PriceLookupType>,
    pub before_time: Option<Zoned>,
}
