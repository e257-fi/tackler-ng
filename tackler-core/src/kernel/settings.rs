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
use crate::config::{
    AccountSelectors, Config, Export, ExportType, Kernel, Report, ReportType, TimezoneType,
};
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

    fn from(cfg: &Config) -> Result<Commodities, Box<dyn Error>> {
        let cfg_comm = &cfg.transaction.commodities;
        let permit_empty_commodity = cfg_comm.permit_empty_commodity.unwrap_or(false);

        let comms =
            cfg_comm.names.iter().try_fold(
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
        Ok(Commodities {
            names: comms,
            permit_empty_commodity,
        })
    }
}

#[derive(Debug, Default)]
struct AccountTrees {
    defined_accounts: HashMap<String, Arc<AccountTreeNode>>,
    synthetic_parents: HashMap<String, Arc<AccountTreeNode>>,
}

impl AccountTrees {
    fn build_account_tree(
        target_account_tree: &mut HashMap<String, Arc<AccountTreeNode>>,
        atn: Arc<AccountTreeNode>,
        other_account_tree: Option<&HashMap<String, Arc<AccountTreeNode>>>,
    ) -> Result<(), Box<dyn Error>> {
        let parent = atn.parent.as_str();
        let has_parent = other_account_tree.map_or(false, |a| a.contains_key(parent))
            || target_account_tree.contains_key(parent);

        if has_parent || atn.is_root() {
            // this breaks recursion
            Ok(())
        } else {
            let parent_atn =
                Arc::new(AccountTreeNode::from(parent).expect("IE: synthetic parent is invalid"));
            target_account_tree.insert(parent.to_string(), parent_atn.clone());

            Self::build_account_tree(target_account_tree, parent_atn, other_account_tree)
        }
    }

    fn from(account_names: &[String], strict_mode: bool) -> Result<AccountTrees, Box<dyn Error>> {
        let defined_accounts =
            account_names
                .iter()
                .try_fold(
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

        let synthetic_parents = if strict_mode {
            // Synthetic Account Parents are only needed in strict mode
            let mut sap = HashMap::new();
            for atn_entry in defined_accounts.iter() {
                if !&defined_accounts.contains_key(atn_entry.1.parent.as_str()) {
                    // Parent is missing -> Let's build synthetic tree
                    let (_, atn) = atn_entry;
                    Self::build_account_tree(&mut sap, atn.clone(), Some(&defined_accounts))?;
                }
            }
            sap
        } else {
            HashMap::new()
        };
        Ok(AccountTrees {
            defined_accounts,
            synthetic_parents,
        })
    }
}

#[derive(Debug)]
pub struct Settings {
    pub(crate) audit_mode: bool,
    pub(crate) report: Report,
    pub(crate) export: Export,
    strict_mode: bool,
    kernel: Kernel,
    global_acc_sel: Option<AccountSelectors>,
    targets: Vec<ReportType>,
    accounts: AccountTrees,
    commodities: Commodities,
    tags: HashMap<String, Arc<Tag>>,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            strict_mode: false,
            audit_mode: false,
            report: Report::default(),
            export: Export::default(),
            kernel: Kernel::default(),
            global_acc_sel: None,
            targets: Vec::new(),
            accounts: AccountTrees::default(),
            commodities: Commodities::default_empty_ok(),
            tags: HashMap::new(),
        }
    }
}

impl Settings {
    pub fn default_audit() -> Self {
        Settings {
            strict_mode: false,
            audit_mode: true,
            report: Report::default(),
            export: Export::default(),

            kernel: Kernel::default(),
            global_acc_sel: None,
            targets: Vec::new(),
            accounts: AccountTrees::default(),
            commodities: Commodities::default_empty_ok(),
            tags: HashMap::new(),
        }
    }
}

impl Settings {
    pub fn from(
        cfg: Config,
        strict_mode_opt: Option<bool>,
        audit_mode_opt: Option<bool>,
        report_accounts: Option<Vec<String>>,
    ) -> Result<Settings, Box<dyn Error>> {
        let strict_mode = match strict_mode_opt {
            Some(s) => s,
            None => cfg.kernel.strict,
        };
        let audit_mode = match audit_mode_opt {
            Some(a) => a,
            None => cfg.kernel.audit.mode,
        };

        let account_trees = AccountTrees::from(&cfg.transaction.accounts.names, strict_mode)?;

        let commodities = Commodities::from(&cfg)?;

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
            strict_mode,
            audit_mode,
            kernel: cfg.kernel,
            global_acc_sel: report_accounts,
            targets: cfg.report.targets.clone(),
            report: cfg.report,
            export: cfg.export,
            accounts: account_trees,
            commodities,
            tags,
        })
    }
}
impl Settings {
    pub(crate) fn get_hash(&self) -> Option<Hash> {
        if self.audit_mode {
            Some(self.kernel.audit.hash.clone())
        } else {
            None
        }
    }

