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

use crate::kernel::balance::Balance;
use crate::model::TxnData;
use crate::report::Report;
use std::error::Error;
use std::io::Write;

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
    fn get_acc_selector(&self) -> Result<Box<dyn BalanceSelector>, Box<dyn Error>> {
        match self.report_settings.ras.as_ref() {
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
}

impl Report for BalanceReporter {
    fn write_txt_report<W: Write + ?Sized>(&self, writer: &mut W, txn_data: &TxnData) {
        let bal_acc_sel = self.get_acc_selector().unwrap(/*:todo:*/);

        let bal_report = Balance::from(
            self.report_settings
                .title
                .as_ref()
                .unwrap_or(&String::default()),
            txn_data,
            bal_acc_sel,
        );
        write!(writer, "{:#?}", bal_report).unwrap(/*:todo:*/);
    }
}
