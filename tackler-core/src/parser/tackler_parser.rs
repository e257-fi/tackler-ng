/*
 * Tackler-NG 2023-2025
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::parser::Stream;
use crate::parser::parts::txns::parse_txns;
use std::fmt::Write;

use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::kernel::Settings;
use crate::model::Txns;

pub(crate) fn txns_text(input: &mut &str, settings: &mut Settings) -> Result<Txns, Box<dyn Error>> {
    let mut is = Stream {
        input,
        state: settings,
    };

    match parse_txns(&mut is) {
        Ok(txns) => Ok(txns),
        Err(err) => {
            let mut msg = "Failed to process txn input\n".to_string();
            //let _ = writeln!(msg, "Error: {}", err);
            match err.into_inner() {
                Ok(ce) => {
                    if let Some(cause) = ce.cause() {
                        let _ = writeln!(msg, "Cause:\n{}\n", cause);
                    }
                    let _ = writeln!(msg, "Error backtrace:");
                    for c in ce.context() {
                        let _ = writeln!(msg, "   {}", c);
                    }
                }
                Err(_err) => {
                    let _ = write!(msg, "Incomplete input");
                }
            }
            let i = is.input.lines().next().unwrap_or(is.input);
            let i_err = if i.chars().count() < 1024 {
                i.to_string()
            } else {
                i.chars().take(1024).collect::<String>()
            };

            let _ = write!(msg, "Failed input:\n{}\n\n", i_err);

            Err(msg.into())
        }
    }
}

pub(crate) fn txns_file(path: &Path, settings: &mut Settings) -> Result<Txns, Box<dyn Error>> {
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
