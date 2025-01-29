/*
 * Tackler-NG 2025
 * SPDX-License-Identifier: Apache-2.0
 */

use std::sync::Arc;

use jiff::Zoned;
use rust_decimal::Decimal;

use super::Commodity;

/// Entry in the price database
#[derive(Debug, Eq)]
pub struct PriceEntry {
    /// Timestamp with Zone information
    pub timestamp: jiff::Zoned,
    /// The commodity for which price is being noted
    pub base_commodity: Arc<Commodity>,
    /// Price of base in _eq_ commodity
    pub eq_amount: Decimal,
    /// The equivalence commodity in which price is being noted
    pub eq_commodity: Arc<Commodity>,
    /// Comments
    pub comments: Option<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum PriceLookup {
    AtTheTimeOfTxn,
    #[default]
    LastPriceDbEntry,
    GivenTime(Zoned),
}

impl Ord for PriceEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.timestamp
            .cmp(&other.timestamp)
            .then_with(|| self.base_commodity.cmp(&other.base_commodity))
            .then_with(|| self.eq_commodity.cmp(&other.eq_commodity))
    }
}

impl PartialEq for PriceEntry {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp
            && self.base_commodity == other.base_commodity
            && self.eq_commodity == other.eq_commodity
    }
}

impl PartialOrd for PriceEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub type PriceDb = Vec<PriceEntry>;
