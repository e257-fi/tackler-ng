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
use crate::kernel::hash::Hash;
use serde::Deserialize;
use std::error::Error;
use std::fmt::Debug;
use std::fs;
use std::path::Path;
use tackler_api::txn_ts;
use tackler_api::txn_ts::{GroupBy, TimestampStyle};
use tackler_rs::get_abs_path;
use time::{format_description, Time, UtcOffset};
use time_tz::{timezones, Tz};

///
/// Timezone type, either named zone or offset
///
pub enum TimezoneType {
    /// Timezone by Offset e.g. -07:00
    Offset(UtcOffset),
    /// Timezone by name e.g. Europe/Helsinki
    Name(&'static Tz),
}
impl Clone for TimezoneType {
    fn clone(&self) -> Self {
        match self {
            TimezoneType::Offset(offset) => TimezoneType::Offset(*offset),
            TimezoneType::Name(tz) => TimezoneType::Name(tz),
        }
    }
}
impl Debug for TimezoneType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            TimezoneType::Offset(offset) => write!(f, "Offset: {:?}", offset),
            TimezoneType::Name(tz) => write!(f, "Timezone: {:?}", tz),
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Config {
    pub(crate) kernel: Kernel,
    pub(crate) transaction: Transaction,
    pub(crate) report: Report,
}

impl Config {
    pub fn from<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn Error>> {
        let cfg_raw: ConfigRaw = toml::from_str(fs::read_to_string(&path)?.as_str())?;

        let accs_path = get_abs_path(&path, cfg_raw.transaction.accounts.file.as_str())?;
        let accounts: Accounts = toml::from_str(fs::read_to_string(accs_path)?.as_str())?;

        let comms_path = get_abs_path(&path, cfg_raw.transaction.commodities.file.as_str())?;
        let commodities: Commodities = toml::from_str(fs::read_to_string(comms_path)?.as_str())?;

        let tags_path = get_abs_path(&path, cfg_raw.transaction.tags.file.as_str())?;
        let tags: Tags = toml::from_str(fs::read_to_string(tags_path)?.as_str())?;

        Ok(Config {
            kernel: Kernel::from(&cfg_raw.kernel)?,
            transaction: Transaction {
                accounts,
                commodities,
                tags,
            },
            report: {
                Report {
                    report_tz: timezones::get_by_name(cfg_raw.report.report_tz.as_str())
                        .ok_or("Timezone err TODO")?,
                    report_acc_sel: cfg_raw.report.accounts.clone(),
                    scale: cfg_raw.report.scale.clone(),
                    register: Register::from(&cfg_raw.report.register, &cfg_raw.report)?,
                    balance_group: BalanceGroup::from(
                        &cfg_raw.report.balance_group,
                        &cfg_raw.report,
                    )?,
                    balance: Balance::from(&cfg_raw.report.balance, &cfg_raw.report)?,
                }
            },
        })
    }
}
#[derive(Deserialize)]
struct ConfigRaw {
    kernel: KernelRaw,
    transaction: TransactionRaw,
    report: ReportRaw,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct Kernel {
    pub(crate) strict: bool,
    pub(crate) timestamp: Timestamp,
    pub(crate) audit: Audit,
    pub input: Input,
}
impl Kernel {
    fn from(k_raw: &KernelRaw) -> Result<Kernel, Box<dyn Error>> {
        let k = Kernel {
            strict: k_raw.strict,
            timestamp: Timestamp::from(&k_raw.timestamp)?,
            audit: Audit::from(&k_raw.audit)?,
            input: k_raw.input.clone(),
        };
        Ok(k)
    }
}

#[allow(dead_code)]
#[derive(Clone, Deserialize)]
pub struct KernelRaw {
    pub(crate) strict: bool,
    pub(crate) timestamp: TimestampRaw,
    pub(crate) audit: AuditRaw,
    pub input: Input,
}

#[derive(Debug, Clone)]
pub struct Timestamp {
    pub(crate) default_time: Time,
    pub(crate) timezone: TimezoneType,
}

impl Default for Timestamp {
    fn default() -> Self {
        Timestamp {
            default_time: Time::MIDNIGHT,
            timezone: TimezoneType::Offset(UtcOffset::UTC),
        }
    }
}

impl Timestamp {
    fn from(ts_raw: &TimestampRaw) -> Result<Timestamp, Box<dyn Error>> {
        let ts = Timestamp {
            default_time: {
                let t = ts_raw.default_time;
                Time::from_hms_nano(t.hour, t.minute, t.second, t.nanosecond)?
            },
            timezone: { TimezoneType::from(&ts_raw.timezone)? },
        };
        Ok(ts)
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct TimestampRaw {
    #[serde(rename = "default-time")]
    pub(crate) default_time: toml::value::Time,
    pub(crate) timezone: TimezoneRaw,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct TimezoneRaw {
    pub(crate) name: Option<String>,
    pub(crate) offset: Option<String>,
}

impl TimezoneType {
    pub fn from(tz_raw: &TimezoneRaw) -> Result<TimezoneType, Box<dyn Error>> {
        let tz = match (&tz_raw.name, &tz_raw.offset) {
            (Some(_), Some(_)) => {
                let msg = "timezone: 'name' and 'offset' are both defined".to_string();
                return Err(msg.into());
            }
            (Some(tz_name), None) => TimezoneType::Name(
                timezones::get_by_name(tz_name).ok_or(format!("Unknown timezone '{tz_name}'"))?,
            ),
            (None, Some(offset)) => {
                let offset_format = format_description::parse("[offset_hour]:[offset_minute]")?;
                let offset = UtcOffset::parse(offset, &offset_format)?;
                TimezoneType::Offset(offset)
            }
            (None, None) => TimezoneType::Offset(UtcOffset::UTC),
        };
        Ok(tz)
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct AuditRaw {
    pub(crate) hash: String,
    pub(crate) mode: bool,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct Audit {
    pub(crate) hash: Hash,
    pub(crate) mode: bool,
}

impl Audit {
    fn from(a_raw: &AuditRaw) -> Result<Audit, Box<dyn Error>> {
        let a = Audit {
            hash: Hash::from(&a_raw.hash)?,
            mode: a_raw.mode,
        };
        Ok(a)
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
struct TransactionRaw {
    accounts: AccountsRaw,
    commodities: CommoditiesRaw,
    tags: TagsRaw,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct Transaction {
    pub(crate) accounts: Accounts,
    pub(crate) commodities: Commodities,
    pub(crate) tags: Tags,
}

#[derive(Debug, Clone, Default, Deserialize)]
struct AccountsRaw {
    file: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Accounts {
    #[serde(rename = "accounts")]
    pub names: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
struct CommoditiesRaw {
    file: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct Commodities {
    #[serde(rename = "permit-empty-commodity")]
    pub(crate) permit_empty_commodity: Option<bool>,

    #[serde(rename = "commodities")]
    pub(crate) names: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
struct TagsRaw {
    file: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct Tags {
    #[serde(rename = "tags")]
    pub(crate) names: Vec<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct Input {
    pub storage: String,
    pub fs: Option<FS>,
    pub git: Option<Git>,
}

#[rustfmt::skip]
impl Input {
    pub const STORAGE_FS:  &'static str = "fs";
    pub const STORAGE_GIT: &'static str = "git";
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct FS {
    pub dir: String,
    pub glob: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct Git {
    pub repo: String,
    #[serde(rename = "ref")]
    pub git_ref: String,
    pub dir: String,
    pub suffix: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReportRaw {
    #[serde(rename = "report-timezone")]
    pub report_tz: String,
    pub accounts: Option<Vec<String>>,
    pub scale: Scale,
    pub register: RegisterRaw,
    #[serde(rename = "balance-group")]
    pub balance_group: BalanceGroupRaw,
    pub balance: BalanceRaw,
}

#[derive(Debug, Clone)]
pub struct Report {
    pub report_tz: &'static Tz,
    pub report_acc_sel: Option<Vec<String>>,
    pub scale: Scale,
    pub register: Register,
    pub balance_group: BalanceGroup,
    pub balance: Balance,
}

impl Default for Report {
    fn default() -> Self {
        Report {
            report_tz: txn_ts::TZ_UTC,
            report_acc_sel: None,
            scale: Scale::default(),
            register: Register::default(),
            balance_group: BalanceGroup::default(),
            balance: Balance::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Scale {
    pub min: u16,
    pub max: u16,
}

impl Default for Scale {
    fn default() -> Self {
        Scale { min: 2, max: 7 }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Register {
    pub title: String,
    pub timestamp_style: TimestampStyle,
    pub acc_sel: Accounts,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RegisterRaw {
    title: String,
    #[serde(rename = "timestamp-style")]
    timestamp_style: String,
    accounts: Option<Vec<String>>,
}

impl Register {
    fn from(regraw: &RegisterRaw, report: &ReportRaw) -> Result<Register, Box<dyn Error>> {
        Ok(Register {
            title: regraw.title.clone(),
            timestamp_style: TimestampStyle::from(regraw.timestamp_style.as_str())?,
            acc_sel: Accounts {
                names: match &regraw.accounts {
                    Some(av) => av.clone(),
                    None => match &report.accounts {
                        Some(av) => av.clone(),
                        None => vec![],
                    },
                },
            },
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct BalanceGroupRaw {
    title: String,
    #[serde(rename = "group-by")]
    group_by: String,
    accounts: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default)]
pub struct BalanceGroup {
    pub title: String,
    pub group_by: GroupBy,
    pub acc_sel: Accounts,
}

impl BalanceGroup {
    fn from(balgrp: &BalanceGroupRaw, report: &ReportRaw) -> Result<BalanceGroup, Box<dyn Error>> {
        Ok(BalanceGroup {
            title: balgrp.title.clone(),
            group_by: GroupBy::from(balgrp.group_by.as_str())?,
            acc_sel: Accounts {
                names: match &balgrp.accounts {
                    Some(av) => av.clone(),
                    None => match &report.accounts {
                        Some(av) => av.clone(),
                        None => vec![],
                    },
                },
            },
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct Balance {
    pub title: String,
    pub acc_sel: Accounts,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BalanceRaw {
    title: String,
    accounts: Option<Vec<String>>,
}

impl Balance {
    fn from(regraw: &BalanceRaw, report: &ReportRaw) -> Result<Balance, Box<dyn Error>> {
        Ok(Balance {
            title: regraw.title.clone(),
            acc_sel: Accounts {
                names: match &regraw.accounts {
                    Some(av) => av.clone(),
                    None => match &report.accounts {
                        Some(av) => av.clone(),
                        None => vec![],
                    },
                },
            },
        })
    }
}
