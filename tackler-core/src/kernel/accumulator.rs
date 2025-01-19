/*
 * Tackler-NG 2023-2024
 *
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::kernel::Settings;
use crate::model::{
    price_entry::PriceLookup, RegisterEntry, RegisterPosting, Transaction, TxnAccount, TxnRefs,
};
use crate::report::RegisterSettings;
use crate::{kernel::balance::Balance, model::Commodity};
use crate::{
    kernel::report_item_selector::{BalanceSelector, RegisterSelector},
    model::price_entry::PriceDb,
};
use itertools::Itertools;
use rust_decimal::Decimal;
use std::error::Error;
use std::io;
use std::{collections::HashMap, sync::Arc};

pub(crate) type RegisterReporterFn<W> =
    fn(writer: &mut W, &RegisterEntry<'_>, &RegisterSettings) -> Result<(), Box<dyn Error>>;

pub(crate) type TxnGroupByOp<'a> = Box<dyn Fn(&Transaction) -> String + 'a>;

pub(crate) fn balance_groups<T>(
    txns: &TxnRefs<'_>,
    group_by_op: TxnGroupByOp<'_>,
    price_db: &PriceDb,
    report_commodity: Option<Arc<Commodity>>,
    price_lookup: &PriceLookup,
    ras: &T,
    settings: &Settings,
) -> Vec<Balance>
where
    T: BalanceSelector + ?Sized,
{
    txns.iter()
        .copied()
        .chunk_by(|txn| group_by_op(txn))
        .into_iter()
        // .par // todo: par-map
        .map(|(group_by_key, bal_grp_txns)| {
            Balance::from_iter(
                &group_by_key,
                report_commodity.clone(),
                price_lookup,
                bal_grp_txns,
                price_db,
                ras,
                settings,
            )
            .expect("Logic error with Balance Group: inner balance failed")
        })
        .filter(|bal| !bal.is_empty())
        .sorted_by_key(|bal| bal.title.clone())
        .collect()
}

pub(crate) fn register_engine<'a, W, T>(
    txns: &'a TxnRefs<'_>,
    price_db: &PriceDb,
    ras: &T,
    w: &mut W,
    reporter: RegisterReporterFn<W>,
    register_settings: &RegisterSettings,
) -> Result<(), Box<dyn Error>>
where
    W: io::Write + ?Sized,
    T: RegisterSelector<'a> + ?Sized,
{
    let report_commodity = register_settings.report_commodity.clone();
    let txn_commodities = txns
        .iter()
        .flat_map(|t| &t.posts)
        .map(|p| p.txn_commodity.clone())
        .collect();

    let lookup_ctx = register_settings.price_lookup.make_ctx(
        report_commodity,
        txn_commodities,
        price_db,
        txns.last().map(|v| &**v),
    );

    let mut register_engine: HashMap<TxnAccount, Decimal> = HashMap::new();
    for txn in txns {
        let register_postings: Vec<_> = lookup_ctx
            .convert_prices(txn)
            .zip(&txn.posts)
            .map(|((account, amount), p)| {
                let running_total = *register_engine
                    .entry(account)
                    .and_modify(|v| {
                        *v += amount;
                    })
                    .or_insert(amount);

                RegisterPosting {
                    post: p,
                    amount: running_total,
                }
            })
            .collect();

        let mut filt_postings: Vec<_> = register_postings
            .into_iter()
            .filter(|p| ras.eval(p))
            .collect();

        filt_postings.sort();

        let register_entry = RegisterEntry {
            txn,
            posts: filt_postings,
        };
        reporter(w, &register_entry, register_settings)?;
    }
    Ok(())
}
