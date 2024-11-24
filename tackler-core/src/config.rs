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
pub(crate) use items::AccountSelectors;
pub use items::Config;
pub(crate) use items::Export;
pub use items::ExportType;
pub use items::Input;
pub(crate) use items::Kernel;
pub(crate) use items::Report;
pub use items::ReportType;
pub(crate) use items::Scale;
pub use items::StorageType;
pub(crate) use items::TimezoneType;
use std::error::Error;

mod items;
mod raw_items;

pub fn to_report_targets(targets: &[String]) -> Result<Vec<ReportType>, Box<dyn Error>> {
    let trgs =
        targets.iter().try_fold(
            Vec::new(),
            |mut trgs: Vec<ReportType>, trg| match ReportType::from(trg.as_str()) {
                Ok(t) => {
                    trgs.push(t);
                    Ok::<Vec<ReportType>, Box<dyn Error>>(trgs)
                }
                Err(e) => {
                    let msg = format!("Invalid report target: {e}");
                    Err(msg.into())
                }
            },
        )?;
    Ok(trgs)
}

pub fn to_export_targets(targets: &[String]) -> Result<Vec<ExportType>, Box<dyn Error>> {
    let trgs =
        targets.iter().try_fold(
            Vec::new(),
            |mut trgs: Vec<ExportType>, trg| match ExportType::from(trg.as_str()) {
                Ok(t) => {
                    trgs.push(t);
                    Ok::<Vec<ExportType>, Box<dyn Error>>(trgs)
                }
                Err(e) => {
                    let msg = format!("Invalid export target: {e}");
                    Err(msg.into())
                }
            },
        )?;
    Ok(trgs)
}
