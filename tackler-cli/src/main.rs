/*
 * Copyright 2022-2024 E257.FI
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
#![forbid(unsafe_code)]

mod cli_args;

use log::error;
use std::error::Error;
use std::io;
use tackler_core::export::write_exports;
use tackler_core::kernel::settings::Settings;
use tackler_core::parser;
use tackler_core::report::write_txt_reports;

use clap::Parser;
use tackler_api::filters::FilterDefinition;
use tackler_core::config::Config;

use tackler_api::txn_ts::GroupBy;
use tackler_core::kernel::settings::InputSettings;
#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

fn run() -> Result<i32, Box<dyn Error>> {
    let cli = cli_args::Cli::parse();
    let cfg = match Config::from(&cli.conf_path) {
        Ok(cfg) => cfg,
        Err(err) => {
            let msg = format!(
                "Configuration error with '{}': {err}",
                cli.conf_path.display()
            );
            error!("{}", msg);
            return Err(msg.into());
        }
    };

    let mut settings = Settings::from(cfg, cli.strict_mode, cli.audit_mode, cli.accounts.clone())?;

    let input_type = cli.get_input_type(&settings)?;

    #[rustfmt::skip]
    let result = match input_type {
        InputSettings::File(f) => {
            parser::paths_to_txns(&[f.path], &mut settings)
        },
        InputSettings::Fs(fs) => {
            let paths = tackler_rs::get_paths_by_ext(fs.dir.as_path(), fs.suffix.as_str())?;
            parser::paths_to_txns(&paths, &mut settings)
        }
        InputSettings::Git(git) => {
            parser::git_to_txns(
                git.repo.as_path(),
                git.dir.as_str(),
                git.ext.as_str(),
                git.git_ref,
                &mut settings,
            )
        },
    };

    let txn_data = match result {
        Ok(txn_data) => txn_data,
        Err(err) => {
            let msg = format!("Txn Data: {err}");
            error!("{}", msg);
            return Err(msg.into());
        }
    };

    let txn_filt = match cli.api_filter_def {
        Some(filt_str) => {
            if FilterDefinition::is_armored(&filt_str) {
                Some(FilterDefinition::from_armor(&filt_str)?)
            } else {
                Some(FilterDefinition::from_json_str(&filt_str)?)
            }
        }
        None => None,
    };

    let txn_set = match txn_filt {
        Some(tf) => txn_data.filter(&tf)?,
        None => txn_data.get_all()?,
    };

    let mut console_output = if cli.output_directory.is_none() {
        Some(Box::new(io::stdout()))
    } else {
        None
    };

    let reports = settings.get_report_targets(cli.reports)?;
    let group_by: Option<GroupBy> = match cli.group_by {
        Some(gp) => Some(GroupBy::from(gp.as_str())?),
        None => None,
    };

    if !reports.is_empty() {
        write_txt_reports(
            &mut console_output,
            cli.output_directory.as_ref(),
            &cli.output_name,
            &reports,
            &txn_set,
            group_by,
            &settings,
            &mut Some(Box::new(io::stdout())),
        )?;
    }

    let exports = settings.get_export_targets(cli.exports)?;
    if !exports.is_empty() && cli.output_directory.is_some() {
        write_exports(
            cli.output_directory
                .as_ref()
                .expect("IE: logic error with CLI arguments"),
            cli.output_name
                .expect("IE: logic error with CLI arguments")
                .as_str(),
            &exports,
            &txn_set,
            &mut settings,
            &mut Some(Box::new(io::stdout())),
        )?;
    }
    Ok(0)
}

fn main() {
    match run() {
        Ok(_) => std::process::exit(0),
        Err(err) => {
            let msg = format!("Tackler error: {err}");
            error!("{msg}");
            eprintln!("{msg}");
            std::process::exit(1)
        }
    }
}
