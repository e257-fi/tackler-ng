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
use crate::config::ReportType;
use crate::kernel::report_item_selector::ReportItemSelector;
use crate::kernel::Settings;
use crate::model::TxnSet;
pub use balance_group_reporter::BalanceGroupReporter;
pub use balance_group_reporter::BalanceGroupSettings;
pub use balance_reporter::BalanceReporter;
pub use balance_reporter::BalanceSettings;
pub use register_reporter::RegisterReporter;
pub use register_reporter::RegisterSettings;
use std::error::Error;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use tackler_api::metadata::items::{AccountSelectorChecksum, ReportTimezone, Text};
use tackler_api::txn_ts::GroupBy;
use tackler_rs::create_output_file;
use time_tz::{TimeZone, Tz};

mod balance_group_reporter;
mod balance_reporter;
mod register_reporter;

pub trait Report {
    fn write_txt_report<W: io::Write + ?Sized>(
        &self,
        cfg: &Settings,
        w: &mut W,
        txns: &TxnSet,
    ) -> Result<(), Box<dyn Error>>;
}

fn get_report_tz(_cfg: &Settings, tz: &'static Tz) -> Result<ReportTimezone, Box<dyn Error>> {
    let rtz = ReportTimezone {
        timezone: match tz.name() {
            // todo: remove this if there is a way to get short name of tz with time-tz
            "Etc/UTC" => "UTC".to_string(),
            name => name.to_string(),
        },
    };
    Ok(rtz)
}

fn write_acc_sel_checksum<W: io::Write + ?Sized, R: ReportItemSelector + ?Sized>(
    cfg: &Settings,
    writer: &mut W,
    acc_sel: &R,
) -> Result<(), Box<dyn Error>> {
    if let Some(hash) = cfg.get_hash() {
        let asc = AccountSelectorChecksum {
            hash: acc_sel.checksum(hash)?,
        };
        for v in asc.text() {
            writeln!(writer, "{}", &v)?;
        }
        writeln!(writer)?;
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub fn write_txt_reports<W: io::Write + ?Sized>(
    console_writer: &mut Option<Box<W>>,
    output_dir: Option<&PathBuf>,
    output_prefix: &Option<String>,
    reports: &Vec<ReportType>,
    txn_set: &TxnSet,
    group_by: Option<GroupBy>,
    settings: &Settings,
    prog_writer: &mut Option<Box<W>>,
) -> Result<(), Box<dyn Error>> {
    if !(output_dir.is_some() && output_prefix.is_some() && console_writer.is_none()
        || output_dir.is_none() && output_prefix.is_none() && console_writer.is_some())
    {
        return Err("IE: Logic error, console output is not supported with file ouput".into());
    }

    let report_separator_len = 82;

    let metadata = &txn_set
        .metadata()
        .map(|md| format!("{}\n", md.text()))
        .unwrap_or_default();

    if let Some(cw) = console_writer.as_mut() {
        write!(cw, "{}", metadata)?;
    }

    for r in reports {
        match r {
            ReportType::Balance => {
                let bal_reporter = BalanceReporter {
                    report_settings: BalanceSettings::from(settings)?,
                };

                match (output_prefix, output_dir) {
                    (Some(output_name), Some(output_dir)) => {
                        let (mut out_writer, path) =
                            create_output_file(output_dir, output_name, "bal", "txt")?;

                        write!(out_writer, "{}", metadata)?;

                        bal_reporter.write_txt_report(settings, &mut out_writer, txn_set)?;

                        if let Some(p) = prog_writer.as_mut() {
                            writeln!(p, "{:>21} : {}", "Balance Report", path)?;
                        }
                    }
                    _ => {
                        let mut cw = console_writer
                            .as_mut()
                            .expect("IE: logic error with output");

                        writeln!(cw, "{}", "*".repeat(report_separator_len))?;
                        bal_reporter.write_txt_report(settings, &mut cw, txn_set)?;
                        writeln!(cw, "{}", "#".repeat(report_separator_len))?;
                    }
                }
            }
            ReportType::BalanceGroup => {
                let group_by = match group_by {
                    Some(gb) => gb,
                    None => settings.report.balance_group.group_by,
                };
                let bal_group_reporter = BalanceGroupReporter {
                    report_settings: BalanceGroupSettings::from(settings, Some(group_by))?,
                };
                match (output_prefix, output_dir) {
                    (Some(output_name), Some(output_dir)) => {
                        let (mut out_writer, path) =
                            create_output_file(output_dir, output_name, "balgrp", "txt")?;

                        write!(out_writer, "{}", metadata)?;

                        bal_group_reporter.write_txt_report(settings, &mut out_writer, txn_set)?;

                        if let Some(p) = prog_writer.as_mut() {
                            writeln!(p, "{:>21} : {}", "Balance Group Report", path)?;
                        }
                    }
                    _ => {
                        let mut cw = console_writer
                            .as_mut()
                            .expect("IE: logic error with output");

                        writeln!(cw, "{}", "*".repeat(report_separator_len))?;
                        bal_group_reporter.write_txt_report(settings, &mut cw, txn_set)?;
                        writeln!(cw, "{}", "#".repeat(report_separator_len))?;
                    }
                }
            }
            ReportType::Register => {
                let reg_reporter = RegisterReporter {
                    report_settings: RegisterSettings::from(settings)?,
                };

                match (output_prefix, output_dir) {
                    (Some(output_name), Some(output_dir)) => {
                        let (mut out_writer, path) =
                            create_output_file(output_dir, output_name, "reg", "txt")?;
                        write!(out_writer, "{}", metadata)?;
                        reg_reporter.write_txt_report(settings, &mut out_writer, txn_set)?;
                        if let Some(p) = prog_writer.as_mut() {
                            writeln!(p, "{:>21} : {}", "Register Report", path)?;
                        }
                    }
                    _ => {
                        let mut cw = console_writer
                            .as_mut()
                            .expect("IE: logic error with output");

                        writeln!(cw, "{}", "*".repeat(report_separator_len))?;
                        reg_reporter.write_txt_report(settings, &mut cw, txn_set)?;
                        writeln!(cw, "{}", "#".repeat(report_separator_len))?;
                    }
                }
            }
        }
    }
    Ok(())
}
