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
use crate::kernel::hash::Hash;
use crate::model::TxnAccount;
use crate::model::{AccountTreeNode, Commodity};
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::rc::Rc;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Accounts {
    names: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Commodities {
    permit_empty_commodity: bool,
    names: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub accounts: Accounts,
    pub commodities: Commodities,
}

#[derive(Debug, Clone, Default)]
pub struct Audit {
    pub hash: Option<Hash>,
}

#[derive(Debug)]
pub struct Report {
    pub accounts: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct Settings {
    pub basedir: Box<Path>,
    pub audit: Audit,
    pub report: Report,
    accounts: HashMap<String, Rc<AccountTreeNode>>,
    commodities: HashMap<String, Rc<Commodity>>,
}

impl Settings {
    pub fn from(
        cfg: &Config,
        hash: Option<Hash>,
        report_accounts: Option<Vec<String>>,
    ) -> Result<Settings, Box<dyn Error>> {
        let accs = cfg
            .accounts
            .names
            .iter()
            .fold(HashMap::new(), |mut accs, acc| {
                let atn = Rc::new(AccountTreeNode::from(acc).unwrap());
                accs.insert(acc.into(), atn);
                accs
            });

        let comms = cfg
            .commodities
            .names
            .iter()
            .fold(HashMap::new(), |mut chm, acc| {
                chm.insert(
                    acc.into(),
                    Rc::new(Commodity::from(acc.to_string()).unwrap()),
                );
                chm
            });

        Ok(Settings {
            basedir: PathBuf::new().into_boxed_path(),
            report: Report {
                accounts: report_accounts,
            },
            audit: Audit { hash },
            accounts: accs,
            commodities: comms,
        })
    }

    pub fn default_audit() -> Self {
        Settings {
            basedir: PathBuf::default().into_boxed_path(),
            report: Report { accounts: None },
            audit: Audit {
                hash: Some(Hash::default()),
            },
            accounts: HashMap::new(),
            commodities: HashMap::new(),
        }
    }

    pub fn get_txn_account(
        &mut self,
        name: &str,
        commodity: Rc<Commodity>,
    ) -> Result<TxnAccount, Box<dyn Error>> {
        // todo: check cfg.strict => account is defined
        let comm = self.get_commodity(Some(commodity.name.as_str()))?;

        match self.accounts.get(name) {
            Some(account_tree) => Ok(TxnAccount {
                atn: account_tree.clone(),
                comm,
            }),
            None => {
                //let msg = format!("Unknown account: '{}'", name);
                //Err(msg.into())

                let atn = Rc::new(AccountTreeNode::from(name)?);
                self.accounts.insert(name.into(), atn.clone());
                Ok(TxnAccount { atn, comm })
            }
        }
    }

    pub fn get_commodity(&mut self, name: Option<&str>) -> Result<Rc<Commodity>, Box<dyn Error>> {
        // todo: check cfg.strict => commodity is defined
        match name {
            Some(n) => {
                if n.is_empty() {
                    return Ok(Rc::new(Commodity::default()));
                }
                match self.commodities.get(n) {
                    Some(comm) => Ok(comm.clone()),
                    None => {
                        //let msg = format!("Unknown commodity: '{}'", n);
                        //Err(msg.into())
                        let comm = Rc::new(Commodity::from(n.into())?);
                        self.commodities.insert(n.into(), comm.clone());
                        Ok(comm)
                    }
                }
            }
            None => {
                let comm = Rc::new(Commodity::default());
                Ok(comm)
            }
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            basedir: PathBuf::default().into_boxed_path(),
            report: Report { accounts: None },
            audit: Audit { hash: None },
            accounts: HashMap::new(),
            commodities: HashMap::new(),
        }
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
            settings.get_txn_account("a:b:c", Rc::new(Commodity::default())).unwrap(/*:test:*/);

        assert_eq!(txnatn_1.atn.depth, 3);
        assert_eq!(txnatn_1.atn.get_root(), "a");
        assert_eq!(txnatn_1.atn.parent, "a:b");
        assert_eq!(txnatn_1.atn.account, "a:b:c");
        assert_eq!(txnatn_1.atn.get_name(), "c");

        let txnatn_2 =
            settings.get_txn_account("a:b:c", Rc::new(Commodity::default())).unwrap(/*:test:*/);

        assert_eq!(txnatn_2.atn.depth, 3);
        assert_eq!(txnatn_2.atn.get_root(), "a");
        assert_eq!(txnatn_2.atn.parent, "a:b");
        assert_eq!(txnatn_2.atn.account, "a:b:c");
        assert_eq!(txnatn_2.atn.get_name(), "c");

        assert_eq!(settings.accounts.len(), 1);
    }
}
