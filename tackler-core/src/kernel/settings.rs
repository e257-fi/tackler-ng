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
use crate::config;
use crate::config::{AccountSelectors, Config, Export, Kernel, Report, ReportType, TimezoneType};
use crate::kernel::hash::Hash;
use crate::model::TxnAccount;
use crate::model::{AccountTreeNode, Commodity};
use crate::parser::GitInputSelector;
use std::collections::HashMap;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tackler_api::txn_header::Tag;
use time::{Date, OffsetDateTime, PrimitiveDateTime};
use time_tz::{OffsetResult, PrimitiveDateTimeExt};

pub struct GitInput {
    pub repo: PathBuf,
    pub dir: String,
    pub git_ref: GitInputSelector,
    pub ext: String,
}

pub struct FileInput {
    pub path: PathBuf,
}

pub struct FsInput {
    pub dir: PathBuf,
    pub suffix: String,
}

pub enum InputSettings {
    File(FileInput),
    Fs(FsInput),
    Git(GitInput),
}

#[derive(Debug, Default)]
struct Commodities {
    names: HashMap<String, Arc<Commodity>>,
    permit_empty_commodity: bool,
}

impl Commodities {
    fn default_empty_ok() -> Self {
        Commodities {
            names: HashMap::new(),
            permit_empty_commodity: true,
        }
    }
}

pub struct Settings {
    pub audit_mode: bool,
    pub report: Report,
    pub export: Export,
    kernel: Kernel,
    global_acc_sel: Option<AccountSelectors>,
    targets: Vec<ReportType>,
    accounts: HashMap<String, Arc<AccountTreeNode>>,
    commodities: Commodities,
    tags: HashMap<String, Arc<Tag>>,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            audit_mode: false,
            report: Report::default(),
            export: Export::default(),
            kernel: Kernel::default(),
            global_acc_sel: None,
            targets: Vec::new(),
            accounts: HashMap::new(),
            commodities: Commodities::default_empty_ok(),
            tags: HashMap::new(),
        }
    }
}

impl Settings {
    pub fn default_audit() -> Self {
        Settings {
            audit_mode: true,
            report: Report::default(),
            export: Export::default(),

            kernel: Kernel::default(),
            global_acc_sel: None,
            targets: Vec::new(),
            accounts: HashMap::new(),
            commodities: Commodities::default_empty_ok(),
            tags: HashMap::new(),
        }
    }
}

impl Settings {
    pub fn from(
        cfg_opt: Option<Config>,
        audit_mode: Option<bool>,
        report_accounts: Option<Vec<String>>,
    ) -> Result<Settings, Box<dyn Error>> {
        let cfg = match cfg_opt {
            Some(c) => c,
            None => {
                return match (audit_mode, report_accounts) {
                    (Some(h), Some(ra)) => {
                        let mut s = match h {
                            true => Self::default_audit(),
                            false => Self::default(),
                        };
                        s.global_acc_sel = Some(ra);
                        Ok(s)
                    }
                    (Some(h), None) => {
                        return Ok(match h {
                            true => Self::default_audit(),
                            false => Self::default(),
                        })
                    }
                    (None, Some(ra)) => Ok(Settings {
                        global_acc_sel: Some(ra),
                        ..Default::default()
                    }),
                    (None, None) => Ok(Settings::default()),
                }
            }
        };
        let accounts = cfg.transaction.accounts.names.iter().try_fold(
            HashMap::new(),
            |mut accs, account| match AccountTreeNode::from(account) {
                Ok(atn) => {
                    accs.insert(account.into(), Arc::new(atn));
                    Ok(accs)
                }
                Err(e) => {
                    let msg = format!("Invalid Chart of Accounts: {e}");
                    Err(msg)
                }
            },
        )?;

        let comms = cfg.transaction.commodities.names.iter().try_fold(
            HashMap::new(),
            |mut chm, comm| match Commodity::from(comm.to_string()) {
                Ok(c) => {
                    chm.insert(comm.into(), Arc::new(c));
                    Ok(chm)
                }
                Err(e) => {
                    let msg = format!("Invalid Chart of Commodities: {e}");
                    Err(msg)
                }
            },
        )?;

        let tags = cfg
            .transaction
            .tags
            .names
            .iter()
            .fold(HashMap::new(), |mut tags, tag| {
                let t = Tag::from(tag.to_string());
                tags.insert(tag.into(), Arc::new(t));
                tags
            });

        Ok(Settings {
            kernel: cfg.kernel.clone(),
            global_acc_sel: report_accounts,
            targets: cfg.report.targets.clone(),
            report: cfg.report,
            export: cfg.export,
            audit_mode: match audit_mode {
                Some(true) => true,
                Some(false) => false,
                None => cfg.kernel.audit.mode,
            },
            accounts,
            commodities: Commodities {
                names: comms,
                permit_empty_commodity: cfg
                    .transaction
                    .commodities
                    .permit_empty_commodity
                    .unwrap_or(false),
            },
            tags,
        })
    }
}
impl Settings {
    pub fn get_hash(&self) -> Option<Hash> {
        if self.audit_mode {
            Some(self.kernel.audit.hash.clone())
        } else {
            None
        }
    }

