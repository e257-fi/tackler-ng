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

use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::token_factory::CommonTokenFactory;
use antlr_rust::{BailErrorStrategy, InputStream};

use crate::model::Txns;
use crate::parser::ctx_handler;
use crate::parser::txn_antlr::txnlexer::TxnLexer;
use crate::parser::txn_antlr::txnparser::TxnParser;

pub(crate) fn txns_text(input_text: &str) -> Result<Txns, Box<dyn Error>> {
    let tf = CommonTokenFactory::default();

    let mut _lexer = TxnLexer::new_with_token_factory(InputStream::new(input_text), &tf);

    let token_source = CommonTokenStream::new(_lexer);
    let mut parser = TxnParser::<'_, _, BailErrorStrategy<'_, _>>::new(token_source);

    let result = parser.txns();

    match result {
        Ok(txns_ctx) => ctx_handler::handle_txns(txns_ctx),
        Err(err) => {
            let msg = format!("ANTRL error: {err}");
            Err(msg.into())
        }
    }
}

pub(crate) fn txns_file(path: &Path) -> Result<Txns, Box<dyn Error>> {
    let f = File::open(path);

    let mut txn_file = match f {
        Ok(file) => file,
        Err(err) => {
            let msg = format!(
                "Can't open file: [{}]. Error is: \"{}\"",
                path.display(),
                err
            );
            return Err(msg.into());
        }
    };

    let mut txns_str = String::new();

    txn_file.read_to_string(&mut txns_str)?;

    // todo: error log
    txns_text(txns_str.as_str())
}
