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
use crate::config::AccountSelectors;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub(super) struct ConfigRaw {
    pub(super) kernel: KernelRaw,
    pub(super) transaction: TransactionRaw,
    pub(super) report: ReportRaw,
    pub(super) export: ExportRaw,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub(super) struct KernelRaw {
    pub(super) strict: bool,
    pub(super) timestamp: TimestampRaw,
    pub(super) audit: AuditRaw,
    pub(super) input: InputRaw,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub(super) struct TimestampRaw {
    #[serde(rename = "default-time")]
    pub(super) default_time: toml::value::Time,
    pub(super) timezone: TimezoneRaw,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub(super) struct TimezoneRaw {
    pub(super) name: Option<String>,
    pub(super) offset: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct AuditRaw {
    pub(super) hash: String,
    pub(super) mode: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct InputRaw {
    pub(super) storage: String,
    pub(super) fs: Option<FsRaw>,
    pub(super) git: Option<GitRaw>,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct FsRaw {
    pub(super) dir: String,
    pub(super) suffix: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub(super) struct GitRaw {
    #[serde(rename = "repository")]
    pub(super) repo: String,
    #[serde(rename = "ref")]
    pub(super) git_ref: String,
    pub(super) dir: String,
    pub(super) suffix: String,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct TransactionRaw {
    pub(super) accounts: AccountsPathRaw,
    pub(super) commodities: CommoditiesPathRaw,
    pub(super) tags: TagsPathRaw,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct AccountsPathRaw {
    pub(super) path: String,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct CommoditiesPathRaw {
    pub(super) path: String,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct TagsPathRaw {
    pub(super) path: String,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct AccountsRaw {
    #[serde(rename = "accounts")]
    pub(super) names: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct CommoditiesRaw {
    #[serde(rename = "permit-empty-commodity")]
    pub(crate) permit_empty_commodity: Option<bool>,

    #[serde(rename = "commodities")]
    pub(crate) names: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct TagsRaw {
    #[serde(rename = "tags")]
    pub(crate) names: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct ReportRaw {
    #[serde(rename = "report-timezone")]
    pub(super) report_tz: String,
    pub(super) targets: Vec<String>,
    pub(super) accounts: Option<Vec<String>>,
    pub(super) scale: ScaleRaw,
    pub(super) register: RegisterRaw,
    #[serde(rename = "balance-group")]
    pub(super) balance_group: BalanceGroupRaw,
    pub(super) balance: BalanceRaw,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct ScaleRaw {
    pub(super) min: u32,
    pub(super) max: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct BalanceRaw {
    pub(super) title: String,
    #[serde(rename = "accounts")]
    pub(super) acc_sel: Option<AccountSelectors>,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct BalanceGroupRaw {
    pub(super) title: String,
    #[serde(rename = "group-by")]
    pub(super) group_by: String,
    #[serde(rename = "accounts")]
    pub(super) acc_sel: Option<AccountSelectors>,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct RegisterRaw {
    pub(super) title: String,
    #[serde(rename = "timestamp-style")]
    pub(super) timestamp_style: Option<String>,
    #[serde(rename = "accounts")]
    pub(super) acc_sel: Option<AccountSelectors>,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct ExportRaw {
    pub(super) equity: EquityRaw,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct EquityRaw {
    #[serde(rename = "equity-account")]
    pub(super) equity_account: String,
    #[serde(rename = "accounts")]
    pub(super) acc_sel: Option<AccountSelectors>,
}
