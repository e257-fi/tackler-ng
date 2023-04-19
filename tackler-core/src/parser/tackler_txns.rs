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

use itertools::Itertools;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::str;
use std::str::FromStr;
//use std::time::{SystemTime, UNIX_EPOCH};

use crate::model::{transaction, TxnData, Txns};
use crate::parser::tackler_parser;
use gix as git;

use crate::kernel::Settings;
use tackler_api::metadata::items::{GitInputReference, MetadataItem};

pub enum GitInputSelector {
    CommitId(String),
    Reference(String),
}

pub fn string_to_txns(input: &str, settings: &Settings) -> Result<TxnData, Box<dyn Error>> {
    let mut txns = tackler_parser::txns_text(input)?;

    // feature: a94d4a60-40dc-4ec0-97a3-eeb69399f01b
    // coverage: "sorted" tested by 200aad57-9275-4d16-bdad-2f1c484bcf17
    txns.sort_by(transaction::ord_by_txn);

    TxnData::from(None, txns, &settings.audit.hash)
}

pub fn paths_to_txns(paths: &[PathBuf], settings: &Settings) -> Result<TxnData, Box<dyn Error>> {
    let all_txns: Result<Txns, Box<dyn Error>> = paths
        .iter()
        .map(|p| tackler_parser::txns_file(p))
        .flatten_ok()
        .collect();

    let mut txns = all_txns?;
    txns.sort_by(transaction::ord_by_txn);

    TxnData::from(None, txns, &settings.audit.hash)
}

pub fn git_to_txns(
    repo_path: &Path,
    dir: &str,
    extension: &str,
    input_selector: GitInputSelector,
    settings: &Settings,
) -> Result<TxnData, Box<dyn Error>> {
    // perf: let mut ts_par_total: u128 = 0;
    // perf: let ts_start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap(/*:test:*/);

    let repo = git::discover(repo_path)?;

    let (object, reference) = match input_selector {
        GitInputSelector::CommitId(id) => {
            let object_id = gix::ObjectId::from_str(&id)?;
            (repo.find_object(object_id)?.try_into_commit()?, None)
        }
        GitInputSelector::Reference(ref_str) => {
            let git_ref = repo.find_reference(&ref_str)?;
            let id = git_ref.into_fully_peeled_id()?;
            (
                repo.find_object(id)?.try_into_commit()?,
                Some(ref_str.clone()),
            )
        }
    };

    let gitmd = GitInputReference {
        commit: object.id.to_string(),
        reference,
        dir: dir.to_string(),
        suffix: extension.to_string(),
        message: object.message()?.title.to_string(),
    };

    let tree = object.tree()?;
    // fixme: Optimization
    //      In the future, this could be optimized with custom walker,
    //      which does the filtering in the first place.
    let txns: Result<Txns, Box<dyn Error>> = tree
        .traverse()
        .breadthfirst
        .files()?
        .iter()
        .map(|entry| {
            use git::objs::tree::EntryMode::*;
            match entry.mode {
                Blob => {
                    if entry.filepath.starts_with(str::as_bytes(dir))
                        && entry.filepath.ends_with(str::as_bytes(extension))
                    {
                        let obj = repo.find_object(entry.oid)?;
                        // perf: let ts_par_start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap(/*:test:*/);

                        let par_res = tackler_parser::txns_text(str::from_utf8(&obj.data)?);

                        // perf: let ts_par_end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap(/*:test:*/);
                        // perf: ts_par_total = ts_par_total + (ts_par_end.as_millis() - ts_par_start.as_millis());
                        match par_res {
                            Ok(txns) => Ok(txns),
                            Err(err) => {
                                let msg = format!(
                                    "\
                                    GIT: Error while processing git object\n\
                                    \x20  commit id: {}\n\
                                    \x20  object id: {}\n\
                                    \x20  path: {}\n\
                                    \x20  msg: {}\
                                    ",
                                    object.id, obj.id, entry.filepath, err
                                );
                                Err(msg.into())
                            }
                        }
                    } else {
                        // It's blob but outside of our file path filter
                        Ok(Vec::default())
                    }
                }
                // It's not a blob
                _ => Ok(Vec::default()),
            }
        })
        .flatten_ok()
        .collect::<Result<Txns, Box<dyn Error>>>();

    let txn_data = TxnData::from(
        Some(MetadataItem::GitInputReference(gitmd)),
        txns?,
        &settings.audit.hash,
    );

    // perf: let ts_end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap(/*:test:*/);
    // perf: eprintln!("total time: {}ms, parse time: {}ms, git: {}ms", (ts_end.as_millis() - ts_start.as_millis()), ts_par_total, (ts_end.as_millis() - ts_start.as_millis())-ts_par_total);

    txn_data
}

#[cfg(test)]
mod tests {
    /*
    it("create git commitId by string") {
      assert(TacklerTxns.gitCommitId("1234567890") === Right[String, String]("1234567890"))
    }

    it("create git ref by settings") {
      val settings = Settings()
      assert(TacklerTxns.gitReference(settings) === Left[String, String]("master"))
    }

    it("create git ref by string") {
      assert(TacklerTxns.gitReference("unit-test-ref") === Left[String, String]("unit-test-ref"))
    }
    */
}
