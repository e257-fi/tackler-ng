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

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Error, Formatter};

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

    // // TXN Header
    // TxnFilterTxnTSBegin(TxnFilterTxnTSBegin),
    // TxnFilterTxnTSEnd(TxnFilterTxnTSEnd),
    // TxnFilterTxnCode(TxnFilterTxnCode),
    // TxnFilterTxnDescription(TxnFilterTxnDescription),
    // TxnFilterTxnUUID(TxnFilterTxnUUID),
    // TxnFilterBBoxLatLon(TxnFilterBBoxLatLon),
    // TxnFilterBBoxLatLonAlt(TxnFilterBBoxLatLonAlt),
    // TxnFilterTxnTags(TxnFilterTxnTags),
    // TxnFilterTxnComments(TxnFilterTxnComments),
    //
    // // TXN Postings
    // TxnFilterPostingAccount(TxnFilterPostingAccount),
    // TxnFilterPostingComment(TxnFilterPostingComment),
    // TxnFilterPostingAmountEqual(TxnFilterPostingAmountEqual),
    // TxnFilterPostingAmountLess(TxnFilterPostingAmountLess),
    // TxnFilterPostingAmountGreater(TxnFilterPostingAmountGreater),
    // TxnFilterPostingCommodity(TxnFilterPostingCommodity),

    //
    PropFilter(PropFilter),
    TsFilter(TsFilter),
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

            // prop filters
            TxnFilter::PropFilter(tf) => tf.i_fmt(indent, f),
            TxnFilter::TsFilter(tf) => tf.i_fmt(indent, f),
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

#[derive(Serialize, Deserialize, Debug)]
pub struct PropFilter {
    pub regex: String,
}

impl IndentDisplay for PropFilter {
    fn i_fmt(&self, indent: &str, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{indent}prop filter: {}", self.regex)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TsFilter {
    pub end: DateTime<FixedOffset>,
}

impl IndentDisplay for TsFilter {
    fn i_fmt(&self, indent: &str, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{indent}timestamp filter: {}", self.end)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FilterDefinition {
    #[serde(rename = "txnFilter")]
    pub txn_filter: TxnFilter,
}

impl Display for FilterDefinition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Filter:\n{}", self.txn_filter)
    }
}
