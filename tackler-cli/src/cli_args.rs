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
use clap::builder::PossibleValue;
use clap::{ArgGroup, Parser};
use std::path::PathBuf;
use tackler_api::txn_ts;

#[derive(Parser)]
#[command(author, version=env!("VERSION"), about, long_about = None)]
#[command(group(
            ArgGroup::new("input")
                .required(true)
                .args(["input_filename", "input_fs_dir", "input_git_repo"]),
        ))]
pub(crate) struct Cli {
    /// Enable audit mode
    ///
    /// Audit mode turns on validation of journal data and
    /// produces checksums of data and used selectors.
    #[arg(long = "audit.mode", value_name = "true|false")]
    pub(crate) audit_mode: Option<bool>,

    /// Path to single transaction journal file
    #[arg(long="input.file",
        value_name = "txn file path",
        conflicts_with_all([
            "input_fs_dir",
            "input_fs_ext",
            "input_git_repo",
            "input_git_ref",
            "input_git_dir"])
    )]
    pub(crate) input_filename: Option<PathBuf>,

    /// Filsystem path to txn directory (tree)
    #[arg(long="input.fs.dir",
        value_name = "txn dir path",
        requires("input_fs_ext"),
        conflicts_with_all([
            "input_git_repo",
            "input_git_ref",
            "input_git_dir"])
    )]
    pub(crate) input_fs_dir: Option<PathBuf>,

    /// Txn file extension
    #[arg(long = "input.fs.ext", value_name = "txn file extension")]
    pub(crate) input_fs_ext: Option<String>,

    /// Path to git repository
    ///
    /// This is path to '.git' directory.
    /// Either it is path to '.git' on bare repository, or path to '.git' on working copy
    #[arg(
        long = "input.git.repo",
        value_name = "git repo path",
        requires("input_git_ref"),
        requires("input_git_dir")
    )]
    pub(crate) input_git_repo: Option<PathBuf>,

    /// Git reference name
    #[arg(
        long = "input.git.ref",
        value_name = "git ref",
        requires("input_git_repo"),
        requires("input_git_dir")
    )]
    pub(crate) input_git_ref: Option<String>,

    /// Path prefix inside git repository
    #[arg(
        long = "input.git.dir",
        value_name = "git-path prefix",
        requires("input_git_repo"),
        requires("input_git_ref")
    )]
    pub(crate) input_git_dir: Option<String>,

    /// Report's Timezone [UTC, Europe/Helsinki, America/New_York, ...]
    #[arg(
        long = "report-tz",
        value_name = "timezone name",
        num_args(1),
        default_value = "UTC"
    )]
    pub(crate) report_tz: Option<String>,

    /// Account selectors for reports
    ///
    /// List of regex patterns for account names. For full match, use anchors ('^...$').
    #[arg(long = "accounts", value_name = "regex", num_args(1..))]
    pub(crate) accounts: Option<Vec<String>>,

    /// List of Reports to generate
    ///
    /// The list is space separated
    #[arg(long = "reports", value_name = "type", num_args(1..),
        value_parser([
            //PossibleValue::new("identity"),
            PossibleValue::new("equity"),
            PossibleValue::new("register"),
            PossibleValue::new("balance"),
            PossibleValue::new("balance-group"),
        ])
    )]
    pub(crate) reports: Option<Vec<String>>,

    /// Group-by -selector for 'balance-group' report
    #[arg(long = "group-by", value_name = "group-by", num_args(1), default_value = "year",
        value_parser([
            PossibleValue::new(txn_ts::GroupBy::YEAR),
            PossibleValue::new(txn_ts::GroupBy::MONTH),
            PossibleValue::new(txn_ts::GroupBy::DATE),
            PossibleValue::new(txn_ts::GroupBy::ISO_WEEK),
            PossibleValue::new(txn_ts::GroupBy::ISO_WEEK_DATE),
        ])
    )]
    pub(crate) group_by: Option<String>,
    /// Txn Filter definition (JSON), it could be ascii armored as base64 encoded
    ///
    /// The base64 ascii armor must have prefix "base64:". For example
    /// "base64:eyJ0eG5GaWx0ZXIiOnsiTnVsbGFyeVRSVUUiOnt9fX0K"
    #[arg(long = "api-filter-def", value_name = "filter def in json")]
    pub(crate) api_filter_def: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert()
    }
}
