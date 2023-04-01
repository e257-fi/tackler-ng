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
use crate::kernel::report_item_selector::{
    RegisterAllSelector, RegisterByAccountSelector, RegisterSelector,
};
use crate::model::{RegisterEntry, TxnSet};
use crate::report::Report;
use std::error::Error;
use std::io::Write;

#[derive(Debug, Clone)]
pub struct RegisterSettings {
    pub title: Option<String>,
    pub ras: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct RegisterReporter {
    pub report_settings: RegisterSettings,
}

impl RegisterReporter {
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

fn reg_entry_txt_writer<W: Write + ?Sized>(
    f: &mut W,
    re: &RegisterEntry,
) -> Result<(), Box<dyn Error>> {
    if !re.posts.is_empty() {
        write!(f, "{}", re)?;
    }
    Ok(())
}

impl Report for RegisterReporter {
    fn write_txt_report<W: Write + ?Sized>(
        &self,
        writer: &mut W,
        txns: &TxnSet,
    ) -> Result<(), Box<dyn Error>> {
        let empty = String::default();

        let title = self.report_settings.title.as_ref().unwrap_or(&empty);
        writeln!(writer, "{}", title)?;
        writeln!(writer, "{}", "-".repeat(title.len()))?;

        let ras = self.get_acc_selector()?;

        accumulator::register_engine(&txns.txns, ras, writer, reg_entry_txt_writer)
    }
}
