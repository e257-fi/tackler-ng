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

use std::error::Error;
use std::io;
use crate::kernel::Settings;
use crate::model::TxnSet;
use crate::export::Export;
use crate::kernel::balance::Balance;
use crate::report::BalanceReporter;

#[derive(Debug, Clone)]
pub struct EquitySettings<'a> {
    pub title: Option<String>,
    pub eqa: Option<String>,
    pub ras: &'a Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct EquityExporter<'a> {
    pub export_settings: EquitySettings<'a>,
}

impl Export for EquityExporter<'_> {
    fn write_export<W: io::Write + ?Sized>(
        &self,
        cfg: &Settings,
        writer: &mut W,
        txn_data: &TxnSet,
    ) -> Result<(), Box<dyn Error>> {

        let bal_acc_sel = if let Some(accs) = &cfg.accounts {
            // BalanceFilterNonZero(cfg.audit.hash)
            // FIXME: not the real one
            BalanceReporter::acc_selector(&self.export_settings.ras)?
        } else {
            // BalanceFilterNonZeroByAccount(cfg.accounts, cfg.audit.hash)
            // FIXME: not the real one
            BalanceReporter::acc_selector(&self.export_settings.ras)?
        };

        let bal = Balance::from(
            &String::default(),
            txn_data,
            bal_acc_sel.as_ref(),
        );

        if bal.is_empty() {
            // todo: check if this is actually possible?
            return Ok(())
        };

        let eq_txn_indent = "   ";
        let last_txn = txn_data.txns.last();


        //unimplemented!()
        writeln!(writer, "Hello There")?; // TODO
        Ok(())
    }
}
