/*
 * Tackler-NG 2023-2025
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::parser::Stream;
use crate::parser::parts::txns::parse_txns;

use crate::kernel::Settings;
use crate::model::Txns;
use crate::tackler;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use winnow::Parser;

pub(crate) fn txns_text(input: &mut &str, settings: &mut Settings) -> Result<Txns, tackler::Error> {
    let is = Stream {
        input,
        state: settings,
    };
    parse_txns.parse(is).map_err(|err| err.to_string().into())
}

pub(crate) fn txns_file(path: &Path, settings: &mut Settings) -> Result<Txns, tackler::Error> {
    let f = File::open(path);

    let mut txn_file = match f {
        Ok(file) => file,
        Err(err) => {
            let msg = format!("Can't open file: '{}' - {}", path.display(), err);
            return Err(msg.into());
        }
    };

    let mut txns_str = String::new();

    txn_file.read_to_string(&mut txns_str)?;

    // todo: error log
    txns_text(&mut txns_str.as_str(), settings)
}
