use std::{
    collections::{BTreeSet, HashMap},
    sync::Arc,
};

use rust_decimal::Decimal;

use crate::model::{
    price_entry::{PriceDb, PriceEntry, PriceLookup},
    Commodity, Transaction, TxnAccount,
};

#[derive(Debug)]
enum Cache<'p> {
    Untimed(HashMap<Arc<Commodity>, Decimal>),
    Timed(Vec<&'p PriceEntry>),
}

pub(crate) struct LookupCtx<'p> {
    cache: Cache<'p>,
    in_commodity: Option<Arc<Commodity>>,
}

impl<'p> LookupCtx<'p> {
    pub(crate) fn convert_prices<'r, 's, 't>(
        &'s self,
        txn: &'t Transaction,
    ) -> impl Iterator<Item = (TxnAccount, Decimal)> + 'r
    where
        'p: 'r,
        's: 'r,
        't: 'r,
    {
        txn.posts.iter().map(move |p| {
            let mut acctn = p.acctn.clone();
            let mut amount = p.amount;
            if let Some(in_commodity) = self.in_commodity.clone() {
                match &self.cache {
                    Cache::Untimed(cache) => {
                        if let Some(c) = cache.get(&p.acctn.comm) {
                            acctn.comm = in_commodity;
                            amount *= c;
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
        in_commodity: Option<Arc<Commodity>>,
        used_commodities: BTreeSet<Arc<Commodity>>,
        price_db: &'p PriceDb,
        last_txn: Option<&Transaction>,
    ) -> LookupCtx<'p> {
        let Some(in_commodity) = in_commodity else {
            return LookupCtx {
                cache: Cache::Timed(Vec::new()),
                in_commodity: None,
            };
        };

        let last_price_db_time = price_db.last().map(|e| e.timestamp.clone());
        let lookup_timestamp = match self {
            PriceLookup::AtTheTimeOfTxn => None,
            PriceLookup::AtTheTimeOfLastTxn => last_txn
                .map(|t| t.header.timestamp.clone())
                .or_else(|| last_price_db_time.clone()),
            PriceLookup::AtTheTimeOfTxnTsEndFilter => last_txn
                .map(|t| t.header.timestamp.clone())
                .or_else(|| last_price_db_time.clone()),
            PriceLookup::LastPriceDbEntry => last_price_db_time,
            PriceLookup::GivenTime(t) => Some(t.clone()),
        };

        let cache = match lookup_timestamp {
            Some(lookup_timestamp) => Cache::Untimed(
                price_db
                    .iter()
                    .filter(|e| {
                        used_commodities.contains(&e.base_commodity)
                            && e.eq_commodity == in_commodity
                            && e.timestamp <= lookup_timestamp
                    })
                    .map(|e| (e.base_commodity.clone(), e.eq_amount))
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

        LookupCtx {
            cache,
            in_commodity: Some(in_commodity),
        }
    }
}
