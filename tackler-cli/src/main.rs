/*
 * Tackler-NG 2022-2024
 *
 * SPDX-License-Identifier: Apache-2.0
 */
#![forbid(unsafe_code)]

mod cli_args;

use log::error;
use std::error::Error;
use std::io;
use tackler_core::kernel::settings::Settings;
use tackler_core::parser;
use tackler_core::report::write_txt_reports;
use tackler_core::{export::write_exports, model::price_entry::PriceLookup};

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

    let price_db = cli
        .pricedb_filename
        .as_deref()
        .map(|path| parser::pricedb_from_file(path, &mut settings))
        .transpose()?
        .unwrap_or_else(Default::default);

    let mut console_output = if cli.output_directory.is_none() {
        Some(Box::new(io::stdout()))
    } else {
        None
    };

    let reports = settings.get_report_targets(cli.reports)?;

    let report_commodity = cli
        .report_commodity
        .as_deref()
        .map(|c| settings.get_commodity(c))
        .transpose()?;
    let report_price_lookup = cli
        .report_price_lookup
        .map(|c| match c {
            cli_args::PriceLookup::AtTheTimeOfTxn => Ok(PriceLookup::AtTheTimeOfTxn),
            cli_args::PriceLookup::AtTheTimeOfLastTxn => Ok(PriceLookup::AtTheTimeOfLastTxn),
            cli_args::PriceLookup::AtTheTimeOfTxnTsEndFilter => {
                Ok(PriceLookup::AtTheTimeOfTxnTsEndFilter)
            }
            cli_args::PriceLookup::LastPriceDbEntry => Ok(PriceLookup::LastPriceDbEntry),
            cli_args::PriceLookup::GivenTime(t) => {
                settings.parse_timestamp(&t).map(PriceLookup::GivenTime)
            }
        })
        .transpose()?;

    let group_by = cli.group_by.as_deref().map(GroupBy::from).transpose()?;

    if !reports.is_empty() {
        write_txt_reports(
            &mut console_output,
            cli.output_directory.as_ref(),
            &cli.output_name,
            &reports,
            report_commodity,
            report_price_lookup,
            &txn_set,
            &price_db,
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
