/*
 * Copyright 2023 E257.FI
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

use crate::model::TxnData;
use std::error::Error;

pub use balance_reporter::BalanceReporter;
pub use balance_reporter::BalanceSettings;
pub use register_reporter::RegisterReporter;
pub use register_reporter::RegisterSettings;
use std::io::Write;

mod balance_reporter;
mod register_reporter;

pub trait Report {
    fn write_txt_report<W: Write + ?Sized>(
        &self,
        w: &mut W,
        txns: &TxnData,
    ) -> Result<(), Box<dyn Error>>;
}
