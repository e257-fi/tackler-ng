/*
 * Tackler-NG 2023-2024
 *
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::config::Scale;
use crate::kernel::accumulator;
use crate::kernel::accumulator::TxnGroupByOp;
use crate::kernel::report_item_selector::BalanceSelector;
use crate::kernel::Settings;
use crate::model::{Transaction, TxnSet};
use crate::report::{get_report_tz, write_acc_sel_checksum, Report};
use crate::report::{BalanceReporter, BalanceSettings};
use std::error::Error;
use std::io;
use tackler_api::metadata::items::Text;
use tackler_api::txn_ts;
use tackler_api::txn_ts::GroupBy;
use time_tz::Tz;

#[derive(Debug, Clone)]
pub struct BalanceGroupSettings {
    pub title: String,
    pub ras: Vec<String>,
    pub group_by: GroupBy,
    pub report_tz: &'static Tz,
    pub scale: Scale,
}

impl BalanceGroupSettings {
    pub fn from(
        settings: &Settings,
        group_by: Option<GroupBy>,
    ) -> Result<BalanceGroupSettings, Box<dyn Error>> {
        let bgs = BalanceGroupSettings {
            title: settings.report.balance_group.title.clone(),
            ras: settings.get_balance_group_ras(),
            group_by: match group_by {
                Some(group_by) => group_by,
                None => settings.report.balance_group.group_by,
            },
            report_tz: settings.report.report_tz,
            scale: settings.report.scale.clone(),
        };
        Ok(bgs)
    }
}

#[derive(Debug, Clone)]
pub struct BalanceGroupReporter {
    pub report_settings: BalanceGroupSettings,
}

impl BalanceGroupReporter {
    fn get_acc_selector(&self) -> Result<Box<dyn BalanceSelector>, Box<dyn Error>> {
        BalanceReporter::acc_selector(&self.report_settings.ras)
    }

    fn get_group_by_op(&self) -> TxnGroupByOp<'_> {
        let tz: &'static Tz = self.report_settings.report_tz;
        match self.report_settings.group_by {
            GroupBy::IsoWeekDate => Box::new(move |txn: &Transaction| {
                txn_ts::as_tz_iso_week_date(txn.header.timestamp, tz)
            }),
            GroupBy::IsoWeek => {
                Box::new(|txn: &Transaction| txn_ts::as_tz_iso_week(txn.header.timestamp, tz))
            }
            GroupBy::Date => {
                Box::new(|txn: &Transaction| txn_ts::as_tz_date(txn.header.timestamp, tz))
            }
            GroupBy::Month => {
                Box::new(|txn: &Transaction| txn_ts::as_tz_month(txn.header.timestamp, tz))
            }
            GroupBy::Year => {
                Box::new(|txn: &Transaction| txn_ts::as_tz_year(txn.header.timestamp, tz))
            }
        }
    }
}

impl Report for BalanceGroupReporter {
    fn write_txt_report<W: io::Write + ?Sized>(
        &self,
        cfg: &Settings,
        writer: &mut W,
        txn_data: &TxnSet<'_>,
    ) -> Result<(), Box<dyn Error>> {
        let bal_acc_sel = self.get_acc_selector()?;

        let group_by_op = self.get_group_by_op();
        let bal_groups =
            accumulator::balance_groups(&txn_data.txns, group_by_op, bal_acc_sel.as_ref(), cfg);

        write_acc_sel_checksum(cfg, writer, bal_acc_sel.as_ref())?;

        for v in get_report_tz(cfg, self.report_settings.report_tz)?.text() {
            writeln!(writer, "{}", &v)?;
        }
        writeln!(writer)?;
        writeln!(writer)?;

        let title = &self.report_settings.title;
        writeln!(writer, "{}", title)?;
        writeln!(writer, "{}", "-".repeat(title.chars().count()))?;

        let bal_settings = BalanceSettings {
            title: String::default(),
            ras: vec![],
            scale: self.report_settings.scale.clone(),
        };
        for bal in &bal_groups {
            BalanceReporter::txt_report(writer, bal, &bal_settings)?
        }
        Ok(())
    }
}
