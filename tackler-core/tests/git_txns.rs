/*
 * Tackler-NG 2019-2024
 *
 * SPDX-License-Identifier: Apache-2.0
 */
#![cfg_attr(rustfmt, rustfmt_skip)]
use std::error::Error;
use std::path::Path;
use tackler_api::metadata::items::MetadataItem;
use tackler_core::kernel::Settings;
use tackler_core::model::TxnData;
use tackler_core::parser;
use tackler_core::parser::GitInputSelector;


// val cfg = ConfigFactory.parseString(
//     """
//       |{
//       |   #  ./ = non-forked JVM
//       |   # ../ = forked JVM
//       |   basedir = "../tests/audit/"
//       |   input {
//       |     storage = git
//       |     git {
//       |       repository = "audit-repo.git"
//       |       dir = "txns/2016"
//       |       suffix = ".txn"
//       |     }
//       |   }
//       |}
//     """.stripMargin)

const REPO_PATH: &str = "../suite/audit/audit-repo.git/";
const TXN_SET_1E1_CHECKSUM: &str = "9b29071e1bf228cfbd31ca2b8e7263212e4b86e51cfee1e8002c9b795ab03f76";
const TXN_SET_1E1_COMMIT_ID: &str = "4aa4e9797501c1aefc92f32dff30ab462dae5545";

const TXN_SET_1E5_CHECKSUM: &str = "27060dc1ebde35bebd8f7af2fd9815bc9949558d3e3c85919813cd80748c99a7";
const TXN_SET_1E5_COMMIT_ID: &str = "cb56fdcdd2b56d41fc08cc5af4a3b410896f03b5";

#[rustfmt::skip]
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
                            panic!(/*:test:*/
                                   "The first item is not Git Input Metadata item")
                        }
                    }
                    match &md.items[1] {
                        MetadataItem::TxnSetChecksum(tscsmd) => {
                            assert_eq!(tscsmd.hash.value, checksum);
                        }
                        _ => {
                            panic!(/*:test:*/
                                   "The second item is not Txn Set Checksum Metadata item")
                        }
                    }
                },
                None => {
                    panic!(/*:test:*/ "no metadata")
                },
            }
        },
        Err(err) => {
            panic!(/*:test:*/ "{:#?}", err);
        }
    }
}

#[test]
//desc: "handle ref with 10 (1E1) txns"
#[allow(non_snake_case)]
fn id_ce2e6523_ee83_46e7_a767_441c5b9f2802__normal_txns_1E1() {
    let result = parser::git_to_txns(Path::new(REPO_PATH), "txns/2016",
                                     "txn",
                                     GitInputSelector::Reference("txns-1E1".to_string()),
                                     &mut Settings::default_audit());
    verify_git_run(result, TXN_SET_1E1_COMMIT_ID, TXN_SET_1E1_CHECKSUM);
}

#[test]
//desc: "handle ref with 100_000 (1E5) txns"
#[allow(non_snake_case)]
fn id_074f5549_346c_4780_90a1_07d60ae0e79d__normal_txns_1E5() {
    let result = parser::git_to_txns(Path::new(REPO_PATH),
                                     "txns/2016",
                                     "txn",
                                     GitInputSelector::Reference("txns-1E5".to_string()),
                                     &mut Settings::default_audit());

    verify_git_run(result, TXN_SET_1E5_COMMIT_ID, TXN_SET_1E5_CHECKSUM);
}

#[test]
//desc: "report reasonable details in case of audit error"
#[allow(non_snake_case)]
fn id_a6cfe3b6_feec_4422_afbf_faeca5baf752__error_reporting() {
    // """GIT: Error while processing git object
    //   |   commit id: 63014ea235b23aa7330511a25bcba0b62cd33c6f
    //   |   object id: d87737611e7a2bc551117c77fadd06dbc2c848d8
    //   |   path: txns/2016/04/01/20160401T115935-25.txn
    //   |   msg : Configuration setting 'auditing.txn-set-checksum' is activated and there is txn without UUID.""".stripMargin

    let result = parser::git_to_txns(Path::new(REPO_PATH), "txns/2016",
                                     "txn",
                                     GitInputSelector::Reference("errs-1E2".to_string()),
                                     &mut Settings::default_audit());

    assert!(result.is_err());
    // todo: let msg = result.err().unwrap(/*:test:*/).to_string();
    // todo: assert!(msg.contains("63014ea235b23aa7330511a25bcba0b62cd33c6f"));
    // todo: assert!(msg.contains("d87737611e7a2bc551117c77fadd06dbc2c848d8"));
    // todo: assert!(msg.contains("without UUID"));
}
