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
use tackler_core::export::{EquityExporter, EquitySettings, Export, IdentityExporter};
use tackler_core::kernel::Settings;
use tackler_core::parser;
use tackler_core::report::{
    BalanceGroupReporter, BalanceGroupSettings, BalanceReporter, BalanceSettings, RegisterReporter,
    RegisterSettings, Report,
};

use clap::Parser;
use tackler_api::filters::FilterDefinition;
use tackler_api::txn_ts;
use tackler_core::kernel::config::Config;

use tackler_core::kernel::settings::Input;
#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

fn run() -> Result<i32, Box<dyn Error>> {
    let cli = cli_args::Cli::parse();
    let cfg = match &cli.conf_path {
        Some(path) => Some(Config::from(path)?),
        None => None,
    };

    let mut settings = Settings::from(cfg, cli.audit_mode, cli.accounts.clone())?;

    let input_type = cli.get_input_type(&settings)?;

    #[rustfmt::skip]
    let result = match input_type {
        Input::File(f) => {
            parser::paths_to_txns(&[f.path], &mut settings)
        },
        Input::Fs(fs) => {
            let paths = tackler_rs::get_paths_by_ext(fs.dir.as_path(), fs.glob.as_str())?;
            parser::paths_to_txns(&paths, &mut settings)
        }
        Input::Git(git) => {
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
            let msg = format!("Error with transaction input: {err}");
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

    if let Some(reports) = cli.reports {
        let mut w: Box<dyn io::Write> = Box::new(io::stdout());

        if let Some(metadata) = &txn_set.metadata() {
            writeln!(&mut w, "{}", metadata.text())?;
        }

        for r in reports {
            match r.as_str() {
                // todo: fix this
                "balance" => {
                    let bal_reporter = BalanceReporter {
                        report_settings: BalanceSettings::from(&settings)?,
                    };
                    bal_reporter.write_txt_report(&mut settings, &mut w, &txn_set)?;
                }
                "balance-group" => {
                    let group_by = cli.group_by.clone().unwrap(/*:ok: clap*/);
                    let bal_group_reporter = BalanceGroupReporter {
                        report_settings: BalanceGroupSettings::from(
                            &settings,
                            Some(txn_ts::GroupBy::from(group_by.as_str())?),
                        )?,
                    };
                    bal_group_reporter.write_txt_report(&mut settings, &mut w, &txn_set)?;
                }
                "register" => {
                    let reg_reporter = RegisterReporter {
                        report_settings: RegisterSettings::from(&settings)?,
                    };
                    reg_reporter.write_txt_report(&mut settings, &mut w, &txn_set)?;
                }
                _ => {
                    return Err("Internal Logic error with reports cli args".into());
                }
            }
        }
    }

    if let Some(exports) = cli.exports {
        let mut w: Box<dyn io::Write> = Box::new(io::stdout());

        for e in exports {
            match e.as_str() {
                "equity" => {
                    let eqa = match &cli.equity_account_name {
                        Some(e) => e,
                        _ => {
                            return Err(
                                "Internal Logic error with exports cli args (equity account name)"
                                    .into(),
                            );
                        }
                    };
                    let eq_exporter = EquityExporter {
                        export_settings: EquitySettings {
                            eqa: Some(eqa.clone()),
                            ras: &settings.report.report_acc_sel.clone(),
                        },
                    };
                    eq_exporter.write_export(&mut settings, &mut w, &txn_set)?;
                }
                "identity" => {
                    let eq_exporter = IdentityExporter {};
                    eq_exporter.write_export(&mut settings, &mut w, &txn_set)?;
                }
                _ => {
                    return Err("Internal Logic error with exports cli args".into());
                }
            }
        }
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
