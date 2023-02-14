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

use crate::model::transaction::Transaction;
use tackler_api::txn_ts;

fn txn_ts_to_string(txn: &Transaction) -> String {
    txn_ts::iso_zoned_ts(txn.header.timestamp)
}
fn txn_code_to_string(txn: &Transaction) -> String {
    txn.header.code.as_ref().unwrap().to_string()
}
fn txn_desc_to_string(txn: &Transaction) -> String {
    txn.header.description.as_ref().unwrap().to_string()
}
fn txn_uuid_to_string(txn: &Transaction) -> String {
    txn.header.uuid.as_ref().unwrap().to_string()
}
fn txn_geo_to_string(txn: &Transaction) -> String {
    format!("{}", &txn.header.location.as_ref().unwrap())
}
fn txn_tags_to_string(txn: &Transaction) -> String {
    txn.header.tags_to_string()
}

mod tackler_txns;
mod txn_header_code;
mod txn_header_desc;
mod txn_header_timestamp;
mod txn_location;
mod txn_metadata;
mod txn_tags;
mod txn_uuid;
mod txn_accounts;
mod txn_commodities;
