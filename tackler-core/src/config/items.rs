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
use crate::config::raw_items::{
    AccountsPathRaw, AccountsRaw, AuditRaw, BalanceGroupRaw, BalanceRaw, CommoditiesPathRaw,
    CommoditiesRaw, ConfigRaw, EquityRaw, ExportRaw, FsRaw, GitRaw, InputRaw, KernelRaw,
    RegisterRaw, ReportRaw, ScaleRaw, TagsPathRaw, TagsRaw, TimestampRaw, TimezoneRaw,
    TransactionRaw,
};
use crate::config::to_report_targets;
use crate::kernel::hash::Hash;
use rust_decimal::Decimal;
use std::error::Error;
use std::fmt::Debug;
use std::path::Path;
use std::{cmp, fs};
use tackler_api::txn_ts;
use tackler_api::txn_ts::{GroupBy, TimestampStyle};
use tackler_rs::get_abs_path;
use time::{format_description, Time, UtcOffset};
use time_tz::{timezones, Tz};

/// UI/CFG key value for none
const NONE_VALUE: &str = "none";

#[derive(Debug, Clone, Default)]
pub enum StorageType {
    #[default]
    FS,
    Git,
}

#[rustfmt::skip]
impl StorageType {
    pub const STORAGE_FS:   &'static str = "fs";
    pub const STORAGE_GIT:  &'static str = "git";

