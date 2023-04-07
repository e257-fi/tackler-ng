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

use crate::kernel::report_item_selector::RegisterSelector;
use crate::model::{RegisterEntry, RegisterPosting, TxnRefs};
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::error::Error;
use std::io;

pub(crate) type RegisterReporterFn<W> =
    fn(writer: &mut W, &RegisterEntry) -> Result<(), Box<dyn Error>>;

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
            .filter(|p| ras.eval(p))
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
