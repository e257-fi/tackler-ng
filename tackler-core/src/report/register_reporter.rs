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

use crate::config::Scale;
use crate::kernel::accumulator;
use crate::kernel::report_item_selector::{
    RegisterAllSelector, RegisterByAccountSelector, RegisterSelector,
};
use crate::kernel::Settings;
use crate::model::{RegisterEntry, TxnSet};
use crate::report::{get_report_tz, write_acc_sel_checksum, Report};
use jiff::tz::TimeZone;
use jiff::Zoned;
use std::error::Error;
use std::io;
use tackler_api::metadata::items::Text;
use tackler_api::txn_ts;
use tackler_api::txn_ts::TimestampStyle;

#[derive(Debug, Clone)]
pub struct RegisterSettings {
    pub title: String,
    pub ras: Vec<String>,
    pub report_tz: TimeZone,
    pub timestamp_style: TimestampStyle,
    pub(crate) scale: Scale,
}

impl RegisterSettings {
    pub fn from(settings: &Settings) -> Result<RegisterSettings, Box<dyn Error>> {
        let rs = RegisterSettings {
            title: settings.report.register.title.clone(),
            ras: settings.get_register_ras(),
            report_tz: settings.report.report_tz.clone(),
            timestamp_style: settings.report.register.timestamp_style,
            scale: settings.report.scale.clone(),
        };
        Ok(rs)
    }
}

#[derive(Debug, Clone)]
pub struct RegisterReporter {
    pub report_settings: RegisterSettings,
}

impl RegisterReporter {
    fn get_acc_selector(&self) -> Result<Box<dyn RegisterSelector<'_>>, Box<dyn Error>> {
        let ras = &self.report_settings.ras;
        if ras.is_empty() {
            Ok(Box::<RegisterAllSelector>::default())
        } else {
            let s: Vec<_> = ras.iter().map(|s| s.as_str()).collect();
            let ras = RegisterByAccountSelector::from(&s)?;

            Ok(Box::new(ras))
        }
    }
}

fn reg_entry_txt_writer<W: io::Write + ?Sized>(
    f: &mut W,
    re: &RegisterEntry<'_>,
    ts_style: TimestampStyle,
    report_tz: TimeZone,
    register_settings: &RegisterSettings,
) -> Result<(), Box<dyn Error>> {
    let fmt: fn(&Zoned, TimeZone) -> String = match ts_style {
        TimestampStyle::Date => txn_ts::as_tz_date,
        TimestampStyle::Secodns => txn_ts::as_tz_seconds,
        TimestampStyle::Full => txn_ts::as_tz_full,
    };

    if !re.posts.is_empty() {
        write!(
            f,
            "{}",
            re.fmt_with_tz(fmt, report_tz, &register_settings.scale)
        )?;
    }
    Ok(())
}

impl Report for RegisterReporter {
    fn write_txt_report<W: io::Write + ?Sized>(
        &self,
        cfg: &Settings,
        writer: &mut W,
        txns: &TxnSet<'_>,
    ) -> Result<(), Box<dyn Error>> {
        let acc_sel = self.get_acc_selector()?;

        write_acc_sel_checksum(cfg, writer, acc_sel.as_ref())?;

        // todo: fix this silliness
        for v in get_report_tz(cfg, self.report_settings.report_tz.clone())?
            .text(self.report_settings.report_tz.clone())
        {
            writeln!(writer, "{}", &v)?;
        }
        writeln!(writer)?;
        writeln!(writer)?;

        let title = &self.report_settings.title;
        writeln!(writer, "{}", title)?;
        writeln!(writer, "{}", "-".repeat(title.chars().count()))?;

        let ras = self.get_acc_selector()?;

        accumulator::register_engine(
            &txns.txns,
            ras.as_ref(),
            self.report_settings.timestamp_style,
            self.report_settings.report_tz.clone(),
            writer,
            reg_entry_txt_writer,
            &self.report_settings,
        )?;
        Ok(())
    }
}
