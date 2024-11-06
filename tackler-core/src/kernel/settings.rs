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
use crate::kernel::config::{Config, Kernel, Report};
use crate::kernel::hash::Hash;
use crate::model::TxnAccount;
use crate::model::{AccountTreeNode, Commodity};
use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;
use tackler_api::txn_header::Tag;
use time::{format_description, Date, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset};
use time_tz::{timezones, OffsetResult, PrimitiveDateTimeExt, Tz};

#[derive(Clone, Default)]
pub struct AuditSettings {
    pub(crate) hash: Option<Hash>,
}

#[derive(Debug, Default)]
struct Commodities {
    names: HashMap<String, Rc<Commodity>>,
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

enum TimezoneType {
    Offset(UtcOffset),
    Name(&'static Tz),
}
pub struct Settings {
    pub kernel: Kernel,
    timezone: TimezoneType,
    default_time: Time,
    pub audit: AuditSettings,
    pub report: Report,
    accounts: HashMap<String, Rc<AccountTreeNode>>,
    commodities: Commodities,
    tags: HashMap<String, Rc<Tag>>,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            kernel: Kernel::default(),
            default_time: Time::MIDNIGHT,
            timezone: TimezoneType::Offset(UtcOffset::UTC),
            report: Report { accounts: None },
            audit: AuditSettings { hash: None },
            accounts: HashMap::new(),
            commodities: Commodities::default_empty_ok(),
            tags: HashMap::new(),
        }
    }
}

impl Settings {
    pub fn default_audit() -> Self {
        Settings {
            kernel: Kernel::default(),
            default_time: Time::MIDNIGHT,
            timezone: TimezoneType::Offset(UtcOffset::UTC),
            report: Report { accounts: None },
            audit: AuditSettings {
                hash: Some(Hash::default()),
            },
            accounts: HashMap::new(),
            commodities: Commodities::default_empty_ok(),
            tags: HashMap::new(),
        }
    }
}

impl Settings {
    pub fn from(
        cfg_opt: Option<Config>,
        hash: Option<bool>,
        report_accounts: Option<Vec<String>>,
    ) -> Result<Settings, Box<dyn Error>> {
        let cfg = match cfg_opt {
            Some(c) => c,
            None => {
                return match (hash, report_accounts) {
                    (Some(h), Some(ra)) => {
                        let mut s = match h {
                            true => Self::default_audit(),
                            false => Self::default(),
                        };
                        s.report.accounts = Some(ra);
                        Ok(s)
                    }
                    (Some(h), None) => {
                        return Ok(match h {
                            true => Self::default_audit(),
                            false => Self::default(),
                        })
                    }
                    (None, Some(ra)) => {
                        let mut s = Self::default();
                        s.report.accounts = Some(ra);
                        Ok(s)
                    }
                    (None, None) => Ok(Settings::default()),
                }
            }
        };
        let accs = cfg.transaction.accounts.names.iter().try_fold(
            HashMap::new(),
            |mut accs, account| match AccountTreeNode::from(account) {
                Ok(atn) => {
                    accs.insert(account.into(), Rc::new(atn));
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
                    chm.insert(comm.into(), Rc::new(c));
                    Ok(chm)
                }
                Err(e) => {
                    let msg = format!("Invalid Chart of Commodities: {e}");
                    Err(msg)
                }
            },
        )?;

        Ok(Settings {
            kernel: cfg.kernel.clone(),
            default_time: {
                let t = &cfg.kernel.timestamp.default_time;
                Time::from_hms_nano(t.hour, t.minute, t.second, t.nanosecond)?
            },
            timezone: {
                match (
                    &cfg.kernel.timestamp.timezone.name,
                    &cfg.kernel.timestamp.timezone.offset,
                ) {
                    (Some(_), Some(_)) => {
                        let msg =
                            "kernel.timezone: 'name' and 'offset' are both defined".to_string();
                        return Err(msg.into());
                    }
                    (None, None) => TimezoneType::Name(
                        timezones::get_by_name("UTC").ok_or("Undefined default (UTC) timezone")?,
                    ),
                    (Some(tz_name), None) => TimezoneType::Name(
                        timezones::get_by_name(tz_name)
                            .ok_or(format!("Unknown timezone '{tz_name}'"))?,
                    ),
                    (None, Some(offset)) => {
                        let offset_format =
                            format_description::parse("[offset_hour]:[offset_minute]")?;
                        let offset = UtcOffset::parse(offset, &offset_format)?;
                        TimezoneType::Offset(offset)
                    }
                }
            },
            report: Report {
                accounts: report_accounts,
            },
            audit: match hash {
                Some(true) => {
                    let hasher = match &cfg.kernel.audit {
                        Some(audit) => Some(Hash::from(audit.hash.as_str())?),
                        _ => {
                            let msg = "kernel.audit.hash is not configured".to_string();
                            return Err(msg.into());
                        }
                    };
                    AuditSettings { hash: hasher }
                }
                Some(false) => AuditSettings::default(),
                None => {
                    let hasher = match &cfg.kernel.audit {
                        Some(audit) => {
                            if audit.mode {
                                Some(Hash::from(audit.hash.as_str())?)
                            } else {
                                None
                            }
                        }
                        _ => None,
                    };
                    AuditSettings { hash: hasher }
                }
            },
            accounts: accs,
            commodities: Commodities {
                names: comms,
                permit_empty_commodity: cfg
                    .transaction
                    .commodities
                    .permit_empty_commodity
                    .unwrap_or(false),
            },
            tags: HashMap::new(), //todo: implement
        })
    }
}
impl Settings {
    pub fn get_hash(&self) -> Option<Hash> {
        self.audit.hash.clone()
    }

    pub fn get_txn_account(
        &mut self,
        name: &str,
        commodity: Rc<Commodity>,
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
                    let atn = Rc::new(AccountTreeNode::from(name)?);
                    self.accounts.insert(name.into(), atn.clone());
                    Ok(TxnAccount { atn, comm })
                }
            }
        }
    }

    pub fn get_commodity(&mut self, name: Option<&str>) -> Result<Rc<Commodity>, Box<dyn Error>> {
        match name {
            Some(n) => {
                if n.is_empty() {
                    if self.commodities.permit_empty_commodity {
                        return Ok(Rc::new(Commodity::default()));
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
                            let comm = Rc::new(Commodity::from(n.into())?);
                            self.commodities.names.insert(n.into(), comm.clone());
                            Ok(comm)
                        }
                    }
                }
            }
            None => {
                let comm = Rc::new(Commodity::default());
                Ok(comm)
            }
        }
    }
    pub fn get_tag(&mut self, name: &str) -> Result<Rc<Tag>, Box<dyn Error>> {
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
                    let tag = Rc::new(Tag::from(name));
                    self.tags.insert(name.into(), tag.clone());
                    Ok(tag)
                }
            }
        }
    }
}

impl Settings {
    pub fn get_offset_datetime(
        &self,
        dt: PrimitiveDateTime,
    ) -> Result<OffsetDateTime, Box<dyn Error>> {
        let ts_tz = match self.timezone {
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
        let ts = PrimitiveDateTime::new(date, self.default_time);
        self.get_offset_datetime(ts)
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
