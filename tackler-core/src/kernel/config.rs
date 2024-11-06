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
use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::path::Path;
use tackler_rs::get_abs_path;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Config {
    pub(crate) kernel: Kernel,
    pub(crate) transaction: Transaction,
}

impl Config {
    pub fn from<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn Error>> {
        let cfg: ConfigRaw = toml::from_str(fs::read_to_string(&path)?.as_str())?;

        let accs_path = get_abs_path(&path, cfg.transaction.accounts.file.as_str())?;
        let accounts: Accounts = toml::from_str(fs::read_to_string(accs_path)?.as_str())?;

        let comms_path = get_abs_path(&path, cfg.transaction.commodities.file.as_str())?;
        let commodities: Commodities = toml::from_str(fs::read_to_string(comms_path)?.as_str())?;

        let tags_path = get_abs_path(&path, cfg.transaction.tags.file.as_str())?;
        let tags: Tags = toml::from_str(fs::read_to_string(tags_path)?.as_str())?;

        Ok(Config {
            kernel: cfg.kernel,
            transaction: Transaction {
                accounts,
                commodities,
                tags,
            },
        })
    }
}
#[derive(Debug, Deserialize)]
struct ConfigRaw {
    kernel: Kernel,
    transaction: TransactionRaw,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct Kernel {
    pub(crate) strict: bool,
    pub(crate) timestamp: Timestamp,
    pub(crate) audit: Option<Audit>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct Timestamp {
    #[serde(rename = "default-time")]
    pub(crate) default_time: toml::value::Time,
    pub(crate) timezone: Timezone,
}
impl Default for Timestamp {
    fn default() -> Self {
        Timestamp {
            default_time: toml::value::Time {
                hour: 0,
                minute: 0,
                second: 0,
                nanosecond: 0,
            },
            timezone: Timezone::default(),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct Timezone {
    pub(crate) name: Option<String>,
    pub(crate) offset: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct Audit {
    pub(crate) hash: String,
    pub(crate) mode: bool,
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
    pub(crate) names: Vec<String>,
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

#[derive(Debug)]
pub struct Report {
    pub accounts: Option<Vec<String>>,
}