    pub(crate) fn from(storage: &str) -> Result<StorageType, Box<dyn Error>> {
        match storage {
            StorageType::STORAGE_FS => Ok(StorageType::FS),
            StorageType::STORAGE_GIT => Ok(StorageType::Git),
            _ => Err(format!("Unknown storage type: {}", storage).into()),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub enum ReportType {
    #[default]
    Balance,
    BalanceGroup,
    Register,
}
impl ReportType {
    pub fn from(r: &str) -> Result<Self, Box<dyn Error>> {
        match r {
            "balance" => Ok(ReportType::Balance),
            "balance-group" => Ok(ReportType::BalanceGroup),
            "register" => Ok(ReportType::Register),
            _ => Err(format!("Unknown report type {r}").into()),
        }
    }
}

#[derive(Debug, Default)]
pub enum ExportType {
    #[default]
    Equity,
    Identity,
}
impl ExportType {
    pub fn from(r: &str) -> Result<Self, Box<dyn Error>> {
        match r {
            "equity" => Ok(ExportType::Equity),
            "identity" => Ok(ExportType::Identity),
            _ => Err(format!("Unknown export type {r}").into()),
        }
    }
}

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

pub type AccountSelectors = Vec<String>;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Config {
    pub(crate) kernel: Kernel,
    pub(crate) transaction: Transaction,
    pub(crate) report: Report,
    pub(crate) export: Export,
}

impl Config {
    pub fn from<P: AsRef<Path>>(cfg_path: P) -> Result<Config, Box<dyn Error>> {
        let cfg_raw: ConfigRaw = toml::from_str(fs::read_to_string(&cfg_path)?.as_str())?;

        Ok(Config {
            kernel: Kernel::from(&cfg_raw.kernel)?,
            transaction: Transaction::from(cfg_path, &cfg_raw.transaction)?,
            report: Report::from(&cfg_raw.report)?,
            export: { Export::from(&cfg_raw.export, &cfg_raw.report)? },
        })
    }
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
            input: Input::from(&k_raw.input)?,
        };
        Ok(k)
    }
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

impl TimezoneType {
    fn from(tz_raw: &TimezoneRaw) -> Result<TimezoneType, Box<dyn Error>> {
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

#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct Input {
    pub storage: StorageType,
    pub fs: Option<FS>,
    pub git: Option<Git>,
}
impl Input {
    fn from(input_raw: &InputRaw) -> Result<Input, Box<dyn Error>> {
        // todo: checks
        let i = Input {
            storage: StorageType::from(input_raw.storage.as_str())?,
            fs: match &input_raw.fs {
                Some(fs) => Some(FS::from(fs)?),
                None => None,
            },
            git: match &input_raw.git {
                Some(git) => Some(Git::from(git)?),
                None => None,
            },
        };
        Ok(i)
    }
}

#[derive(Debug, Clone, Default)]
pub struct FS {
    pub dir: String,
    pub suffix: String,
}
impl FS {
    fn from(fs_raw: &FsRaw) -> Result<FS, Box<dyn Error>> {
        Ok(FS {
            dir: fs_raw.dir.clone(),
            suffix: fs_raw.suffix.clone(),
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct Git {
    pub repo: String,
    pub git_ref: String,
    pub dir: String,
    pub suffix: String,
}
impl Git {
    fn from(git_raw: &GitRaw) -> Result<Git, Box<dyn Error>> {
        Ok(Git {
            repo: git_raw.repo.clone(),
            git_ref: git_raw.git_ref.clone(),
            dir: git_raw.dir.clone(),
            suffix: git_raw.suffix.clone(),
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct Transaction {
    pub(crate) accounts: Accounts,
    pub(crate) commodities: Commodities,
    pub(crate) tags: Tags,
}

impl Transaction {
    fn from<P: AsRef<Path>>(
        path: P,
        txn_raw: &TransactionRaw,
    ) -> Result<Transaction, Box<dyn Error>> {
        Ok(Transaction {
            accounts: Accounts::from(&path, &txn_raw.accounts)?,
            commodities: Commodities::from(&path, &txn_raw.commodities)?,
            tags: Tags::from(&path, &txn_raw.tags)?,
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct Accounts {
    pub names: Vec<String>,
}
impl Accounts {
    fn from<P: AsRef<Path>>(
        path: P,
        accs_path_raw: &AccountsPathRaw,
    ) -> Result<Accounts, Box<dyn Error>> {
        let accs_path_str = accs_path_raw.path.as_str();
        match accs_path_str {
            NONE_VALUE => Ok(Accounts::default()),
            _ => {
                let accs_path = get_abs_path(&path, accs_path_str)?;
                let acc_raw: AccountsRaw = match fs::read_to_string(&accs_path) {
                    Ok(s) => toml::from_str(s.as_str())?,
                    Err(err) => {
                        let msg = format!("Accounts configuration error while reading file '{accs_path_str}': {err}");
                        return Err(msg.into());
                    }
                };
                Ok(Accounts {
                    names: acc_raw.names,
                })
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Commodities {
    pub(crate) permit_empty_commodity: Option<bool>,

    pub(crate) names: Vec<String>,
}
impl Commodities {
    fn from<P: AsRef<Path>>(
        path: P,
        comm_path_raw: &CommoditiesPathRaw,
    ) -> Result<Commodities, Box<dyn Error>> {
        let comm_path_str = comm_path_raw.path.as_str();
        match comm_path_str {
            NONE_VALUE => Ok(Commodities {
                permit_empty_commodity: Some(true),
                names: Vec::new(),
            }),
            _ => {
                let comm_path = get_abs_path(&path, comm_path_str)?;
                let comm_raw: CommoditiesRaw = match fs::read_to_string(&comm_path) {
                    Ok(s) => toml::from_str(s.as_str())?,
                    Err(err) => {
                        let msg = format!("Commodities configuration error while reading file '{comm_path_str}': {err}");
                        return Err(msg.into());
                    }
                };
                Ok(Commodities {
                    permit_empty_commodity: comm_raw.permit_empty_commodity,
                    names: comm_raw.names,
                })
            }
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct Tags {
    pub(crate) names: Vec<String>,
}

impl Tags {
    fn from<P: AsRef<Path>>(path: P, tags_path_raw: &TagsPathRaw) -> Result<Tags, Box<dyn Error>> {
        let tags_path_str = tags_path_raw.path.as_str();
        match tags_path_str {
            NONE_VALUE => Ok(Tags::default()),
            _ => {
                let tags_path = get_abs_path(&path, tags_path_str)?;
                let tags_raw: TagsRaw = match fs::read_to_string(&tags_path) {
                    Ok(s) => toml::from_str(s.as_str())?,
                    Err(err) => {
                        let msg = format!(
                            "Tags configuration error while reading file '{tags_path_str}': {err}"
                        );
                        return Err(msg.into());
                    }
                };
                Ok(Tags {
                    names: tags_raw.names,
                })
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Report {
    pub report_tz: &'static Tz,
    pub targets: Vec<ReportType>,
    pub report_acc_sel: Option<AccountSelectors>,
    pub scale: Scale,
    pub register: Register,
    pub balance_group: BalanceGroup,
    pub balance: Balance,
}

impl Default for Report {
    fn default() -> Self {
        Report {
            report_tz: txn_ts::TZ_UTC,
            targets: Vec::new(),
            report_acc_sel: None,
            scale: Scale::default(),
            register: Register::default(),
            balance_group: BalanceGroup::default(),
            balance: Balance::default(),
        }
    }
}

impl Report {
    fn from(report_raw: &ReportRaw) -> Result<Report, Box<dyn Error>> {
        let trgs = to_report_targets(&report_raw.targets)?;
        Ok(Report {
            report_tz: timezones::get_by_name(report_raw.report_tz.as_str())
                .ok_or("Timezone err TODO")?,
            targets: trgs,
            report_acc_sel: report_raw.accounts.clone(),
            scale: Scale::from(&report_raw.scale)?,
            register: Register::from(&report_raw.register, report_raw)?,
            balance_group: BalanceGroup::from(&report_raw.balance_group, report_raw)?,
            balance: Balance::from(&report_raw.balance, report_raw)?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Scale {
    min: u32,
    max: u32,
}
impl Scale {
    fn from(scale_raw: &ScaleRaw) -> Result<Scale, Box<dyn Error>> {
        let max_scale = 28;
        if scale_raw.min > max_scale || scale_raw.max > max_scale {
            let msg = format!(
                "scale error: too large value - maximum scale value for min or max is {max_scale}"
            );
            return Err(msg.into());
        }
        if scale_raw.max < scale_raw.min {
            let msg = "scale error: 'min' can't be greater than 'max'";
            return Err(msg.into());
        }
        Ok(Scale {
            min: scale_raw.min,
            max: scale_raw.max,
        })
    }
    pub fn get_precision(&self, d: &Decimal) -> usize {
        cmp::max(cmp::min(d.scale(), self.max), self.min) as usize
    }
}

impl Default for Scale {
    fn default() -> Self {
        Scale { min: 2, max: 7 }
    }
}

fn get_account_selector(
    acc_sel: &Option<AccountSelectors>,
    report: &ReportRaw,
) -> AccountSelectors {
    match acc_sel {
        Some(av) => av.clone(),
        None => match &report.accounts {
            Some(av) => av.clone(),
            None => vec![],
        },
    }
}

#[derive(Debug, Clone, Default)]
pub struct Register {
    pub title: String,
    pub timestamp_style: TimestampStyle,
    pub acc_sel: AccountSelectors,
}

impl Register {
    fn from(reg_raw: &RegisterRaw, report: &ReportRaw) -> Result<Register, Box<dyn Error>> {
        Ok(Register {
            title: reg_raw.title.clone(),
            timestamp_style: TimestampStyle::from(reg_raw.timestamp_style.as_str())?,
            acc_sel: get_account_selector(&reg_raw.acc_sel, report),
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct BalanceGroup {
    pub title: String,
    pub group_by: GroupBy,
    pub acc_sel: AccountSelectors,
}

impl BalanceGroup {
    fn from(
        balgrp_raw: &BalanceGroupRaw,
        report: &ReportRaw,
    ) -> Result<BalanceGroup, Box<dyn Error>> {
        Ok(BalanceGroup {
            title: balgrp_raw.title.clone(),
            group_by: GroupBy::from(balgrp_raw.group_by.as_str())?,
            acc_sel: get_account_selector(&balgrp_raw.acc_sel, report),
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct Balance {
    pub title: String,
    pub acc_sel: AccountSelectors,
}

impl Balance {
    fn from(bal_raw: &BalanceRaw, report: &ReportRaw) -> Result<Balance, Box<dyn Error>> {
        Ok(Balance {
            title: bal_raw.title.clone(),
            acc_sel: get_account_selector(&bal_raw.acc_sel, report),
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct Export {
    pub equity: Equity,
}
impl Export {
    fn from(export_raw: &ExportRaw, report: &ReportRaw) -> Result<Export, Box<dyn Error>> {
        Ok(Export {
            equity: Equity::from(&export_raw.equity, report)?,
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct Equity {
    pub(crate) equity_account: String,
    pub(crate) acc_sel: AccountSelectors,
}

impl Equity {
    fn from(eq_raw: &EquityRaw, report: &ReportRaw) -> Result<Equity, Box<dyn Error>> {
        Ok(Equity {
            equity_account: eq_raw.equity_account.clone(),
            acc_sel: get_account_selector(&eq_raw.acc_sel, report),
        })
    }
}