    pub(crate) fn get_txn_account(
        &self,
        name: &str,
        commodity: Arc<Commodity>,
    ) -> Result<TxnAccount, Box<dyn Error>> {
        let comm = self.get_commodity(commodity.name.as_str())?;

        match self.accounts.defined_accounts.get(name) {
            Some(account_tree) => Ok(TxnAccount {
                atn: account_tree.clone(),
                comm,
            }),
            None => {
                if let Some(acc_parent) = self.accounts.synthetic_parents.get(name) {
                    Ok(TxnAccount {
                        atn: acc_parent.clone(),
                        comm,
                    })
                } else {
                    let msg = format!("gta: Unknown account: '{}'", name);
                    Err(msg.into())
                }
            }
        }
    }

    pub(crate) fn get_or_create_txn_account(
        &mut self,
        name: &str,
        commodity: Arc<Commodity>,
    ) -> Result<TxnAccount, Box<dyn Error>> {
        let comm = self.get_or_create_commodity(Some(commodity.name.as_str()))?;

        let strict_mode = self.strict_mode;
        let atn_opt = self.accounts.defined_accounts.get(name).cloned();

        let atn = match atn_opt {
            Some(account_tree) => TxnAccount {
                atn: account_tree.clone(),
                comm,
            },
            None => {
                if self.strict_mode {
                    let msg = format!("Unknown account: '{}'", name);
                    return Err(msg.into());
                } else {
                    let atn = Arc::new(AccountTreeNode::from(name)?);
                    self.accounts
                        .defined_accounts
                        .insert(name.into(), atn.clone());
                    AccountTrees::build_account_tree(
                        &mut self.accounts.defined_accounts,
                        atn.clone(),
                        None,
                    )?;

                    TxnAccount { atn, comm }
                }
            }
        };
        if !strict_mode {
            // Not strict mode, so we build the (missing) parents
            // directly into main Chart of Accounts
            AccountTrees::build_account_tree(
                &mut self.accounts.defined_accounts,
                atn.atn.clone(),
                None,
            )?;
        }

        Ok(atn)
    }

    pub(crate) fn get_commodity(&self, name: &str) -> Result<Arc<Commodity>, Box<dyn Error>> {
        match self.commodities.names.get(name) {
            Some(comm) => Ok(comm.clone()),
            None => {
                let msg = format!("Unknown commodity: '{}'", name);
                Err(msg.into())
            }
        }
    }

