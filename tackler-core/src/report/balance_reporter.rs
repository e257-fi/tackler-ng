/*
 * Tackler-NG 2023-2025
 * SPDX-License-Identifier: Apache-2.0
 */

use super::Commodity;
use crate::kernel::balance::{BTNs, Balance, Deltas};
use crate::kernel::report_item_selector::{
    BalanceAllSelector, BalanceByAccountSelector, BalanceSelector,
};
use crate::kernel::Settings;
use crate::model::{BalanceTreeNode, TxnSet};
use crate::report::{write_acc_sel_checksum, Report};
use crate::{
    config::Scale,
    model::price_entry::{PriceDb, PriceLookup},
};
use itertools::Itertools;
use rust_decimal::prelude::Zero;
use rust_decimal::{Decimal, RoundingStrategy};
use std::error::Error;
use std::io;
use std::{cmp::max, sync::Arc};

#[derive(Debug, Clone)]
pub struct BalanceSettings {
    pub(crate) title: String,
    pub(crate) ras: Vec<String>,
    pub(crate) scale: Scale,
    pub(crate) commodity: Option<Arc<Commodity>>,
    pub(crate) price_lookup: PriceLookup,
}

impl TryFrom<&Settings> for BalanceSettings {
    type Error = Box<dyn Error>;

    fn try_from(settings: &Settings) -> Result<Self, Self::Error> {
        Ok(BalanceSettings {
            title: settings.report.balance.title.clone(),
            ras: settings.get_balance_ras(),
            scale: settings.report.scale.clone(),
            commodity: None,
            price_lookup: Default::default(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct BalanceReporter {
    pub report_settings: BalanceSettings,
}

impl TryFrom<&Settings> for BalanceReporter {
    type Error = Box<dyn Error>;

    fn try_from(settings: &Settings) -> Result<Self, Self::Error> {
        Ok(BalanceReporter {
            report_settings: BalanceSettings::try_from(settings)?,
        })
    }
}

impl BalanceReporter {
    pub(crate) fn acc_selector(ras: &[String]) -> Result<Box<dyn BalanceSelector>, Box<dyn Error>> {
        if ras.is_empty() {
            Ok(Box::<BalanceAllSelector>::default())
        } else {
            let s: Vec<_> = ras.iter().map(|s| s.as_str()).collect();
            let ras = BalanceByAccountSelector::from(&s)?;
            Ok(Box::new(ras))
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
        bal_settings: &BalanceSettings,
    ) -> Result<(), Box<dyn Error>> {
        let get_max_sum_len = |bal: &BTNs, f: fn(&BalanceTreeNode) -> Decimal| -> usize {
            bal.iter()
                .map(|btn| {
                    let d = f(btn);
                    // include space for '+-' to the length always
                    format!("{:+.prec$}", d, prec = bal_settings.scale.get_precision(&d))
                        .chars()
                        .count()
                })
                .fold(0, max)
        };
        fn get_max_delta_len(deltas: &Deltas) -> usize {
            deltas
                .iter()
                .map(|(_, d)| format!("{}", d).chars().count())
                .fold(0, max)
        }
        /// Max used length of commodity could be calculated from deltas
        /// because all balance account commodities are present in there
        fn get_max_commodity_len(deltas: &Deltas) -> usize {
            deltas
                .iter()
                .map(|(opt_comm, _)| {
                    opt_comm
                        .as_ref()
                        .map_or(0, |comm| comm.name.chars().count())
                })
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

        let sub_acc_tree_sum_len = get_max_sum_len(&bal_report.bal, |btn| btn.sub_acc_tree_sum);

        // filler between account sums (acc and accTree sums)
        // width of this filler is mandated by delta sum's max commodity length,
        // because then AccTreesSum won't overlap with delta's commodity
        let filler_field = if comm_max_len.is_zero() {
            " ".repeat(3)
        } else {
            " ".repeat(4 + comm_max_len)
        };
        let filler_field_len = filler_field.chars().count();

        fn make_commodity_field(comm_max_len: usize, btn: &BalanceTreeNode) -> String {
            if comm_max_len.is_zero() {
                // always separate with two spaces
                " ".repeat(2)
            } else {
                let comm = &btn.acctn.comm;
                match &comm.is_some() {
                    true => {
                        format!(" {: <cl$}  ", comm.name, cl = comm_max_len)
                    }
                    false => format!(" {}  ", " ".repeat(comm_max_len)),
                }
            }
        }

        let left_ruler = " ".repeat(9);

        writeln!(writer, "{}", bal_report.title)?;
        writeln!(writer, "{}", "-".repeat(bal_report.title.chars().count()))?;

        if !bal_report.is_empty() {
            for btn in &bal_report.bal {
                let prec_1 = bal_settings.scale.get_precision(&btn.account_sum);
                let prec_2 = bal_settings.scale.get_precision(&btn.sub_acc_tree_sum);

                writeln!(
                    writer,
                    "{left_ruler}{:>asl$.prec_1$}{:>width$}{:>satsl$.prec_2$}{}{}",
                    btn.account_sum.round_dp_with_strategy(
                        prec_1 as u32,
                        RoundingStrategy::MidpointAwayFromZero
                    ),
                    "",
                    btn.sub_acc_tree_sum.round_dp_with_strategy(
                        prec_2 as u32,
                        RoundingStrategy::MidpointAwayFromZero
                    ),
                    make_commodity_field(comm_max_len, btn),
                    btn.acctn.atn,
                    asl = left_sum_len,
                    satsl = sub_acc_tree_sum_len,
                    width = filler_field_len,
                )?;
            }

            writeln!(
                writer,
                "{}",
                "=".repeat(
                    left_ruler.chars().count()
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
                let prec = bal_settings.scale.get_precision(delta.1);
                writeln!(
                    writer,
                    "{left_ruler}{:>width$.prec$}{}",
                    delta.1.round_dp_with_strategy(
                        prec as u32,
                        RoundingStrategy::MidpointAwayFromZero
                    ),
                    delta
                        .0
                        .as_ref()
                        .map_or(String::default(), |c| format!(" {}", &c.name)),
                    width = left_sum_len,
                )?;
            }
        }
        Ok(())
    }
}

impl Report for BalanceReporter {
    fn write_txt_report<W: io::Write + ?Sized>(
        &self,
        cfg: &Settings,
        writer: &mut W,
        txn_data: &TxnSet<'_>,
        price_db: &PriceDb,
    ) -> Result<(), Box<dyn Error>> {
        let bal_acc_sel = self.get_acc_selector()?;

        let price_lookup_ctx = &self.report_settings.price_lookup.make_ctx(
            &txn_data.txns,
            self.report_settings.commodity.clone(),
            price_db,
        );

        write_acc_sel_checksum(cfg, writer, bal_acc_sel.as_ref())?;
        writeln!(writer)?;

        let bal_report = Balance::from(
            &self.report_settings.title,
            txn_data,
            price_lookup_ctx,
            bal_acc_sel.as_ref(),
            cfg,
        )?;

        BalanceReporter::txt_report(writer, &bal_report, &self.report_settings)?;
        Ok(())
    }
}
