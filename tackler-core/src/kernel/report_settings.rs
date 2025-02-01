/*
 * Tackler-NG 2025
 * SPDX-License-Identifier: Apache-2.0
 */
use crate::config::Scale;
use crate::kernel::price_lookup::PriceLookup;
use crate::kernel::Settings;
use crate::model::Commodity;
use jiff::tz::TimeZone;
use std::error::Error;
use std::sync::Arc;
use tackler_api::txn_ts::{GroupBy, TimestampStyle};

#[derive(Debug, Clone)]
pub struct BalanceSettings {
    pub(crate) title: String,
    pub(crate) ras: Vec<String>,
    pub(crate) scale: Scale,
    pub(crate) report_commodity: Option<Arc<Commodity>>,
    pub(crate) price_lookup: PriceLookup,
}

impl TryFrom<&Settings> for BalanceSettings {
    type Error = Box<dyn Error>;

    fn try_from(settings: &Settings) -> Result<Self, Self::Error> {
        Ok(BalanceSettings {
            title: settings.report.balance.title.clone(),
            ras: settings.get_balance_ras(),
            scale: settings.report.scale.clone(),
            report_commodity: settings.get_report_commodity(),
            price_lookup: settings.get_price_lookup(),
        })
    }
}

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
    ) -> Result<BalanceGroupSettings, Box<dyn Error>> {
        let bgs = BalanceGroupSettings {
            title: settings.report.balance_group.title.clone(),
            ras: settings.get_balance_group_ras(),
            group_by: group_by.unwrap_or(settings.report.balance_group.group_by),
            report_tz: settings.report.report_tz.clone(),
            report_commodity: settings.get_report_commodity(),
            price_lookup: settings.get_price_lookup(),
            scale: settings.report.scale.clone(),
        };
        Ok(bgs)
    }
}

#[derive(Debug, Clone)]
pub struct RegisterSettings {
    pub title: String,
    pub ras: Vec<String>,
    pub report_tz: TimeZone,
    pub report_commodity: Option<Arc<Commodity>>,
    pub price_lookup: PriceLookup,
    pub timestamp_style: TimestampStyle,
    pub(crate) scale: Scale,
}

impl TryFrom<&Settings> for RegisterSettings {
    type Error = Box<dyn Error>;

    fn try_from(settings: &Settings) -> Result<RegisterSettings, Box<dyn Error>> {
        let rs = RegisterSettings {
            title: settings.report.register.title.clone(),
            ras: settings.get_register_ras(),
            report_tz: settings.report.report_tz.clone(),
            report_commodity: settings.get_report_commodity(),
            price_lookup: settings.get_price_lookup(),
            timestamp_style: settings.report.register.timestamp_style,
            scale: settings.report.scale.clone(),
        };
        Ok(rs)
    }
}
