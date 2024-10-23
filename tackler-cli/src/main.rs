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

use std::error::Error;
use std::io;
use std::path::PathBuf;

use log::error;

use tackler_core::export::{EquityExporter, EquitySettings, Export, IdentityExporter};
use tackler_core::kernel::Settings;
use tackler_core::parser;
use tackler_core::parser::GitInputSelector;
use tackler_core::report::{
    BalanceGroupReporter, BalanceGroupSettings, BalanceReporter, BalanceSettings, RegisterReporter,
    RegisterSettings, Report,
};

use clap::Parser;
use tackler_api::filters::FilterDefinition;
use tackler_api::txn_ts;
use tackler_core::kernel::hash::Hash;
use tackler_core::kernel::settings::Audit;
use time_tz::timezones;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn run() -> Result<i32, Box<dyn Error>> {
    let cli = cli_args::Cli::parse();

    let hash = if let Some(audit) = cli.audit_mode {
        if audit {
            Some(Hash::default())
        } else {
            None
        }
    } else {
        None
    };

    let cfg = Settings {
        basedir: PathBuf::new().into_boxed_path(),
        accounts: cli.accounts,
        audit: Audit { hash },
    };

    let result = if cli.input_filename.is_some()
        || (cli.input_fs_dir.is_some() && cli.input_fs_ext.is_some())
    {
        if let Some(filename) = cli.input_filename {
            parser::paths_to_txns(&[filename], &cfg)
        } else {
            let paths = tackler_rs::get_paths_by_ext(
                cli.input_fs_dir.unwrap(/*:ok: clap */).as_path(),
                &cli.input_fs_ext.unwrap(/*:ok: clap */),
            )?;
            parser::paths_to_txns(&paths, &cfg)
        }
    } else if cli.input_git_repo.is_some()
        && cli.input_git_dir.is_some()
        && cli.input_git_ref.is_some()
    {
        parser::git_to_txns(
            cli.input_git_repo.unwrap(/*:ok: clap */).as_path(),
            cli.input_git_dir.as_deref().unwrap(/*:ok: clap */),
            "txn",
            GitInputSelector::Reference(cli.input_git_ref.unwrap(/*:ok: clap */)),
            &cfg,
        )
    } else {
        return Err("No input".into());
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

    match result {
        Ok(txn_data) => {
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
                                report_settings: BalanceSettings {
                                    title: Some("BALANCE".to_string()),
                                    ras: &cfg.accounts,
                                },
                            };
                            bal_reporter.write_txt_report(&cfg, &mut w, &txn_set)?;
                        }
                        "balance-group" => {
                            let report_tz = cli.report_tz.clone().unwrap(/*:ok: clap*/);
                            let group_by = cli.group_by.clone().unwrap(/*:ok: clap*/);
                            let bal_group_reporter = BalanceGroupReporter {
                                report_settings: BalanceGroupSettings {
                                    title: Some("BALANCE GROUP".to_string()), // todo: settings
                                    ras: &cfg.accounts,
                                    group_by: txn_ts::GroupBy::from(&group_by)?,
                                    report_tz: timezones::get_by_name(&report_tz)
                                        .ok_or(format!("Can't recognise tz [{report_tz}]"))?,
                                },
                            };
                            bal_group_reporter.write_txt_report(&cfg, &mut w, &txn_set)?;
                        }
                        "register" => {
                            let reg_reporter = RegisterReporter {
                                report_settings: RegisterSettings {
                                    title: Some("REGISTER".to_string()),
                                    ras: &cfg.accounts,
                                },
                            };
                            reg_reporter.write_txt_report(&cfg, &mut w, &txn_set)?;
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
                                    return Err("Internal Logic error with exports cli args (equity account name)".into());
                                }
                            };
                            let eq_exporter = EquityExporter {
                                export_settings: EquitySettings {
                                    eqa: Some(eqa.clone()),
                                    ras: &cfg.accounts,
                                },
                            };
                            eq_exporter.write_export(&cfg, &mut w, &txn_set)?;
                        }
                        "identity" => {
                            let eq_exporter = IdentityExporter {};
                            eq_exporter.write_export(&cfg, &mut w, &txn_set)?;
                        }
                        _ => {
                            return Err("Internal Logic error with exports cli args".into());
                        }
                    }
                }
            }
            Ok(0)
        }
        Err(err) => {
            let msg = format!("Error with transaction input: {err}");
            error!("{}", msg);
            Err(msg.into())
        }
    }
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