    pub fn get_txn_account(
        &mut self,
        name: &str,
        commodity: Arc<Commodity>,
    ) -> Result<TxnAccount, Box<dyn Error>> {
        let comm = self.get_commodity(Some(commodity.name.as_str()))?;

        match self.accounts.get(name) {
            Some(account_tree) => Ok(TxnAccount {
                atn: account_tree.clone(),
                comm,
            }),
            None => {
                if self.kernel.strict {
                    let msg = format!("Unknown account: '{}'", name);
                    Err(msg.into())
                } else {
                    let atn = Arc::new(AccountTreeNode::from(name)?);
                    self.accounts.insert(name.into(), atn.clone());
                    Ok(TxnAccount { atn, comm })
                }
            }
        }
    }

    pub fn get_commodity(&mut self, name: Option<&str>) -> Result<Arc<Commodity>, Box<dyn Error>> {
        match name {
            Some(n) => {
                if n.is_empty() {
                    if self.commodities.permit_empty_commodity {
                        return Ok(Arc::new(Commodity::default()));
                    } else {
                        let msg =
                            "Empty commodity and 'permit-empty-commodity' is not set".to_string();
                        return Err(msg.into());
                    }
                }
                match self.commodities.names.get(n) {
                    Some(comm) => Ok(comm.clone()),
                    None => {
                        if self.kernel.strict {
                            let msg = format!("Unknown commodity: '{}'", n);
                            Err(msg.into())
                        } else {
                            let comm = Arc::new(Commodity::from(n.into())?);
                            self.commodities.names.insert(n.into(), comm.clone());
                            Ok(comm)
                        }
                    }
                }
            }
            None => {
                let comm = Arc::new(Commodity::default());
                Ok(comm)
            }
        }
    }
    pub fn get_tag(&mut self, name: &str) -> Result<Arc<Tag>, Box<dyn Error>> {
        if name.is_empty() {
            let msg = "Tag name is empty string".to_string();
            return Err(msg.into());
        }
        match self.tags.get(name) {
            Some(tag) => Ok(tag.clone()),
            None => {
                if self.kernel.strict {
                    let msg = format!("Unknown tag: '{}'", name);
                    Err(msg.into())
                } else {
                    let tag = Arc::new(Tag::from(name));
                    self.tags.insert(name.into(), tag.clone());
                    Ok(tag)
                }
            }
        }
    }

