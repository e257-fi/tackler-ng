/*
 * Tackler-NG 2023-2025
 * SPDX-License-Identifier: Apache-2.0
 */
use crate::config::ReportType;
use crate::kernel::report_item_selector::ReportItemSelector;
use crate::kernel::{BalanceGroupSettings, RegisterSettings, Settings};
use crate::model::TxnSet;
pub use balance_group_reporter::BalanceGroupReporter;
pub use balance_reporter::BalanceReporter;
pub use register_reporter::RegisterReporter;
use std::error::Error;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use tackler_api::metadata::items::{AccountSelectorChecksum, ReportTimezone, Text};
use tackler_api::txn_ts::GroupBy;
use tackler_rs::create_output_file;

mod balance_group_reporter;
mod balance_reporter;
mod register_reporter;

pub trait Report {
    fn write_txt_report<W: io::Write + ?Sized>(
        &self,
        cfg: &Settings,
        w: &mut W,
        txns: &TxnSet<'_>,
    ) -> Result<(), Box<dyn Error>>;
}

fn write_report_timezone<W: io::Write + ?Sized>(
    cfg: &Settings,
    writer: &mut W,
) -> Result<(), Box<dyn Error>> {
    let rtz = ReportTimezone {
        timezone: match cfg.report.report_tz.iana_name() {
            Some(tz) => tz.to_string(),
            None => {
                let msg = "no name for tz!?!";
                return Err(msg.into());
            }
        },
    };
    for v in rtz.text(cfg.report.report_tz.clone()) {
        writeln!(writer, "{}", &v)?;
    }
    Ok(())
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
        for v in asc.text(cfg.report.report_tz.clone()) {
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
    txn_set: &TxnSet<'_>,
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
        .map(|md| format!("{}\n", md.text(settings.report.report_tz.clone())))
        .unwrap_or_default();

    if let Some(cw) = console_writer.as_mut() {
        write!(cw, "{}", metadata)?;
    }

    for r in reports {
        match r {
            ReportType::Balance => {
                let bal_reporter = BalanceReporter::try_from(settings)?;

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
                let group_by = group_by.unwrap_or(settings.report.balance_group.group_by);
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
                    report_settings: RegisterSettings::try_from(settings)?,
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
