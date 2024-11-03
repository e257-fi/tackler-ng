/*
 * Copyright 2024 E257.FI
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

use crate::export::Export;
use crate::kernel::balance::Balance;
use crate::kernel::report_item_selector::{
    BalanceNonZeroByAccountSelector, BalanceNonZeroSelector, BalanceSelector,
};
use crate::kernel::Settings;
use crate::model::{Transaction, TxnSet};
use crate::report::get_account_selector_checksum;
use itertools::Itertools;
use rust_decimal::Decimal;
use std::error::Error;
use std::io;
use tackler_api::metadata::items::Text;
use time::format_description::well_known::Rfc3339;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct EquitySettings<'a> {
    pub eqa: Option<String>,
    pub ras: &'a Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct EquityExporter<'a> {
    pub export_settings: EquitySettings<'a>,
}

impl EquityExporter<'_> {
    fn get_acc_selector(&self) -> Result<Box<dyn BalanceSelector>, Box<dyn Error>> {
        match self.export_settings.ras {
            Some(v) => {
                if v.is_empty() {
                    Ok(Box::new(BalanceNonZeroSelector {}))
                } else {
                    let s: Vec<_> = v.iter().map(|s| s.as_str()).collect();
                    let ras = BalanceNonZeroByAccountSelector::from(&s)?;

                    Ok(Box::new(ras))
                }
            }
            None => Ok(Box::<BalanceNonZeroSelector>::default()),
        }
    }
}

impl Export for EquityExporter<'_> {
    fn write_export<'a, W: io::Write + ?Sized>(
        &self,
        cfg: &mut Settings,
        writer: &mut W,
        txn_data: &TxnSet,
    ) -> Result<(), Box<dyn Error>> {
        let bal_acc_sel = self.get_acc_selector()?;

        let bal = Balance::from(&String::default(), txn_data, bal_acc_sel.as_ref(), cfg);

        if bal.is_empty() {
            // todo: check if this is actually possible?
            return Ok(());
        };

        let eq_txn_indent = "   ";
        let equity_account = "Equity:Default·Account".to_string();

        let hdr_str = |last_txn: Option<&&Transaction>, c: &String| -> String {
            let comm_str = || -> String {
                if c.is_empty() {
                    String::default()
                } else {
                    format!(" for {}", c)
                }
            };
            let txn_uuid_str = |uuid: Option<Uuid>| -> String {
                match uuid {
                    Some(u) => format!(": last txn (uuid): {}", u),
                    None => String::default(),
                }
            };
            match last_txn {
                Some(txn) => {
                    format!(
                        "{} 'Equity{}{}",
                        txn.header.timestamp.format(&Rfc3339).unwrap(), // todo: unwrap
                        comm_str(),
                        txn_uuid_str(txn.header.uuid)
                    )
                }
                _ => {
                    "Internal logic error".to_string() // todo: fix this
                }
            }
        };

        let last_txn = txn_data.txns.last();

        let equity_txn_str: Vec<String> = bal
            .bal
            .iter()
            .chunk_by(|btn| &btn.acctn.comm.name)
            .into_iter()
            .flat_map(|(c, bs)| {
                let btns: Vec<_> = bs.collect();
                let dsum: Decimal = btns.clone().into_iter().map(|b| b.account_sum).sum();
                let bal_posting = {
                    let value = if c.is_empty() {
                        format!("{}", -dsum)
                    } else {
                        format!("{} {}", -dsum, c)
                    };
                    let ea = match &self.export_settings.eqa {
                        Some(eqa) => eqa,
                        None => &equity_account,
                    };
                    format!("{}{}  {}", eq_txn_indent, ea, value)
                };
                /*
                 * equity transaction per commodity
                 */
                let eq_postings = btns
                    .into_iter()
                    .map(|b| {
                        let comm = &b.acctn.comm;
                        format!(
                            "{}{}  {}{}",
                            eq_txn_indent,
                            b.acctn.atn.account,
                            b.account_sum,
                            match comm.is_some() {
                                true => { format!(" {}", comm.name) },
                                false => String::new(),
                            }
                        )
                    })
                    .collect::<Vec<String>>();

                let mut eq_txn = Vec::<String>::new();

                eq_txn.push(hdr_str(last_txn, c));
                if let Some(md) = &txn_data.metadata {
                        for mdi in md.items.clone() {
                            eq_txn.extend(mdi.text().iter().map(|v| {
                                format!("{}; {}", eq_txn_indent, v)
                            }).collect::<Vec<_>>());
                            eq_txn.push(format!("{};", eq_txn_indent));
                        }

                        if let Some(asc) = get_account_selector_checksum(cfg, self.export_settings.ras).unwrap() { // todo: unwrap
                            for v in asc.text() {
                                eq_txn.push(format!("{}; {}", eq_txn_indent, &v));
                            }
                            eq_txn.push(format!("{};", eq_txn_indent));
                        }
                };
                if dsum.is_zero() {
                    eq_txn.push(format!("{}; WARNING:", eq_txn_indent));
                    eq_txn.push(format!("{}; WARNING: The sum of equity transaction is zero without equity account.", eq_txn_indent));
                    eq_txn.push(format!("{}; WARNING: Therefore there is no equity posting row, and this is probably not right.", eq_txn_indent));
                    eq_txn.push(format!("{}; WARNING: Is account selector correct for this Equity Export?", eq_txn_indent));
                    eq_txn.push(format!("{}; WARNING:", eq_txn_indent));
                }

                eq_txn.extend(eq_postings);
                if !dsum.is_zero() {
                    eq_txn.push(bal_posting);
                }
                eq_txn.push("".to_string());
                eq_txn
            })
            .collect();

        for i in equity_txn_str {
            writeln!(writer, "{}", i)?;
        }

        Ok(())
    }
}
