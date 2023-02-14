/*
 * Copyright 2022 E257.FI
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
use std::env;
use std::error::Error;
use std::path::Path;

use log::error;

use tackler_core::kernel::Settings;
use tackler_core::parser;
use tackler_core::parser::GitInputSelector;

const CFG_FILE: &str = "tackler.conf";

fn run() -> Result<i32, Box<dyn Error>> {
    let _cfg: Settings = Settings::from(CFG_FILE)?;

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Error: Missing input file");
        eprintln!("Usage: {} <tackler txns-file in base dir>", &args[0]);
        std::process::exit(1);
    }

    let result = if false {
        let paths = tackler_rs::get_paths_by_ext(Path::new(&args[1]), "txn").unwrap();
        parser::paths_to_txns(paths)
    } else {
        parser::git_to_txns(Path::new(&args[1]), "txns-1E1",
                            "txn",
                            GitInputSelector::Reference("main".to_string()))
                            //GitInputSelector::CommitId("359400fa06c3e516a7133eea0d74f9a84310032a".to_string()))
    };

    println!("tackler: {}", env!("VERSION"));

    match result {
        Ok(txn_data) => {
            println!("ok!");
            if let Some(metadata) = txn_data.metadata {
                println!("{:#?}", &metadata);
                println!("MetaData:");
                println!("{}", metadata.text());
            }
            println!("TxnsData:");
            for txn in txn_data.txns {
                println!("{txn}");
            }
            Ok(0)
        }
        Err(err) => {
            let msg = format!("Error with transaction input: {err}");
            error!("{}", msg);
            Err(msg.into())
        }
    }
}

fn main() {
    match run() {
        Ok(_) => std::process::exit(0),
        Err(err) => {
            let msg = format!("Tackler error: {err}");
            error!("{msg}");
            eprintln!("{msg}");
            std::process::exit(1)
        }
    }
}
