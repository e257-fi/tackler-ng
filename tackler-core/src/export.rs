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
use std::error::Error;
use std::io;

pub use equity_exporter::EquityExporter;
pub use equity_exporter::EquitySettings;

pub use identity_exporter::IdentityExporter;

mod equity_exporter;
mod identity_exporter;

pub trait Export {
    fn write_export<W: io::Write + ?Sized>(
        &self,
        cfg: &mut Settings,
        w: &mut W,
        txns: &TxnSet,
    ) -> Result<(), Box<dyn Error>>;
}
