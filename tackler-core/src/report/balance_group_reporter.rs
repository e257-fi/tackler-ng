/*
 * Tackler-NG 2023-2025
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::kernel::accumulator::TxnGroupByOp;
use crate::kernel::report_item_selector::BalanceSelector;
use crate::kernel::Settings;
use crate::model::{Transaction, TxnSet};
use crate::report::{write_acc_sel_checksum, write_report_timezone, Report};
use crate::report::{BalanceReporter, BalanceSettings};
use crate::{config::Scale, model::price_entry::PriceDb};
use crate::{kernel::accumulator, model::price_entry::PriceLookup};
use jiff::tz::TimeZone;
use std::io;
use std::{error::Error, sync::Arc};
use tackler_api::txn_ts;
use tackler_api::txn_ts::GroupBy;

use super::Commodity;

#[derive(Debug, Clone)]
pub struct BalanceGroupSettings {
    pub title: String,
    pub ras: Vec<String>,
    pub group_by: GroupBy,
    pub report_tz: TimeZone,
    pub report_commodity: Option<Arc<Commodity>>,
    pub price_lookup: PriceLookup,
    pub scale: Scale,
}

impl BalanceGroupSettings {
    pub fn from(
        settings: &Settings,
        group_by: Option<GroupBy>,
        report_commodity: Option<Arc<Commodity>>,
        price_lookup: PriceLookup,
    ) -> Result<BalanceGroupSettings, Box<dyn Error>> {
        let bgs = BalanceGroupSettings {
            title: settings.report.balance_group.title.clone(),
            ras: settings.get_balance_group_ras(),
            group_by: group_by.unwrap_or(settings.report.balance_group.group_by),
            report_tz: settings.report.report_tz.clone(),
            report_commodity,
            price_lookup,
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
        let tz: TimeZone = self.report_settings.report_tz.clone();
        match self.report_settings.group_by {
            GroupBy::IsoWeekDate => Box::new(move |txn: &Transaction| {
                txn_ts::as_tz_iso_week_date(&txn.header.timestamp, tz.clone())
            }),
            GroupBy::IsoWeek => Box::new(move |txn: &Transaction| {
                txn_ts::as_tz_iso_week(&txn.header.timestamp, tz.clone())
            }),
            GroupBy::Date => Box::new(move |txn: &Transaction| {
                txn_ts::as_tz_date(&txn.header.timestamp, tz.clone())
            }),
            GroupBy::Month => Box::new(move |txn: &Transaction| {
                txn_ts::as_tz_month(&txn.header.timestamp, tz.clone())
            }),
            GroupBy::Year => Box::new(move |txn: &Transaction| {
                txn_ts::as_tz_year(&txn.header.timestamp, tz.clone())
            }),
        }
    }
}

impl Report for BalanceGroupReporter {
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
            self.report_settings.report_commodity.clone(),
            price_db,
        );

        let group_by_op = self.get_group_by_op();
        let bal_groups = accumulator::balance_groups(
            &txn_data.txns,
            group_by_op,
            price_lookup_ctx,
            bal_acc_sel.as_ref(),
            cfg,
        );

        write_acc_sel_checksum(cfg, writer, bal_acc_sel.as_ref())?;

        write_report_timezone(cfg, writer)?;

        writeln!(writer)?;
        writeln!(writer)?;

        let title = &self.report_settings.title;
        writeln!(writer, "{}", title)?;
        writeln!(writer, "{}", "-".repeat(title.chars().count()))?;

        let bal_settings = BalanceSettings {
            title: String::default(),
            ras: vec![],
            scale: self.report_settings.scale.clone(),
            commodity: self.report_settings.report_commodity.clone(),
            price_lookup: self.report_settings.price_lookup.clone(),
        };
        for bal in &bal_groups {
            BalanceReporter::txt_report(writer, bal, &bal_settings)?
        }
        Ok(())
    }
}
