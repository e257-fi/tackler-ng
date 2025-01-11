/*
 * Tackler-NG 2024
 *
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::export::Export;
use crate::kernel::Settings;
use crate::model::TxnSet;
use std::error::Error;
use std::io;

#[derive(Debug, Clone)]
pub struct IdentityExporter {}

impl Export for IdentityExporter {
    fn write_export<W: io::Write + ?Sized>(
        &self,
        _cfg: &Settings,
        writer: &mut W,
        txn_data: &TxnSet<'_>,
    ) -> Result<(), Box<dyn Error>> {
        for txn in &txn_data.txns {
            writeln!(writer, "{}", txn)?;
        }
        Ok(())
    }
}
