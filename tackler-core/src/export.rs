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
use crate::kernel::Settings;
use crate::model::TxnSet;
pub use equity_exporter::EquityExporter;
pub use equity_exporter::EquitySettings;
use std::error::Error;
use std::io;
use std::path::Path;

use crate::config::ExportType;
pub use identity_exporter::IdentityExporter;
use tackler_rs::create_output_file;

mod equity_exporter;
mod identity_exporter;

pub trait Export {
    fn write_export<W: io::Write + ?Sized>(
        &self,
        cfg: &Settings,
        w: &mut W,
        txns: &TxnSet<'_>,
    ) -> Result<(), Box<dyn Error>>;
}

pub fn write_exports<ProgW: io::Write + ?Sized>(
    output_dir: &Path,
    output_name: &str,
    exports: &Vec<ExportType>,
    txn_set: &TxnSet<'_>,
    settings: &mut Settings,
    prog_writer: &mut Option<Box<ProgW>>,
) -> Result<(), Box<dyn Error>> {
    for e in exports {
        match e {
            ExportType::Equity => {
                let eq_exporter = EquityExporter {
                    export_settings: EquitySettings::from(settings)?,
                };

                let (mut out_writer, path) =
                    create_output_file(output_dir, output_name, "equity", "txn")?;
                eq_exporter.write_export(settings, &mut out_writer, txn_set)?;
                if let Some(p) = prog_writer.as_mut() {
                    writeln!(p, "{:>21} : {}", "Equity Export", path)?;
                }
            }
            ExportType::Identity => {
                let id_exporter = IdentityExporter {};

                let (mut out_writer, path) =
                    create_output_file(output_dir, output_name, "identity", "txn")?;
                id_exporter.write_export(settings, &mut out_writer, txn_set)?;
                if let Some(p) = prog_writer.as_mut() {
                    writeln!(p, "{:>21} : {}", "Identity Export", path)?;
                }
            }
        }
    }

    Ok(())
}
