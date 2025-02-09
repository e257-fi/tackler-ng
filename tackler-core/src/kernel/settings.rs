/*
 * Tackler-NG 2023-2025
 * SPDX-License-Identifier: Apache-2.0
 */
use crate::config::overlaps::OverlapConfig;
use crate::config::{
    AccountSelectors, Config, Export, ExportType, Kernel, PriceLookupType, Report, ReportType,
};
use crate::kernel::hash::Hash;
use crate::kernel::price_lookup::PriceLookup;
use crate::model::TxnAccount;
use crate::model::price_entry::PriceDb;
use crate::model::{AccountTreeNode, Commodity};
use crate::parser::GitInputSelector;
use crate::{config, parser};
use jiff::Zoned;
use std::collections::HashMap;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tackler_api::txn_header::Tag;
use tackler_api::txn_ts::GroupBy;

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
        let has_parent = other_account_tree.is_some_and(|a| a.contains_key(parent))
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

#[derive(Debug, Default)]
pub struct Price {
    // todo: fix visibility
    pub price_db: PriceDb,
    pub lookup_type: PriceLookupType,
}

#[derive(Debug)]
pub struct Settings {
    pub(crate) audit_mode: bool,
    pub(crate) report: Report,
    pub(crate) export: Export,
    strict_mode: bool,
    kernel: Kernel,
    pub price: Price,
    price_lookup: PriceLookup,
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
            price: Price::default(),
            price_lookup: PriceLookup::default(),
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
            audit_mode: true,
            ..Self::default()
        }
    }
}