    pub(crate) fn get_or_create_commodity(
        &mut self,
        name: Option<&str>,
    ) -> Result<Arc<Commodity>, Box<dyn Error>> {
        match name {
            Some(n) => {
                if n.is_empty() {
                    if self.commodities.permit_empty_commodity {
                        return match self.commodities.names.get(n) {
                            Some(c) => Ok(c.clone()),
                            None => {
                                let comm = Arc::new(Commodity::default());
                                self.commodities.names.insert(n.into(), comm.clone());

                                Ok(comm.clone())
                            }
                        };
                    } else {
                        let msg =
                            "Empty commodity and 'permit-empty-commodity' is not set".to_string();
                        return Err(msg.into());
                    }
                }
                match self.commodities.names.get(n) {
                    Some(comm) => Ok(comm.clone()),
                    None => {
                        if self.strict_mode {
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

    pub(crate) fn get_or_create_tag(&mut self, name: &str) -> Result<Arc<Tag>, Box<dyn Error>> {
        if name.is_empty() {
            let msg = "Tag name is empty string".to_string();
            return Err(msg.into());
        }
        match self.tags.get(name) {
            Some(tag) => Ok(tag.clone()),
            None => {
                if self.strict_mode {
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

    pub fn get_report_targets(
        &self,
        arg_trgs: Option<Vec<String>>,
    ) -> Result<Vec<ReportType>, Box<dyn Error>> {
        match arg_trgs {
            Some(trgs) => config::to_report_targets(&trgs),
            None => Ok(self.targets.clone()),
        }
    }

    pub fn get_export_targets(
        &self,
        arg_trgs: Option<Vec<String>>,
    ) -> Result<Vec<ExportType>, Box<dyn Error>> {
        match arg_trgs {
            Some(trgs) => config::to_export_targets(&trgs),
            None => Ok(self.export.targets.clone()),
        }
    }

    fn get_account_selector(&self, acc_sel: &AccountSelectors) -> AccountSelectors {
        match &self.global_acc_sel {
            Some(global_acc_sel) => global_acc_sel.clone(),
            None => acc_sel.clone(),
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
    fn accounts_strict_false() {
        let comm = Arc::new(Commodity::default());
        let mut settings = Settings::default();

        let txntn_1 = settings.get_or_create_txn_account("a:b:c", comm.clone()).unwrap(/*:test:*/);
        assert_eq!(settings.accounts.defined_accounts.len(), 3);

        assert_eq!(txntn_1.atn.depth, 3);
        assert_eq!(txntn_1.atn.get_root(), "a");
        assert_eq!(txntn_1.atn.parent, "a:b");
        assert_eq!(txntn_1.atn.account, "a:b:c");
        assert_eq!(txntn_1.atn.get_name(), "c");

        let txntn_2 = settings.get_txn_account("a:b:c", comm.clone()).unwrap(/*:test:*/);
        assert_eq!(settings.accounts.defined_accounts.len(), 3);

        assert_eq!(txntn_2.atn.depth, 3);
        assert_eq!(txntn_2.atn.get_root(), "a");
        assert_eq!(txntn_2.atn.parent, "a:b");
        assert_eq!(txntn_2.atn.account, "a:b:c");
        assert_eq!(txntn_2.atn.get_name(), "c");

        let txntn_3 =
            settings.get_or_create_txn_account("a:b:b-leaf", comm.clone()).unwrap(/*:test:*/);
        assert_eq!(settings.accounts.defined_accounts.len(), 4);

        assert_eq!(txntn_3.atn.depth, 3);
        assert_eq!(txntn_3.atn.get_root(), "a");
        assert_eq!(txntn_3.atn.parent, "a:b");
        assert_eq!(txntn_3.atn.account, "a:b:b-leaf");
        assert_eq!(txntn_3.atn.get_name(), "b-leaf");
    }

    #[test]
    fn accounts_strict_true() {
        let comm = Arc::new(Commodity::default());
        let mut settings = Settings::default();
        let accounts = vec!["a:b:c".to_string()];

        let acc_trees = AccountTrees::from(&accounts, true).unwrap(/*:test:*/);
        settings.accounts = acc_trees;
        settings.strict_mode = true;

        assert_eq!(settings.accounts.defined_accounts.len(), 1);
        assert_eq!(settings.accounts.synthetic_parents.len(), 2);

        let txntn_1 = settings.get_or_create_txn_account("a:b:c", comm.clone()).unwrap(/*:test:*/);
        assert_eq!(settings.accounts.defined_accounts.len(), 1);
        assert_eq!(settings.accounts.synthetic_parents.len(), 2);

        assert_eq!(txntn_1.atn.depth, 3);
        assert_eq!(txntn_1.atn.get_root(), "a");
        assert_eq!(txntn_1.atn.parent, "a:b");
        assert_eq!(txntn_1.atn.account, "a:b:c");
        assert_eq!(txntn_1.atn.get_name(), "c");

        let txntn_2 = settings.get_txn_account("a:b:c", comm.clone()).unwrap(/*:test:*/);
        assert_eq!(settings.accounts.defined_accounts.len(), 1);
        assert_eq!(settings.accounts.synthetic_parents.len(), 2);

        assert_eq!(txntn_2.atn.depth, 3);
        assert_eq!(txntn_2.atn.get_root(), "a");
        assert_eq!(txntn_2.atn.parent, "a:b");
        assert_eq!(txntn_2.atn.account, "a:b:c");
        assert_eq!(txntn_2.atn.get_name(), "c");

        // Check that it won't create a synthetic account as real one
        assert!(settings
            .get_or_create_txn_account("a:b", comm.clone())
            .is_err());
        assert_eq!(settings.accounts.defined_accounts.len(), 1);
        assert_eq!(settings.accounts.synthetic_parents.len(), 2);

        // Check synthetic account
        let txntn_3 = settings.get_txn_account("a:b", comm.clone()).unwrap(/*:test:*/);
        assert_eq!(settings.accounts.defined_accounts.len(), 1);
        assert_eq!(settings.accounts.synthetic_parents.len(), 2);

        assert_eq!(txntn_3.atn.depth, 2);
        assert_eq!(txntn_3.atn.get_root(), "a");
        assert_eq!(txntn_3.atn.parent, "a");
        assert_eq!(txntn_3.atn.account, "a:b");
        assert_eq!(txntn_3.atn.get_name(), "b");

        // Check synthetic account
        let txntn_4 = settings.get_txn_account("a", comm.clone()).unwrap(/*:test:*/);
        assert_eq!(settings.accounts.defined_accounts.len(), 1);
        assert_eq!(settings.accounts.synthetic_parents.len(), 2);

        assert_eq!(txntn_4.atn.depth, 1);
        assert_eq!(txntn_4.atn.get_root(), "a");
        assert_eq!(txntn_4.atn.parent, "");
        assert_eq!(txntn_4.atn.account, "a");
        assert_eq!(txntn_4.atn.get_name(), "a");
    }

    #[test]
    fn accounts_strict_true_child_first() {
        let comm = Arc::new(Commodity::default());
        let mut settings = Settings::default();
        let accounts = vec!["a:b:c".to_string(), "a:b".to_string(), "a".to_string()];

        let acc_trees = AccountTrees::from(&accounts, true).unwrap(/*:test:*/);
        settings.accounts = acc_trees;
        settings.strict_mode = true;

        assert_eq!(settings.accounts.defined_accounts.len(), 3);
        assert_eq!(settings.accounts.synthetic_parents.len(), 0);

        let txntn_1 = settings.get_or_create_txn_account("a:b:c", comm.clone()).unwrap(/*:test:*/);
        assert_eq!(settings.accounts.defined_accounts.len(), 3);
        assert_eq!(settings.accounts.synthetic_parents.len(), 0);
        assert_eq!(txntn_1.atn.account, "a:b:c");

        let txntn_2 = settings.get_or_create_txn_account("a:b", comm.clone()).unwrap(/*:test:*/);
        assert_eq!(settings.accounts.defined_accounts.len(), 3);
        assert_eq!(settings.accounts.synthetic_parents.len(), 0);
        assert_eq!(txntn_2.atn.account, "a:b");

        let txntn_2 = settings.get_or_create_txn_account("a", comm.clone()).unwrap(/*:test:*/);
        assert_eq!(settings.accounts.defined_accounts.len(), 3);
        assert_eq!(settings.accounts.synthetic_parents.len(), 0);
        assert_eq!(txntn_2.atn.account, "a");
    }

    #[test]
    fn accounts_strict_true_gap() {
        let comm = Arc::new(Commodity::default());
        let mut settings = Settings::default();
        let accounts = vec!["a:b:c:d".to_string(), "a:b".to_string(), "a".to_string()];

        let acc_trees = AccountTrees::from(&accounts, true).unwrap(/*:test:*/);
        settings.accounts = acc_trees;
        settings.strict_mode = true;

        assert_eq!(settings.accounts.defined_accounts.len(), 3);
        assert_eq!(settings.accounts.synthetic_parents.len(), 1);

        // Check that it won't create a synthetic account as real one
        assert!(settings
            .get_or_create_txn_account("a:b:c", comm.clone())
            .is_err());

        let txntn_synth = settings.get_txn_account("a:b:c", comm.clone()).unwrap(/*:test:*/);
        assert_eq!(settings.accounts.defined_accounts.len(), 3);
        assert_eq!(settings.accounts.synthetic_parents.len(), 1);
        assert_eq!(txntn_synth.atn.account, "a:b:c");

        let txntn_2 = settings.get_or_create_txn_account("a:b", comm.clone()).unwrap(/*:test:*/);
        assert_eq!(settings.accounts.defined_accounts.len(), 3);
        assert_eq!(settings.accounts.synthetic_parents.len(), 1);
        assert_eq!(txntn_2.atn.account, "a:b");

        let txntn_2 = settings.get_or_create_txn_account("a", comm.clone()).unwrap(/*:test:*/);
        assert_eq!(settings.accounts.defined_accounts.len(), 3);
        assert_eq!(settings.accounts.synthetic_parents.len(), 1);
        assert_eq!(txntn_2.atn.account, "a");
    }
}
