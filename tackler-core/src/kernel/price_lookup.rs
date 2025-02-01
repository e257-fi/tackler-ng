/*
 * Tackler-NG 2025
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::model::{
    price_entry::{PriceDb, PriceEntry},
    Commodity, Transaction, TxnAccount, TxnRefs,
};
use itertools::Itertools;
use jiff::tz::TimeZone;
use jiff::{Timestamp, Zoned};
use rust_decimal::Decimal;
use std::{
    collections::{BTreeSet, HashMap},
    sync::Arc,
};

#[derive(Debug)]
enum Cache<'p> {
    Fixed(HashMap<Arc<Commodity>, (Zoned, Decimal)>),
    Timed(HashMap<Arc<Commodity>, Vec<&'p PriceEntry>>),
}

#[derive(Debug)]
pub struct PriceLookupCtx<'p> {
    cache: Cache<'p>,
    in_commodity: Option<Arc<Commodity>>,
}

impl Default for PriceLookupCtx<'_> {
    fn default() -> Self {
        PriceLookupCtx {
            cache: Cache::Fixed(HashMap::new()),
            in_commodity: None,
        }
    }
}

impl PriceLookupCtx<'_> {
    #[inline]
    pub(crate) fn convert_prices<'r, 's, 't>(
        &'s self,
        txn: &'t Transaction,
    ) -> Box<dyn Iterator<Item = (TxnAccount, Decimal, Option<Decimal>)> + 'r>
    where
        's: 'r,
        't: 'r,
    {
        match &self.in_commodity {
            Some(comm) => Box::new(self.convert_prices_inner(txn, comm.clone())),
            None => Box::new(txn.posts.iter().map(|p| (p.acctn.clone(), p.amount, None))),
        }
    }

    fn convert_prices_inner<'r, 's, 't>(
        &'s self,
        txn: &'t Transaction,
        in_commodity: Arc<Commodity>,
    ) -> Box<dyn Iterator<Item = (TxnAccount, Decimal, Option<Decimal>)> + 'r>
    where
        's: 'r,
        't: 'r,
    {
        Box::new(txn.posts.iter().map(move |p| {
            if p.acctn.comm.is_any() {
                let mut acctn = p.acctn.clone();
                let mut amount = p.amount;
                match &self.cache {
                    Cache::Fixed(cache) => {
                        if let Some(c) = cache.get(&p.acctn.comm) {
                            acctn.comm = in_commodity.clone();
                            amount *= c.1;
                        }
                        (acctn, amount, None)
                    }
                    Cache::Timed(comm_cache) => {
                        let cache = comm_cache
                            .get(&p.acctn.comm)
                            .expect("IE: cache logic error");
                        let i = match cache
                            .binary_search_by_key(&(&txn.header.timestamp, &p.acctn.comm), |e| {
                                (&e.timestamp, &e.base_commodity)
                            }) {
                            Ok(i) => Some(i),
                            Err(i) => i.checked_sub(1),
                        };
                        let rate = if let Some(i) = i {
                            acctn.comm = in_commodity.clone();
                            amount *= cache[i].eq_amount;
                            Some(cache[i].eq_amount)
                        } else {
                            None
                        };
                        (acctn, amount, rate)
                    }
                }
            } else {
                (p.acctn.clone(), p.amount, None)
            }
        }))
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum PriceLookup {
    AtTheTimeOfTxn,
    #[default]
    LastPriceDbEntry,
    GivenTime(Zoned),
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
            Some(lookup_timestamp) => Cache::Fixed(
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
            None => {
                let mut cache = HashMap::new();
                for comm in used_commodities {
                    let comm_cache = price_db
                        .iter()
                        .filter(|e| comm == e.base_commodity && e.eq_commodity == in_commodity)
                        .sorted_by_key(|e| &e.timestamp) // make sure it's sorted
                        .collect();
                    cache.insert(comm, comm_cache);
                }
                Cache::Timed(cache)
            }
        };

        PriceLookupCtx {
            cache,
            in_commodity: Some(in_commodity),
        }
    }
}
