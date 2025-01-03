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
    let txns = parse_txns(&mut is)?;

    Ok(txns)
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
