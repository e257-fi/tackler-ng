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
use std::error::Error;

use crate::kernel::Settings;
use crate::model::TxnSet;
pub use balance_group_reporter::BalanceGroupReporter;
pub use balance_group_reporter::BalanceGroupSettings;
pub use balance_reporter::BalanceReporter;
pub use balance_reporter::BalanceSettings;
pub use register_reporter::RegisterReporter;
pub use register_reporter::RegisterSettings;
use std::io;
use tackler_api::metadata::items::{AccountSelectorChecksum, ReportTimezone};
use time_tz::{TimeZone, Tz};

mod balance_group_reporter;
mod balance_reporter;
mod register_reporter;

pub trait Report {
    fn write_txt_report<W: io::Write + ?Sized>(
        &self,
        cfg: &mut Settings,
        w: &mut W,
        txns: &TxnSet,
    ) -> Result<(), Box<dyn Error>>;
}

pub fn get_report_tz(_cfg: &Settings, tz: &'static Tz) -> Result<ReportTimezone, Box<dyn Error>> {
    let rtz = ReportTimezone {
        timezone: tz.name().to_string(),
    };
    Ok(rtz)
}

pub fn get_account_selector_checksum(
    cfg: &Settings,
    ras: &[String],
) -> Result<Option<AccountSelectorChecksum>, Box<dyn Error>> {
    if let Some(hash) = cfg.get_hash() {
        if ras.is_empty() {
            Ok(None)
        } else {
            // todo: refactor and test this
            let mut accsel = ras.to_owned();
            accsel.sort();
            let h = hash.checksum(&accsel, "\n".as_bytes())?;
            let asc = AccountSelectorChecksum { hash: h };
            Ok(Some(asc))
        }
    } else {
        Ok(None)
    }
}