impl Settings {
    pub fn try_from(cfg: Config, overlaps: OverlapConfig) -> Result<Settings, Box<dyn Error>> {
        let strict_mode = overlaps.strict.mode.unwrap_or(cfg.kernel.strict);
        let audit_mode = overlaps.audit.mode.unwrap_or(cfg.kernel.audit.mode);

        let lookup_type = overlaps.price.lookup_type.unwrap_or(cfg.price.lookup_type);

        let db_path = overlaps.price.db_path.unwrap_or(cfg.price.db_path.clone());

        let account_trees = AccountTrees::from(&cfg.transaction.accounts.names, strict_mode)?;

        let mut commodities = Commodities::from(&cfg)?;

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

        let cfg_rpt_commodity = cfg
            .report
            .commodity
            .map(|c| {
                Self::inner_get_or_create_commodity(
                    &mut commodities,
                    strict_mode,
                    Some(c.name.as_str()),
                )
            })
            .transpose()?;

        let report_commodity = match overlaps.report.commodity {
            Some(c) => Some(Self::inner_get_or_create_commodity(
                &mut commodities,
                strict_mode,
                Some(c.as_str()),
            )?),
            None => cfg_rpt_commodity,
        };

        if report_commodity.is_none() && lookup_type != PriceLookupType::None {
            let msg =
                "Price conversion is activated, but there is no `report.commodity`".to_string();
            return Err(msg.into());
        }

        let group_by = overlaps
            .report
            .group_by
            .map(|g| GroupBy::from(g.as_str()))
            .unwrap_or(Ok(cfg.report.balance_group.group_by))?;

        let mut tmp_settings = Settings {
            strict_mode,
            audit_mode,
            kernel: cfg.kernel,
            price: Price::default(), // this is not real, see next one
            price_lookup: PriceLookup::default(), // this is not real, see next one
            global_acc_sel: overlaps.report.account_overlap,
            targets: cfg.report.targets.clone(),
            report: Report {
                commodity: report_commodity,
                ..cfg.report
            },
            export: cfg.export,
            accounts: account_trees,
            commodities,
            tags,
        };
        tmp_settings.report.balance_group.group_by = group_by;

        let given_time = overlaps.price.before_time;

        fn check_given_time_usage(
            gt: &Option<String>,
            plt: &PriceLookupType,
        ) -> Result<(), Box<dyn Error>> {
            if gt.is_some() {
                let msg = format!(
                    "Price \"before timestamp\" is not allowed when price lookup type is \"{}\"",
                    plt
                );
                return Err(msg.into());
            }
            Ok(())
        }
        let price_lookup = match lookup_type {
            ref plt @ PriceLookupType::LastPrice => {
                check_given_time_usage(&given_time, plt)?;
                PriceLookup::LastPriceDbEntry
            }
            ref plt @ PriceLookupType::TxnTime => {
                check_given_time_usage(&given_time, plt)?;
                PriceLookup::AtTheTimeOfTxn
            }
            ref plt @ PriceLookupType::GivenTime => match given_time {
                Some(ts) => tmp_settings
                    .parse_timestamp(ts.as_str())
                    .map(PriceLookup::GivenTime)?,
                None => {
                    let msg = format!(
                        "Price lookup type is \"{}\" and there is no timestamp given",
                        plt
                    );
                    return Err(msg.into());
                }
            },
            ref plt @ PriceLookupType::None => {
                check_given_time_usage(&given_time, plt)?;
                PriceLookup::None
            }
        };

        let price = match &lookup_type {
            PriceLookupType::None => Price::default(),
            _ => Price {
                // we need half-baked settings here bc commodity and timestamp lookups
                price_db: parser::pricedb_from_file(&db_path, &mut tmp_settings)?,
                lookup_type,
            },
        };

        Ok(Settings {
            price,
            price_lookup,
            ..tmp_settings
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

    pub fn get_commodity(&self, name: &str) -> Result<Arc<Commodity>, Box<dyn Error>> {
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
        Self::inner_get_or_create_commodity(&mut self.commodities, self.strict_mode, name)
    }

    fn inner_get_or_create_commodity(
        commodities: &mut Commodities,
        strict_mode: bool,
        name: Option<&str>,
    ) -> Result<Arc<Commodity>, Box<dyn Error>> {
        match name {
            Some(n) => {
                if n.is_empty() {
                    if commodities.permit_empty_commodity {
                        return match commodities.names.get(n) {
                            Some(c) => Ok(c.clone()),
                            None => {
                                let comm = Arc::new(Commodity::default());
                                commodities.names.insert(n.into(), comm.clone());

                                Ok(comm.clone())
                            }
                        };
                    } else {
                        let msg =
                            "Empty commodity and 'permit-empty-commodity' is not set".to_string();
                        return Err(msg.into());
                    }
                }
                match commodities.names.get(n) {
                    Some(comm) => Ok(comm.clone()),
                    None => {
                        if strict_mode {
                            let msg = format!("Unknown commodity: '{}'", n);
                            Err(msg.into())
                        } else {
                            let comm = Arc::new(Commodity::from(n.into())?);
                            commodities.names.insert(n.into(), comm.clone());
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

    pub fn get_price_lookup(&self) -> PriceLookup {
        self.price_lookup.clone()
    }

    pub fn get_input_settings(
        &self,
        storage: Option<&String>,
        ref_path: Option<&Path>,
    ) -> Result<InputSettings, Box<dyn Error>> {
        let input = &self.kernel.input;

        let storage_type = match storage {
            Some(storage) => config::StorageType::from(storage.as_str())?,
            None => input.storage,
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
                Some(git) => {
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
    pub fn parse_timestamp(&mut self, ts: &str) -> Result<Zoned, Box<dyn Error>> {
        Ok(winnow::Parser::parse(
            &mut crate::parser::parts::timestamp::parse_timestamp,
            winnow::Stateful {
                input: ts,
                state: self,
            },
        )
        .map_err(|e| e.to_string())?)
    }

    pub fn get_offset_datetime(
        &self,
        dt: jiff::civil::DateTime,
    ) -> Result<jiff::Zoned, Box<dyn Error>> {
        match dt.to_zoned(self.kernel.timestamp.timezone.clone()) {
            Ok(ts) => Ok(ts),
            Err(err) => {
                let msg = format!("time is invalid '{:?}'", err);
                Err(msg.into())
            }
        }
    }
    pub fn get_offset_date(&self, date: jiff::civil::Date) -> Result<jiff::Zoned, Box<dyn Error>> {
        let ts = date.to_datetime(self.kernel.timestamp.default_time);
        match ts.to_zoned(self.kernel.timestamp.timezone.clone()) {
            Ok(ts) => Ok(ts),
            Err(err) => {
                let msg = format!("time is invalid '{:?}'", err);
                Err(msg.into())
            }
        }
    }

    pub fn get_report_commodity(&self) -> Option<Arc<Commodity>> {
        self.report.commodity.as_ref().map(|c| c.clone())
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
        assert!(
            settings
                .get_or_create_txn_account("a:b", comm.clone())
                .is_err()
        );
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
        assert!(
            settings
                .get_or_create_txn_account("a:b:c", comm.clone())
                .is_err()
        );

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
