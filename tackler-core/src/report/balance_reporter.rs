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

use crate::kernel::balance::{BTNs, Balance, Deltas};
use crate::model::{BalanceTreeNode, TxnSet};
use crate::report::Report;
use itertools::Itertools;
use rust_decimal::prelude::Zero;
use rust_decimal::Decimal;
use std::cmp::max;
use std::error::Error;
use std::io;

use crate::kernel::report_item_selector::{
    BalanceAllSelector, BalanceByAccountSelector, BalanceSelector,
};

#[derive(Debug, Clone)]
pub struct BalanceSettings {
    pub title: Option<String>,
    pub ras: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct BalanceReporter {
    pub report_settings: BalanceSettings,
}

impl BalanceReporter {
    pub(crate) fn acc_selector(
        ras: &Option<Vec<String>>,
    ) -> Result<Box<dyn BalanceSelector>, Box<dyn Error>> {
        match ras.as_ref() {
            Some(v) => {
                if v.is_empty() {
                    Ok(Box::new(BalanceAllSelector {}))
                } else {
                    let s: Vec<_> = v.iter().map(|s| s.as_str()).collect();
                    let ras = BalanceByAccountSelector::from(&s)?;

                    Ok(Box::new(ras))
                }
            }
            None => Ok(Box::<BalanceAllSelector>::default()),
        }
    }

    fn get_acc_selector(&self) -> Result<Box<dyn BalanceSelector>, Box<dyn Error>> {
        BalanceReporter::acc_selector(&self.report_settings.ras)
    }
}

impl BalanceReporter {
    pub(crate) fn txt_report<W: io::Write + ?Sized>(
        writer: &mut W,
        bal_report: &Balance,
    ) -> Result<(), Box<dyn Error>> {
        fn get_max_sum_len(bal: &BTNs, f: fn(&BalanceTreeNode) -> Decimal) -> usize {
            bal.iter()
                .map(|btn| format!("{:.prec$}", f(btn), prec = 2).len())
                .fold(0, max)
        }

        fn get_max_delta_len(deltas: &Deltas) -> usize {
            deltas
                .iter()
                .map(|(_, d)| format!("{}", d).len())
                .fold(0, max)
        }
        /// Max used length of commodity could be calculated from deltas
        /// because all balance account commodities are present in there
        fn get_max_commodity_len(deltas: &Deltas) -> usize {
            deltas
                .iter()
                .map(|(opt_comm, _)| opt_comm.as_ref().map_or(0, |comm| comm.name.len()))
                .fold(0, max)
        }

        let delta_max_len = get_max_delta_len(&bal_report.deltas);
        let comm_max_len = get_max_commodity_len(&bal_report.deltas);

        // max of 12, max_sum_len or delta_max_len
        let left_sum_len = max(
            12,
            max(
                get_max_sum_len(&bal_report.bal, |btn| btn.account_sum),
                delta_max_len,
            ),
        );

        let sub_acc_sum_len = get_max_sum_len(&bal_report.bal, |btn| btn.sub_acc_tree_sum);

        // filler between account sums (acc and accTree sums)
        // width of this filler is mandated by delta sum's max commodity length,
        // because then AccTreesSum won't overlap with delta's commodity
        let filler_field = if comm_max_len.is_zero() {
            " ".repeat(3)
        } else {
            " ".repeat(4 + comm_max_len)
        }
        .len();

        fn make_commodity_field(comm_max_len: usize, btn: &BalanceTreeNode) -> String {
            if comm_max_len.is_zero() {
                // always separate with two spaces
                " ".repeat(2)
            } else {
                match &btn.acctn.commodity {
                    Some(c) => {
                        format!(" {: <cl$} ", c.name, cl = comm_max_len)
                    }
                    None => format!(" {} ", " ".repeat(comm_max_len)),
                }
            }
        }

        let left_ruler = " ".repeat(9);

        writeln!(writer, "{}", bal_report.title)?;
        writeln!(writer, "{}", "-".repeat(bal_report.title.len()))?;

        if !bal_report.is_empty() {
            for btn in &bal_report.bal {
                writeln!(
                    writer,
                    "{left_ruler}{:>asl$.prec$}{:>width$}{:>atl$.prec$}{}{}",
                    btn.account_sum,
                    "",
                    btn.sub_acc_tree_sum,
                    make_commodity_field(comm_max_len, btn),
                    btn.acctn,
                    asl = left_sum_len,
                    atl = sub_acc_sum_len,
                    width = filler_field,
                    prec = 2,
                )?;
            }

            writeln!(
                writer,
                "{}",
                "=".repeat(
                    left_ruler.len()
                        + left_sum_len
                        + (if comm_max_len.is_zero() {
                            0
                        } else {
                            comm_max_len + 1
                        })
                )
            )?;

            let deltas = bal_report.deltas.iter().sorted_by_key(|i| {
                i.0.as_ref()
                    .map_or(String::default(), |comm| comm.name.clone())
            });
            for delta in deltas {
                writeln!(
                    writer,
                    "{left_ruler}{:>width$.prec$} {}",
                    delta.1,
                    delta.0.as_ref().map_or(&String::default(), |c| &c.name),
                    width = left_sum_len,
                    prec = 2,
                )?;
            }
        }

        Ok(())
    }
}

impl Report for BalanceReporter {
    fn write_txt_report<W: io::Write + ?Sized>(
        &self,
        writer: &mut W,
        txn_data: &TxnSet,
    ) -> Result<(), Box<dyn Error>> {
        let bal_acc_sel = self.get_acc_selector()?;

        let bal_report = Balance::from(
            self.report_settings
                .title
                .as_ref()
                .unwrap_or(&String::default()),
            txn_data,
            bal_acc_sel.as_ref(),
        );

        BalanceReporter::txt_report(writer, &bal_report)
    }
}
