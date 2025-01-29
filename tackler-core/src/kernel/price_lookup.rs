/*
 * Tackler-NG 2025
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::model::{
    price_entry::{PriceDb, PriceEntry, PriceLookup},
    Commodity, Transaction, TxnAccount, TxnRefs,
};
use jiff::tz::TimeZone;
use jiff::{Timestamp, Zoned};
use rust_decimal::Decimal;
use std::{
    collections::{BTreeSet, HashMap},
    sync::Arc,
};

#[derive(Debug)]
enum Cache<'p> {
    Untimed(HashMap<Arc<Commodity>, (Zoned, Decimal)>),
    Timed(Vec<&'p PriceEntry>),
}

#[derive(Debug)]
pub struct PriceLookupCtx<'p> {
    cache: Cache<'p>,
    in_commodity: Option<Arc<Commodity>>,
}

impl Default for PriceLookupCtx<'_> {
    fn default() -> Self {
        PriceLookupCtx {
            cache: Cache::Untimed(HashMap::new()),
            in_commodity: None,
        }
    }
}

impl<'p> PriceLookupCtx<'p> {
    pub(crate) fn convert_prices<'r, 's, 't>(
        &'s self,
        txn: &'t Transaction,
    ) -> impl Iterator<Item = (TxnAccount, Decimal)> + 'r
    where
        'p: 'r,
        's: 'r,
        't: 'r,
    {
        txn.posts.iter().map(|p| {
            let mut acctn = p.acctn.clone();
            let mut amount = p.amount;
            if let Some(in_commodity) = self.in_commodity.clone() {
                match &self.cache {
                    Cache::Untimed(cache) => {
                        if let Some(c) = cache.get(&p.acctn.comm) {
                            acctn.comm = in_commodity;
                            amount *= c.1;
                        }
                    }
                    Cache::Timed(cache) => {
                        let i = match cache
                            .binary_search_by_key(&(&txn.header.timestamp, &p.acctn.comm), |e| {
                                (&e.timestamp, &e.base_commodity)
                            }) {
                            Ok(i) => Some(i),
                            Err(i) => i.checked_sub(1),
                        };
                        if let Some(i) = i {
                            acctn.comm = in_commodity;
                            amount *= cache[i].eq_amount;
                        }
                    }
                }
            }
            (acctn, amount)
        })
    }
}

impl PriceLookup {
    pub(crate) fn make_ctx<'p>(
        &self,
        txns: &TxnRefs<'_>,
        in_commodity: Option<Arc<Commodity>>,
        price_db: &'p PriceDb,
    ) -> PriceLookupCtx<'p> {
        let Some(in_commodity) = in_commodity else {
            // No commodity conversion, short-circuit out
            return PriceLookupCtx::default();
        };

        let used_commodities = txns
            .iter()
            .flat_map(|t| &t.posts)
            // This must be acctn.comm as txn_commodity is commodity for whole txn
            .map(|p| p.acctn.comm.clone())
            .collect::<BTreeSet<_>>();

        let lookup_timestamp = match self {
            PriceLookup::AtTheTimeOfTxn => None,
            PriceLookup::LastPriceDbEntry => Some(Timestamp::MAX.to_zoned(TimeZone::UTC)),
            PriceLookup::GivenTime(t) => Some(t.clone()),
        };

        let cache = match lookup_timestamp {
            Some(lookup_timestamp) => Cache::Untimed(
                price_db
                    .iter()
                    .filter(|e| {
                        used_commodities.contains(&e.base_commodity)
                            && e.eq_commodity == in_commodity
                            && e.timestamp < lookup_timestamp
                    })
                    .map(|e| (e.base_commodity.clone(), (e.timestamp.clone(), e.eq_amount)))
                    .collect(),
            ),
            None if self != &PriceLookup::AtTheTimeOfTxn => Cache::Timed(Vec::new()),
            None => Cache::Timed(
                price_db
                    .iter()
                    .filter(|e| {
                        used_commodities.contains(&e.base_commodity)
                            && e.eq_commodity == in_commodity
                    })
                    .collect(),
            ),
        };

        PriceLookupCtx {
            cache,
            in_commodity: Some(in_commodity),
        }
    }
}
