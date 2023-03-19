/*
 * Copyright 2023 E257.FI
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

use crate::kernel::report_item_selector::{RegisterItemSelector, ReportItemSelector};
use crate::model::{RegisterEntry, RegisterPosting, Txns};
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::io::Write;

pub(crate) fn register_engine<'a, W, T>(
    txns: &'a Txns,
    ras: &T,
    w: &mut Box<W>,
    reporter: fn(f: &mut Box<W>, &RegisterEntry),
) where
    W: Write + ?Sized,
    T: RegisterItemSelector<'a> + ReportItemSelector,
{
    let mut register_engine: HashMap<String, Decimal> = HashMap::new();
    txns.iter().for_each(|txn| {
        let register_postings: Vec<_> = txn
            .posts
            .iter()
            .map(|p| {
                let key = p.acctn.get_full();
                let mut_val = register_engine
                    .raw_entry_mut()
                    .from_key(&key)
                    .or_insert(key, Decimal::ZERO)
                    .1;
                let running_total = p.amount + *mut_val;
                *mut_val = running_total;

                RegisterPosting {
                    post: p,
                    amount: running_total,
                }
            })
            .collect();

        let mut filt_postings: Vec<_> = register_postings
            .iter()
            .cloned()
            .filter(|p| ras.predicate(&p))
            .collect();

        filt_postings.sort();

        let register_entry = RegisterEntry {
            txn,
            posts: filt_postings,
        };
        reporter(w, &register_entry);
    })
}
