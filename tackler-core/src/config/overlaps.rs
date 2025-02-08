/*
 * Tackler-NG 2025
 * SPDX-License-Identifier: Apache-2.0
 */

//! This module contains the overlap
//! configuration items to be used e.g. with CLI

use crate::config::PriceLookupType;
use std::path::PathBuf;

/// Collections of all configuration overlaps
pub struct OverlapConfig {
    pub audit: AuditOverlap,
    pub strict: StrictOverlap,
    pub price: PriceOverlap,
    pub report: ReportOverlap,
}

/// Audit mode related overlaps
pub struct AuditOverlap {
    pub mode: Option<bool>,
}

/// Strict mode related overlaps
pub struct StrictOverlap {
    pub mode: Option<bool>,
}

/// Price overlap configuration
#[derive(Debug, Clone)]
pub struct PriceOverlap {
    pub db_path: Option<PathBuf>,
    pub lookup_type: Option<PriceLookupType>,
    pub before_time: Option<String>,
}

/// Report overlap configuration
#[derive(Debug, Clone)]
pub struct ReportOverlap {
    pub commodity: Option<String>,
    pub account_overlap: Option<Vec<String>>,
    pub group_by: Option<String>,
}
