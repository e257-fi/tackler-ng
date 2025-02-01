/*
 * Tackler-NG 2023-2025
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::kernel::accumulator;
use crate::kernel::report_item_selector::{
    RegisterAllSelector, RegisterByAccountSelector, RegisterSelector,
};
use crate::kernel::report_settings::RegisterSettings;
use crate::kernel::Settings;
use crate::model::{RegisterEntry, TxnSet};
use crate::report::{write_acc_sel_checksum, write_report_timezone, Report};
use jiff::tz::TimeZone;
use jiff::Zoned;
use std::error::Error;
use std::io;
use tackler_api::txn_ts;
use tackler_api::txn_ts::TimestampStyle;

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
    register_settings: &RegisterSettings,
) -> Result<(), Box<dyn Error>> {
    let ts_style = register_settings.timestamp_style;
    let report_tz = register_settings.report_tz.clone();

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
        txn_data: &TxnSet<'_>,
    ) -> Result<(), Box<dyn Error>> {
        let acc_sel = self.get_acc_selector()?;

        let report_commodity = self.report_settings.report_commodity.clone();
        let price_lookup_ctx = self.report_settings.price_lookup.make_ctx(
            &txn_data.txns,
            report_commodity,
            &cfg.price.price_db,
        );

        write_acc_sel_checksum(cfg, writer, acc_sel.as_ref())?;

        write_report_timezone(cfg, writer)?;

        writeln!(writer)?;
        writeln!(writer)?;

        let title = &self.report_settings.title;
        writeln!(writer, "{}", title)?;
        writeln!(writer, "{}", "-".repeat(title.chars().count()))?;

        let ras = self.get_acc_selector()?;

        accumulator::register_engine(
            &txn_data.txns,
            &price_lookup_ctx,
            ras.as_ref(),
            writer,
            reg_entry_txt_writer,
            &self.report_settings,
        )?;
        Ok(())
    }
}
