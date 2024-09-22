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

use crate::kernel::accumulator;
use crate::kernel::accumulator::TxnGroupByOp;
use crate::kernel::Settings;
use crate::kernel::report_item_selector::BalanceSelector;
use crate::model::{Transaction, TxnSet};
use crate::report::{BalanceReporter};
use crate::report::{get_account_selector_checksum, Report};
use std::error::Error;
use std::io;
use tackler_api::txn_ts;
use tackler_api::txn_ts::GroupBy;
use time_tz::Tz;
use tackler_api::metadata::items::Text;

#[derive(Debug, Clone)]
pub struct BalanceGroupSettings<'a> {
    pub title: Option<String>,
    pub ras: &'a Option<Vec<String>>,
    pub group_by: GroupBy,
    pub report_tz: &'a Tz,
}

#[derive(Debug, Clone)]
pub struct BalanceGroupReporter<'a> {
    pub report_settings: BalanceGroupSettings<'a>,
}

impl<'a> BalanceGroupReporter<'a> {
    fn get_acc_selector(&self) -> Result<Box<dyn BalanceSelector>, Box<dyn Error>> {
        BalanceReporter::acc_selector(&self.report_settings.ras)
    }

    fn get_group_by_op(&self) -> TxnGroupByOp<'a> {
        let tz = self.report_settings.report_tz;
        match self.report_settings.group_by {
            GroupBy::IsoWeekDate => {
                Box::new(|txn: &Transaction| txn_ts::zoned_iso_week_date(txn.header.timestamp, tz))
            }
            GroupBy::IsoWeek => {
                Box::new(|txn: &Transaction| txn_ts::zoned_iso_week(txn.header.timestamp, tz))
            }
            GroupBy::Date => {
                Box::new(|txn: &Transaction| txn_ts::zoned_date(txn.header.timestamp, tz))
            }
            GroupBy::Month => {
                Box::new(|txn: &Transaction| txn_ts::zoned_month(txn.header.timestamp, tz))
            }
            GroupBy::Year => {
                Box::new(|txn: &Transaction| txn_ts::zoned_year(txn.header.timestamp, tz))
            }
        }
    }
}

impl<'a> Report for BalanceGroupReporter<'a> {
    fn write_txt_report<W: io::Write + ?Sized>(
        &self,
        cfg: &Settings,
        writer: &mut W,
        txn_data: &TxnSet,
    ) -> Result<(), Box<dyn Error>> {
        let bal_acc_sel = self.get_acc_selector()?;

        let group_by_op = self.get_group_by_op();
        let bal_groups =
            accumulator::balance_groups(&txn_data.txns, group_by_op, bal_acc_sel.as_ref());

        writeln!(writer, "{}", "-".repeat(82))?;
        if let Some(asc) = get_account_selector_checksum(&cfg, self.report_settings.ras)? {
            for v in asc.text() {
                writeln!(writer, "{}", &v)?;
            }
        }
        writeln!(writer, "")?;


        if let Some(title) = &self.report_settings.title {
            writeln!(writer, "{}", title)?;
            writeln!(writer, "{}", "-".repeat(title.len()))?;
            writeln!(writer)?;
        }

        for bal in &bal_groups {
            BalanceReporter::txt_report(writer, bal)?
        }
        writeln!(writer, "{}", "-".repeat(82))?;
        Ok(())
    }
}
