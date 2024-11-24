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
use clap::Parser;
use std::error::Error;
use std::path::PathBuf;
use tackler_api::txn_ts;
use tackler_api::txn_ts::GroupBy;
use tackler_core::config;
use tackler_core::kernel::settings::{FileInput, FsInput, GitInput, InputSettings};
use tackler_core::kernel::Settings;
use tackler_core::parser::GitInputSelector;

#[derive(Parser)]
#[command(author, version=env!("VERSION"), about, long_about = None)]
pub(crate) struct Cli {
    #[arg(long = "config", value_name = "config file path")]
    pub(crate) conf_path: PathBuf,

    /// Strict txn data mode
    ///
    /// Turn on strict validation of transactions (accounts, commodities and tags).
    #[arg(long = "strict.mode", value_name = "true|false")]
    pub(crate) strict_mode: Option<bool>,

    /// Txn set audit mode
    ///
    /// Produce checksums for transaction data and account selectors
    #[arg(long = "audit.mode", value_name = "true|false")]
    pub(crate) audit_mode: Option<bool>,

    /// Path to output directory
    #[arg(
        long = "output.dir",
        value_name = "output directory for reports",
        requires("output_name")
    )]
    pub(crate) output_directory: Option<PathBuf>,

    /// Basename of report files
    #[arg(
        long = "output.prefix",
        value_name = "prefix of name for report files",
        requires("output_directory")
    )]
    pub(crate) output_name: Option<String>,

    /// Path to single transaction journal file
    #[arg(long="input.file",
        value_name = "txn file path",
        conflicts_with_all([
            "input_storage",
            "input_fs_dir",
            "input_fs_ext",
            "input_git_repo",
            "input_git_ref",
            "input_git_dir"])
    )]
    pub(crate) input_filename: Option<PathBuf>,

    ///
    /// Select used transaction storage
    ///
    #[arg(long="input.storage",
        value_name = "storage type",
        value_parser([
            PossibleValue::new(config::StorageType::STORAGE_FS),
            PossibleValue::new(config::StorageType::STORAGE_GIT),
        ])
    )]
    pub(crate) input_storage: Option<String>,

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
    #[arg(
        long = "input.fs.ext",
        value_name = "txn file suffix",
        requires("input_fs_dir")
    )]
    pub(crate) input_fs_ext: Option<String>,

    /// Path to git repository
    ///
    /// Path to '.git' directory or bare git-repository.
    ///
    /// This could be a path to '.git' directory inside working copy
    #[arg(
        long = "input.git.repository",
        value_name = "git repository path",
        requires("input_git_ref"),
        requires("input_git_dir")
    )]
    pub(crate) input_git_repo: Option<PathBuf>,

    /// Git reference name
    #[arg(long = "input.git.ref", value_name = "git ref")]
    pub(crate) input_git_ref: Option<String>,

    /// Path prefix inside git repository
    #[arg(
        long = "input.git.dir",
        value_name = "git-path prefix",
        requires("input_git_repo"),
        requires("input_git_ref")
    )]
    pub(crate) input_git_dir: Option<String>,

    /// Account selectors for reports and exports
    ///
    /// List of patterns (regex) for account names.
    ///
    /// Use anchors ('^...$') for exact match.
    ///
    /// Use empty string "" to list all accounts
    #[arg(long = "accounts", value_name = "regex", num_args(1..))]
    pub(crate) accounts: Option<Vec<String>>,

    /// List of Reports to generate
    ///
    /// The list is space separated
    #[arg(long = "reports", value_name = "type", num_args(1..),
        value_parser([
            PossibleValue::new("register"),
            PossibleValue::new("balance"),
            PossibleValue::new("balance-group"),
        ])
    )]
    pub(crate) reports: Option<Vec<String>>,

    /// Group-by -selector for 'balance-group' report
    #[arg(long = "group-by", value_name = "group-by", num_args(1),
        value_parser([
            PossibleValue::new(txn_ts::GroupBy::YEAR),
            PossibleValue::new(txn_ts::GroupBy::MONTH),
            PossibleValue::new(txn_ts::GroupBy::DATE),
            PossibleValue::new(txn_ts::GroupBy::ISO_WEEK),
            PossibleValue::new(txn_ts::GroupBy::ISO_WEEK_DATE),
        ])
    )]
    pub(crate) group_by: Option<GroupBy>,

    /// List of Exports to generate
    ///
    /// The list is space separated
    #[arg(long = "exports", value_name = "type", num_args(1..),
        value_parser([
            PossibleValue::new("identity"),
            PossibleValue::new("equity"),
        ])
    )]
    pub(crate) exports: Option<Vec<String>>,

    /// Txn Filter definition (JSON), it could be ascii armored as base64 encoded
    ///
    /// The base64 ascii armor must have prefix "base64:".
    ///
    /// For example "base64:eyJ0eG5GaWx0ZXIiOnsiTnVsbGFyeVRSVUUiOnt9fX0K"
    #[arg(long = "api-filter-def", value_name = "filter def in json")]
    pub(crate) api_filter_def: Option<String>,
}

impl Cli {
    pub fn get_input_type(&self, settings: &Settings) -> Result<InputSettings, Box<dyn Error>> {
        if let Some(filename) = &self.input_filename {
            let i = FileInput {
                path: filename.clone(),
            };
            Ok(InputSettings::File(i))
        } else if self.input_fs_dir.is_some() {
            let i = FsInput {
                dir: self.input_fs_dir.clone().unwrap(/*:ok: clap */),
                suffix: self.input_fs_ext.clone().unwrap(/*:ok: clap */),
            };
            Ok(InputSettings::Fs(i))
        } else if self.input_git_repo.is_some() {
            let i = GitInput {
                repo: self.input_git_repo.clone().unwrap(/*:ok: clap */),
                git_ref: GitInputSelector::Reference(
                    self.input_git_ref.clone().unwrap(/*:ok: clap */),
                ),
                dir: self.input_git_dir.clone().unwrap(/*:ok: clap */).clone(),
                ext: String::from("txn"),
            };
            Ok(InputSettings::Git(i))
        } else if self.input_git_ref.is_some() && self.input_git_repo.is_none() {
            let input_git_ref = self.input_git_ref.clone().unwrap(/*:ok: clap */);
            match settings.get_input_settings(
                Some(&config::StorageType::STORAGE_GIT.to_string()),
                Some(self.conf_path.as_path()),
            )? {
                InputSettings::Git(git) => Ok(InputSettings::Git(GitInput {
                    git_ref: GitInputSelector::Reference(input_git_ref),
                    ..git
                })),
                _ => {
                    let msg = "CLI Arg handling: Internal logic error";
                    Err(msg.into())
                }
            }
        } else {
            settings.get_input_settings(self.input_storage.as_ref(), Some(self.conf_path.as_path()))
        }
    }
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
