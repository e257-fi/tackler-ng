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
use crate::model::{RegisterEntry, RegisterPosting, Transaction, TxnRefs, TxnSet};
use itertools::Itertools;
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::error::Error;
use std::io;

pub(crate) type RegisterReporterFn<W> =
    fn(writer: &mut W, &RegisterEntry) -> Result<(), Box<dyn Error>>;

pub(crate) type TxnGroupByOp<'a> = Box<dyn Fn(&Transaction) -> String + 'a>;

pub(crate) fn balance_groups<T>(
    txns: &TxnRefs,
    group_by_op: TxnGroupByOp,
    ras: &T,
    settings: &mut Settings,
) -> Vec<Balance>
where
    T: BalanceSelector + ?Sized,
{
    txns.iter()
        .cloned() // this is originally &&Transaction
        .chunk_by(|txn| group_by_op(txn))
        .into_iter()
        // .par // todo: par-map
        .map(|(group_by_key, bal_grp_txns)| {
            // todo: could this be an iterator?
            let txns = bal_grp_txns.collect();
            // This is a single balance inside balance group,
            // so there shouldn't be any audit or txn-set-checksum for this sub-group (bal) of txns
            let metadata = None;
            let txn_set = TxnSet { metadata, txns };

            Balance::from(&group_by_key, &txn_set, ras, settings)
        })
        .filter(|bal| !bal.is_empty())
        .sorted_by_key(|bal| bal.title.clone()) // todo: could this clone be avoided?
        .collect()
}

pub(crate) fn register_engine<'a, W, T>(
    txns: &'a TxnRefs,
    ras: &T,
    w: &mut W,
    reporter: RegisterReporterFn<W>,
) -> Result<(), Box<dyn Error>>
where
    W: io::Write + ?Sized,
    T: RegisterSelector<'a> + ?Sized,
{
    let mut register_engine: HashMap<String, Decimal> = HashMap::new();
    for txn in txns {
        let register_postings: Vec<_> = txn
            .posts
            .iter()
            .map(|p| {
                let key = p.acctn.get_full();
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
            .iter()
            .filter(|p| ras.eval(p))
            .cloned()
            .collect();

        filt_postings.sort();

        let register_entry = RegisterEntry {
            txn,
            posts: filt_postings,
        };
        reporter(w, &register_entry)?;
    }
    Ok(())
}
