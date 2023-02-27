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

mod logic_and;
mod logic_not;
mod logic_or;
mod posting_account;
mod posting_amount_equal;
mod posting_amount_greater;
mod posting_amount_less;
mod posting_comment;
mod posting_commodity;
mod txn_bbox_lat_lon;
mod txn_bbox_lat_lon_alt;
mod txn_code;
mod txn_comments;
mod txn_description;
mod txn_tags;
mod txn_ts_begin;
mod txn_ts_end;
mod txn_uuid;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Error, Formatter};

use crate::filters::posting_account::TxnFilterPostingAccount;
use crate::filters::posting_amount_equal::TxnFilterPostingAmountEqual;
use crate::filters::posting_amount_greater::TxnFilterPostingAmountGreater;
use crate::filters::posting_amount_less::TxnFilterPostingAmountLess;
use crate::filters::posting_comment::TxnFilterPostingComment;
use crate::filters::posting_commodity::TxnFilterPostingCommodity;
use crate::filters::txn_bbox_lat_lon::TxnFilterBBoxLatLon;
use crate::filters::txn_bbox_lat_lon_alt::TxnFilterBBoxLatLonAlt;
use crate::filters::txn_code::TxnFilterTxnCode;
use crate::filters::txn_comments::TxnFilterTxnComments;
use crate::filters::txn_description::TxnFilterTxnDescription;
use crate::filters::txn_tags::TxnFilterTxnTags;
use crate::filters::txn_ts_begin::TxnFilterTxnTSBegin;
use crate::filters::txn_ts_end::TxnFilterTxnTSEnd;
use crate::filters::txn_uuid::TxnFilterTxnUUID;
pub use logic_and::TxnFilterAND;
pub use logic_not::TxnFilterNOT;
pub use logic_or::TxnFilterOR;

pub trait IndentDisplay {
    fn i_fmt(&self, indent: &str, f: &mut Formatter<'_>) -> std::fmt::Result;
}

/// Data models for transaction filters.
///
/// Actual filtering implementation is done by Trait [`FilterTxn`]
///
/// [`FilterTxn`]: ../tackler_core/filter/index.html
#[derive(Serialize, Deserialize, Debug)]
pub enum TxnFilter {
    // Nullary test filters
    NullaryTRUE(NullaryTRUE),
    NullaryFALSE(NullaryFALSE),

    // Logic filters
    TxnFilterAND(TxnFilterAND),
    TxnFilterOR(TxnFilterOR),
    TxnFilterNOT(TxnFilterNOT),

    // TXN Header filters
    TxnFilterTxnTSBegin(TxnFilterTxnTSBegin),
    TxnFilterTxnTSEnd(TxnFilterTxnTSEnd),
    TxnFilterTxnCode(TxnFilterTxnCode),
    TxnFilterTxnDescription(TxnFilterTxnDescription),
    TxnFilterTxnUUID(TxnFilterTxnUUID),
    TxnFilterBBoxLatLon(TxnFilterBBoxLatLon),
    TxnFilterBBoxLatLonAlt(TxnFilterBBoxLatLonAlt),
    TxnFilterTxnTags(TxnFilterTxnTags),
    TxnFilterTxnComments(TxnFilterTxnComments),

    // TXN Postings
    TxnFilterPostingAccount(TxnFilterPostingAccount),
    TxnFilterPostingComment(TxnFilterPostingComment),
    TxnFilterPostingAmountEqual(TxnFilterPostingAmountEqual),
    TxnFilterPostingAmountLess(TxnFilterPostingAmountLess),
    TxnFilterPostingAmountGreater(TxnFilterPostingAmountGreater),
    TxnFilterPostingCommodity(TxnFilterPostingCommodity),
}

impl Display for TxnFilter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.i_fmt("", f)
    }
}

impl IndentDisplay for TxnFilter {
    fn i_fmt(&self, indent: &str, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            // specials
            TxnFilter::NullaryTRUE(tf) => tf.i_fmt(indent, f),
            TxnFilter::NullaryFALSE(tf) => tf.i_fmt(indent, f),

            // logic filters
            TxnFilter::TxnFilterAND(tf) => tf.i_fmt(indent, f),
            TxnFilter::TxnFilterOR(tf) => tf.i_fmt(indent, f),
            TxnFilter::TxnFilterNOT(tf) => tf.i_fmt(indent, f),

            // txn header filters
            TxnFilter::TxnFilterTxnTSBegin(tf) => tf.i_fmt(indent, f),
            TxnFilter::TxnFilterTxnTSEnd(tf) => tf.i_fmt(indent, f),
            TxnFilter::TxnFilterTxnCode(tf) => tf.i_fmt(indent, f),
            TxnFilter::TxnFilterTxnDescription(tf) => tf.i_fmt(indent, f),
            TxnFilter::TxnFilterTxnUUID(tf) => tf.i_fmt(indent, f),
            TxnFilter::TxnFilterBBoxLatLon(tf) => tf.i_fmt(indent, f),
            TxnFilter::TxnFilterBBoxLatLonAlt(tf) => tf.i_fmt(indent, f),
            TxnFilter::TxnFilterTxnTags(tf) => tf.i_fmt(indent, f),
            TxnFilter::TxnFilterTxnComments(tf) => tf.i_fmt(indent, f),

            // posting filters
            TxnFilter::TxnFilterPostingAccount(tf) => tf.i_fmt(indent, f),
            TxnFilter::TxnFilterPostingComment(tf) => tf.i_fmt(indent, f),
            TxnFilter::TxnFilterPostingAmountEqual(tf) => tf.i_fmt(indent, f),
            TxnFilter::TxnFilterPostingAmountLess(tf) => tf.i_fmt(indent, f),
            TxnFilter::TxnFilterPostingAmountGreater(tf) => tf.i_fmt(indent, f),
            TxnFilter::TxnFilterPostingCommodity(tf) => tf.i_fmt(indent, f),
        }
    }
}

/// Special filter which will always return true
#[derive(Serialize, Deserialize, Debug)]
pub struct NullaryTRUE {}

impl IndentDisplay for NullaryTRUE {
    fn i_fmt(&self, indent: &str, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{indent}All pass")
    }
}

/// Special filter which will always return false
#[derive(Serialize, Deserialize, Debug)]
pub struct NullaryFALSE {}
impl IndentDisplay for NullaryFALSE {
    fn i_fmt(&self, indent: &str, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{indent}None pass")
    }
}

fn logic_filter_indent_fmt(
    op: &str,
    indent: &str,
    filters: &[TxnFilter],
    f: &mut Formatter<'_>,
) -> std::fmt::Result {
    let new_ident = format!("{indent}  ");

    writeln!(f, "{indent}{op}")?;
    let result: Result<Vec<()>, Error> = filters.iter().map(|tf| tf.i_fmt(&new_ident, f)).collect();

    match result {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

fn posting_filter_indent_fmt(
    indent: &str,
    target: &str,
    regex: &str,
    op: &str,
    amount: &Decimal,
    f: &mut Formatter<'_>,
) -> std::fmt::Result {
    let my_indent = format!("{indent}  ");
    writeln!(f, "{indent}{target}")?;
    writeln!(f, "{my_indent}account: \"{regex}\"")?;
    writeln!(f, "{my_indent}amount {op} {amount}")
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FilterDefinition {
    #[serde(rename = "txnFilter")]
    pub txn_filter: TxnFilter,
}

impl Display for FilterDefinition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Filter:")?;
        self.txn_filter.i_fmt("  ", f)
    }
}
