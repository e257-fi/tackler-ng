/*
 * Copyright 2019-2023 E257.FI
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
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use tackler_api::metadata::items::MetadataItem;
use tackler_core::kernel::Settings;
use tackler_core::model::TxnData;
use tackler_core::parser;
use tackler_core::parser::GitInputSelector;

#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

const REPO_PATH: &str = "suite/audit/audit-repo.git/";
const TXN_SET_1E1_CHECKSUM: &str =
    "9b29071e1bf228cfbd31ca2b8e7263212e4b86e51cfee1e8002c9b795ab03f76";
const TXN_SET_1E1_COMMIT_ID: &str = "4aa4e9797501c1aefc92f32dff30ab462dae5545";

const TXN_SET_1E5_CHECKSUM: &str =
    "27060dc1ebde35bebd8f7af2fd9815bc9949558d3e3c85919813cd80748c99a7";
const TXN_SET_1E5_COMMIT_ID: &str = "cb56fdcdd2b56d41fc08cc5af4a3b410896f03b5";

fn verify_git_run(result: Result<TxnData, Box<dyn Error>>, commit: &str, checksum: &str) {
    match result {
        Ok(txn_data) => {
            let txn_set = txn_data.get_all().unwrap(/*:test:*/);
            match txn_set.metadata() {
                Some(md) => {
                    assert_eq!(md.items.len(), 2, "Metadata Item count is wrong");
                    match &md.items[0] {
                        MetadataItem::GitInputReference(gitmd) => {
                            assert_eq!(gitmd.commit, commit);
                        }
                        _ => {
                            panic!(/*:test:*/ "The first item is not Git Input Metadata item")
                        }
                    }
                    match &md.items[1] {
                        MetadataItem::TxnSetChecksum(tscsmd) => {
                            assert_eq!(tscsmd.hash.value, checksum);
                        }
                        _ => {
                            panic!(
                                /*:test:*/
                                "The second item is not Txn Set Checksum Metadata item"
                            )
                        }
                    }
                }
                None => {
                    panic!(/*:test:*/ "no metadata")
                }
            }
        }
        Err(err) => {
            eprintln!("{err:#}");
            panic!(/*:test:*/);
        }
    }
}

// desc: "make 10 loops with txns-1E5"
#[allow(non_snake_case)]
fn id_33d85471_a04c_49b9_b7a0_9d7f7f5762eb__loop_with_txns_1E5_10x() {
    eprintln!("\n\nMake 10 loops with txns-1E5:");
    let mut all_txns_per_s = 0.0;
    for i in 0..10 {
        let ts_start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap(/*:test:*/);
        let result = parser::git_to_txns(
            Path::new(REPO_PATH),
            "txns/2016",
            "txn",
            GitInputSelector::Reference("txns-1E5".to_string()),
            &Settings::default_audit(),
        );
        let ts_end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap(/*:test:*/);
        verify_git_run(result, TXN_SET_1E5_COMMIT_ID, TXN_SET_1E5_CHECKSUM);

        let txn_per_s = 100_000.0 / ((ts_end.as_millis() - ts_start.as_millis()) as f64 / 1000.0);

        eprintln!("Done: {i:10} of 10 loops {txn_per_s:.0} txn/s");
        all_txns_per_s += txn_per_s;
    }
    let txn_per_s_ave = all_txns_per_s / 10.0;
    // yes, these are correct
    let scala_txn_per_s_ref = 40000.0;
    let rust_txn_per_s_ref = 25000.0;

    eprintln!("\nOn average {txn_per_s_ave:.0} txn/s");
    eprintln!(
        "Reference system (laptop): {rust_txn_per_s_ref:.0} txn/s ({:>6.0} txn/s)",
        txn_per_s_ave - rust_txn_per_s_ref
    );
    eprintln!(
        "Reference implementation:  {scala_txn_per_s_ref:.0} txn/s ({:>6.0} txn/s)",
        txn_per_s_ave - scala_txn_per_s_ref
    );
}

// desc: "make 10 loops with txns-1E5"
#[allow(non_snake_case)]
fn id_fae31eb0_bd4a_483e_9eb7_9e4c36e7f785__loop_with_txns_1E1_10000() {
    let loops = 10_000;
    eprintln!("\n\nMake 10_000 loops with txns-1E1:");
    let mut r = 0;
    let mut loop_count = 0;
    for i in 1..loops + 1 {
        let result = parser::git_to_txns(
            Path::new(REPO_PATH),
            "txns/2016",
            "txn",
            GitInputSelector::Reference("txns-1E1".to_string()),
            &Settings::default_audit(),
        );
        verify_git_run(result, TXN_SET_1E1_COMMIT_ID, TXN_SET_1E1_CHECKSUM);

        let print_round = i / (loops / 10);
        if r < print_round {
            eprintln!("Done: {i:10.0} of {loops} loops");
            r = print_round;
        }
        loop_count += 1;
    }
    assert_eq!(loop_count, 10000);
}

fn main() {
    id_fae31eb0_bd4a_483e_9eb7_9e4c36e7f785__loop_with_txns_1E1_10000();

    id_33d85471_a04c_49b9_b7a0_9d7f7f5762eb__loop_with_txns_1E5_10x();
}
