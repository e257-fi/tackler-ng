/*
 * Copyright 2023-2025 E257.FI
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

use crate::parser::parts::txns::parse_txns;
use crate::parser::Stream;
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
                Some(ce) => {
                    if let Some(cause) = ce.cause() {
                        let _ = writeln!(msg, "Cause:\n{}\n", cause);
                    }
                    let _ = writeln!(msg, "Error backtrace:");
                    for c in ce.context() {
                        let _ = writeln!(msg, "   {}", c);
                    }
                }
                None => {
                    let _ = write!(msg, "No detailed error information available");
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
