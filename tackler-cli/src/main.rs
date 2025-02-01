/*
 * Tackler-NG 2022-2025
 * SPDX-License-Identifier: Apache-2.0
 */
#![forbid(unsafe_code)]

mod cli_args;
mod commands;

use crate::cli_args::PRICE_BEFORE;
use log::error;
use std::error::Error;
use std::io;
use tackler_core::export::write_exports;
use tackler_core::kernel::settings::Settings;
use tackler_core::report::write_txt_reports;
use tackler_core::{config, parser};

use clap::Parser;
use tackler_api::filters::FilterDefinition;
use tackler_core::config::Config;

use crate::cli_args::{Commands, DefaultModeArgs};
use tackler_api::txn_ts::GroupBy;
use tackler_core::kernel::price_lookup::PriceLookup;
use tackler_core::kernel::settings::InputSettings;
#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

fn run(cli: DefaultModeArgs) -> Result<(), Box<dyn Error>> {
    let cfg = match Config::from(cli.conf_path.as_ref().unwrap()) {
        Ok(cfg) => cfg,
        Err(err) => {
            let msg = format!(
                "Configuration error with '{}': {err}",
                cli.conf_path.as_ref().unwrap().display()
            );
            error!("{}", msg);
            return Err(msg.into());
        }
    };

    let price_overlap = cli.get_price_overlap();
    let report_overlap = cli.get_report_overlap();

    let mut settings = Settings::try_from(
        cfg,
        cli.strict_mode,
        cli.audit_mode,
        report_overlap,
        price_overlap,
    )?;

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

    let report_commodity = settings.get_report_commodity();

    let report_price_lookup: Option<PriceLookup> = match settings.price.lookup_type {
        // todo: move this logic to settings and check given_time usage with other targets
        config::PriceLookupType::LastPrice => Some(PriceLookup::LastPriceDbEntry),
        config::PriceLookupType::TxnTime => Some(PriceLookup::AtTheTimeOfTxn),
        config::PriceLookupType::GivenTime => Some(cli.price_before_ts.map_or_else(
            || {
                Err(format!(
                    "Price lookup type is \"{}\" and there is no timestamp by --{PRICE_BEFORE}",
                    config::PriceLookupType::GIVEN_TIME
                )
                .into())
            },
            |ts| settings.parse_timestamp(&ts).map(PriceLookup::GivenTime),
        )?),
        config::PriceLookupType::None => None,
    };

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
            &settings.price.price_db,
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
    Ok(())
}

fn main() {
    let cli = cli_args::Cli::parse();

    let command = cli.cmd();

    let res = match command {
        Commands::New { name } => commands::new::exec(name.as_str()),
        Commands::Init {} => commands::init::exec("."),
        Commands::Report(args) => run(args),
    };

    match res {
        Ok(_) => std::process::exit(0),
        Err(err) => {
            let msg = format!("Tackler error: {err}");
            error!("{msg}");
            eprintln!("{msg}");
            std::process::exit(1)
        }
    }
}
