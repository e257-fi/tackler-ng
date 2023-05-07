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

use crate::kernel::accumulator;
use crate::kernel::report_item_selector::BalanceSelector;
use crate::model::{Transaction, TxnSet};
use crate::report::{BalanceReporter, BalanceSettings, Report};
use std::error::Error;
use std::io;
use tackler_api::txn_ts;

#[derive(Debug, Clone)]
pub struct BalanceGroupReporter {
    pub report_settings: BalanceSettings,
}

impl BalanceGroupReporter {
    fn get_acc_selector(&self) -> Result<Box<dyn BalanceSelector>, Box<dyn Error>> {
        BalanceReporter::acc_selector(&self.report_settings.ras)
    }
}

fn group_by_date(txn: &Transaction) -> String {
    txn_ts::utc_date(txn.header.timestamp)
}

fn group_by_month(txn: &Transaction) -> String {
    txn_ts::utc_month(txn.header.timestamp)
}

impl Report for BalanceGroupReporter {
    fn write_txt_report<W: io::Write + ?Sized>(
        &self,
        writer: &mut W,
        txn_data: &TxnSet,
    ) -> Result<(), Box<dyn Error>> {
        let bal_acc_sel = self.get_acc_selector()?;

        let bal_groups =
            accumulator::balance_groups(&txn_data.txns, group_by_month, bal_acc_sel.as_ref());

        if let Some(title) = &self.report_settings.title {
            writeln!(writer, "{}", title)?;
            writeln!(writer, "{}", "-".repeat(title.len()))?;
            writeln!(writer)?;
        }

        for bal in &bal_groups {
            BalanceReporter::txt_report(writer, bal)?
        }
        Ok(())
    }
}