    pub fn get_input_settings(
        &self,
        storage: Option<&String>,
        ref_path: Option<&Path>,
    ) -> Result<InputSettings, Box<dyn Error>> {
        let input = &self.kernel.input;

        let storage_type = match storage {
            Some(storage) => config::StorageType::from(storage.as_str())?,
            None => input.storage.clone(),
        };

        match storage_type {
            config::StorageType::FS => match &input.fs {
                Some(fs) => {
                    let dir = fs.dir.as_str();
                    let suffix = &fs.suffix;
                    let i = FsInput {
                        dir: match ref_path {
                            Some(p) => tackler_rs::get_abs_path(p, dir)?,
                            None => PathBuf::from(dir),
                        },
                        suffix: suffix.strip_prefix('.').unwrap_or(suffix.as_str()).into(),
                    };
                    Ok(InputSettings::Fs(i))
                }
                None => Err("Storage type 'fs' is not configured".into()),
            },
            config::StorageType::Git => match &input.git {
                Some(ref git) => {
                    let repo = git.repo.as_str();
                    let suffix = &git.suffix;
                    let i = GitInput {
                        repo: match ref_path {
                            Some(p) => tackler_rs::get_abs_path(p, repo)?,
                            None => PathBuf::from(repo),
                        },
                        git_ref: GitInputSelector::Reference(git.git_ref.clone()),
                        dir: git.dir.clone(),
                        ext: suffix.strip_prefix('.').unwrap_or(suffix.as_str()).into(),
                    };
                    Ok(InputSettings::Git(i))
                }
                None => Err("Storage type 'git' is not configured".into()),
            },
        }
    }
}

impl Settings {
    pub fn get_offset_datetime(
        &self,
        dt: PrimitiveDateTime,
    ) -> Result<OffsetDateTime, Box<dyn Error>> {
        let ts_tz = match self.kernel.timestamp.timezone {
            TimezoneType::Name(tz) => match dt.assume_timezone(tz) {
                OffsetResult::Some(ts) => ts,
                OffsetResult::Ambiguous(_, _) => {
                    let msg = format!("time conversion is ambiguous '{dt:?}'");
                    return Err(msg.into());
                }
                OffsetResult::None => {
                    let msg = format!("time is invalid '{dt:?}'");
                    return Err(msg.into());
                }
            },
            TimezoneType::Offset(tz) => dt.assume_offset(tz),
        };
        Ok(ts_tz)
    }
    pub fn get_offset_date(&self, date: Date) -> Result<OffsetDateTime, Box<dyn Error>> {
        let ts = PrimitiveDateTime::new(date, self.kernel.timestamp.default_time);
        self.get_offset_datetime(ts)
    }

    fn get_account_selector(&self, acc_sel: &AccountSelectors) -> AccountSelectors {
        match &self.global_acc_sel {
            Some(global_acc_sel) => global_acc_sel.clone(),
            None => acc_sel.clone(),
        }
    }

    pub fn get_report_targets(
        &self,
        arg_trgs: Option<Vec<String>>,
    ) -> Result<Vec<ReportType>, Box<dyn Error>> {
        match arg_trgs {
            Some(trgs) => config::to_report_targets(&trgs),
            None => Ok(self.targets.clone()),
        }
    }

    pub fn get_balance_ras(&self) -> AccountSelectors {
        self.get_account_selector(&self.report.balance.acc_sel)
    }

    pub fn get_balance_group_ras(&self) -> AccountSelectors {
        self.get_account_selector(&self.report.balance_group.acc_sel)
    }

    pub fn get_register_ras(&self) -> AccountSelectors {
        self.get_account_selector(&self.report.register.acc_sel)
    }

    pub fn get_equity_ras(&self) -> AccountSelectors {
        self.get_account_selector(&self.export.equity.acc_sel)
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn txnatn_atn() {
        let mut settings = Settings::default();

        let txnatn_1 =
            settings.get_txn_account("a:b:c", Arc::new(Commodity::default())).unwrap(/*:test:*/);

        assert_eq!(txnatn_1.atn.depth, 3);
        assert_eq!(txnatn_1.atn.get_root(), "a");
        assert_eq!(txnatn_1.atn.parent, "a:b");
        assert_eq!(txnatn_1.atn.account, "a:b:c");
        assert_eq!(txnatn_1.atn.get_name(), "c");

        let txnatn_2 =
            settings.get_txn_account("a:b:c", Arc::new(Commodity::default())).unwrap(/*:test:*/);

        assert_eq!(txnatn_2.atn.depth, 3);
        assert_eq!(txnatn_2.atn.get_root(), "a");
        assert_eq!(txnatn_2.atn.parent, "a:b");
        assert_eq!(txnatn_2.atn.account, "a:b:c");
        assert_eq!(txnatn_2.atn.get_name(), "c");

        assert_eq!(settings.accounts.len(), 1);
    }
}
