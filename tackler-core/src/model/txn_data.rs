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

use crate::kernel;
use crate::model::Txns;
use tackler_api::{Checksum, Metadata, MetadataItem, TxnSetChecksum};

#[derive(Debug)]
pub struct TxnData {
    pub metadata: Option<Metadata>,
    pub txns: Txns,
    pub algorithm: Option<String>,
}

impl TxnData {
    //   def apply(imdi: Option[InputMetadataItem], txns: Txns, settingsOpt: Option[Settings]): TxnData = {
    pub fn from(
        mdi_opt: Option<MetadataItem>,
        txns: Txns,
        hash_opt: Option<&str>,
    ) -> Result<TxnData, Box<dyn Error>> {
        // todo: settings
        let md_stuff = match hash_opt {
            Some(hash) => {
                let cs = calc_txn_checksum(&txns, hash)?;
                let algorithm = String::from(hash);

                let mut metadata = Metadata::new();
                let new_mdi = MetadataItem::TxnSetChecksum(TxnSetChecksum {
                    size: txns.len(),
                    hash: cs,
                });

                if let Some(mdi) = mdi_opt {
                    metadata.items.push(mdi);
                }
                metadata.items.push(new_mdi);

                (Some(metadata), Some(algorithm))
            }
            None => {
                if let Some(mdi) = mdi_opt {
                    let mut metadata = Metadata::new();
                    metadata.items.push(mdi);
                    (Some(metadata), None)
                } else {
                    (None, None)
                }
            }
        };

        Ok(TxnData {
            metadata: md_stuff.0,
            txns,
            algorithm: md_stuff.1,
        })
    }
}

fn calc_txn_checksum(txns: &Txns, hash: &str) -> Result<Checksum, Box<dyn Error>> {
    let uuids: Result<Vec<String>, Box<dyn Error>> = txns
        .iter()
        .map(|txn| match txn.header.uuid {
            Some(uuid) => Ok(uuid.to_string()),
            None => {
                let msg = "Txn without UUID. It is mandatory with transaction set checksum.";
                Err(msg.into())
            }
        })
        .collect();

    let mut u = uuids?;
    u.sort();

    let dups: Vec<String> = u.iter().duplicates().cloned().collect();
    if !dups.is_empty() {
        let dups_count = dups.len();
        let msg = if dups_count < 10 {
            format!(
                "Found {} duplicate txn uuids with txn set checksum.\nDuplicate ids are:\n{}",
                dups.len(),
                dups.join(",\n")
            )
        } else {
            format!("Found {} duplicate txn uuids with txn set checksum.\nFirst ten duplicate ids are:\n{}", dups.len(), dups[0..10].join(",\n"))
        };
        return Err(msg.into());
    }

    let cs = kernel::hash::checksum(hash, &u, "\n".as_bytes())?;
    Ok(cs)
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    #[test]
    // desc: check that uuid::to_string returns normalized lower-case UUID
    fn uuid_as_lower_case() {
        let uuid_ref = "e274c99e-1ebb-45e8-832d-58caf54ed95f";
        let uuid_mixed = "E274C99E-1ebb-45e8-832d-58Caf54Ed95f";
        let uuid_upper = "E274C99E-1EBB-45E8-832D-58CAF54ED95F";

        assert_eq!(Uuid::parse_str(uuid_ref).unwrap().to_string(), uuid_ref);
        assert_eq!(Uuid::parse_str(uuid_mixed).unwrap().to_string(), uuid_ref);
        assert_eq!(Uuid::parse_str(uuid_upper).unwrap().to_string(), uuid_ref);
    }
}
