/*
 * Copyright 2023-2024 E257.FI
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

use crate::kernel::balance::Balance;
use crate::kernel::report_item_selector::{BalanceSelector, RegisterSelector};
use crate::kernel::Settings;
use crate::model::{RegisterEntry, RegisterPosting, Transaction, TxnAccount, TxnRefs};
use crate::report::RegisterSettings;
use itertools::Itertools;
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::error::Error;
use std::io;
use tackler_api::txn_ts::TimestampStyle;
use time_tz::Tz;

pub(crate) type RegisterReporterFn<W> = fn(
    writer: &mut W,
    &RegisterEntry<'_>,
    TimestampStyle,
    &'static Tz,
    &RegisterSettings,
) -> Result<(), Box<dyn Error>>;

pub(crate) type TxnGroupByOp<'a> = Box<dyn Fn(&Transaction) -> String + 'a>;

pub(crate) fn balance_groups<T>(
    txns: &TxnRefs<'_>,
    group_by_op: TxnGroupByOp<'_>,
    ras: &T,
    settings: &Settings,
) -> Vec<Balance>
where
    T: BalanceSelector + ?Sized,
{
    txns.iter()
        .chunk_by(|txn| group_by_op(txn))
        .into_iter()
        // .par // todo: par-map
        .map(|(group_by_key, bal_grp_txns)| {
            Balance::from_iter(&group_by_key, bal_grp_txns, ras, settings)
                .expect("Logic error with Balance Group: inner balance failed")
        })
        .filter(|bal| !bal.is_empty())
        .sorted_by_key(|bal| bal.title.clone())
        .collect()
}

pub(crate) fn register_engine<'a, W, T>(
    txns: &'a TxnRefs<'_>,
    ras: &T,
    ts_style: TimestampStyle,
    report_tz: &'static Tz,
    w: &mut W,
    reporter: RegisterReporterFn<W>,
    register_settings: &RegisterSettings,
) -> Result<(), Box<dyn Error>>
where
    W: io::Write + ?Sized,
    T: RegisterSelector<'a> + ?Sized,
{
    let mut register_engine: HashMap<&TxnAccount, Decimal> = HashMap::new();
    for txn in txns {
        let register_postings: Vec<_> = txn
            .posts
            .iter()
            .map(|p| {
                let key = &p.acctn;
                let running_total = *register_engine
                    .entry(key)
                    .and_modify(|v| {
                        *v += p.amount;
                    })
                    .or_insert(p.amount);

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
        reporter(w, &register_entry, ts_style, report_tz, register_settings)?;
    }
    Ok(())
}
