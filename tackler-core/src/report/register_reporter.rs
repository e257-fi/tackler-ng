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
use crate::kernel::report_item_selector::{
    RegisterAllSelector, RegisterByAccountSelector, RegisterSelector,
};
use crate::kernel::Settings;
use crate::model::{RegisterEntry, TxnSet};
use crate::report::{get_account_selector_checksum, Report};
use std::error::Error;
use std::io;
use tackler_api::metadata::items::Text;

#[derive(Debug, Clone)]
pub struct RegisterSettings<'a> {
    pub title: Option<String>,
    pub ras: &'a Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct RegisterReporter<'a> {
    pub report_settings: RegisterSettings<'a>,
}

impl RegisterReporter<'_> {
    fn get_acc_selector(&self) -> Result<Box<dyn RegisterSelector>, Box<dyn Error>> {
        match self.report_settings.ras.as_ref() {
            Some(v) => {
                if v.is_empty() {
                    Ok(Box::new(RegisterAllSelector {}))
                } else {
                    let s: Vec<_> = v.iter().map(|s| s.as_str()).collect();
                    let ras = RegisterByAccountSelector::from(&s)?;

                    Ok(Box::new(ras))
                }
            }
            None => Ok(Box::new(RegisterAllSelector {})),
        }
    }
}

fn reg_entry_txt_writer<W: io::Write + ?Sized>(
    f: &mut W,
    re: &RegisterEntry,
) -> Result<(), Box<dyn Error>> {
    if !re.posts.is_empty() {
        write!(f, "{}", re)?;
    }
    Ok(())
}

impl Report for RegisterReporter<'_> {
    fn write_txt_report<W: io::Write + ?Sized>(
        &self,
        cfg: &Settings,
        writer: &mut W,
        txns: &TxnSet,
    ) -> Result<(), Box<dyn Error>> {
        let empty = String::default();

        writeln!(writer, "{}", "-".repeat(82))?;
        if let Some(asc) = get_account_selector_checksum(cfg, self.report_settings.ras)? {
            for v in asc.text() {
                writeln!(writer, "{}", &v)?;
            }
        }
        writeln!(writer)?;

        let title = self.report_settings.title.as_ref().unwrap_or(&empty);
        writeln!(writer, "{}", title)?;
        writeln!(writer, "{}", "-".repeat(title.len()))?;

        let ras = self.get_acc_selector()?;

        accumulator::register_engine(&txns.txns, ras.as_ref(), writer, reg_entry_txt_writer)?;
        writeln!(writer, "{}", "=".repeat(82))?;
        Ok(())
    }
}
