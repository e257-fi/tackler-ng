/*
 * Copyright 2024-2025 E257.FI and Muhammad Ragib Hasin
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

use rust_decimal::Decimal;
use std::{fmt::Write, sync::Arc};
use time::OffsetDateTime;
use winnow::{seq, PResult, Parser};

use crate::parser::parts::timestamp::parse_timestamp;
// use crate::parser::parts::txn_comment::parse_txn_comment;
// use crate::parser::parts::txn_header_code::parse_txn_code;
// use crate::parser::parts::txn_header_desc::parse_txn_description;
// use crate::parser::parts::txn_metadata::{parse_txn_meta, TxnMeta};
use crate::parser::{from_error, make_semantic_error, Stream};
use tackler_api::txn_header::{Comments, TxnHeader};
use winnow::ascii::{line_ending, space1};
use winnow::combinator::{cut_err, opt, preceded, repeat};
use winnow::error::{StrContext, StrContextValue};

use super::Commodity;

/// Entry in the price database
#[derive(Debug)]
pub struct PriceEntry {
    /// Timestamp with Zone information
    pub timestamp: jiff::Zoned,
    /// The commodity for which price is being noted
    pub base_commodity: Arc<Commodity>,
    /// Price of base in _eq_ commodity
    pub eq_amount: Decimal,
    /// The equivalence commodity in which price is being noted
    pub eq_commodity: Arc<Commodity>,
    /// Comments
    pub comments: Option<String>,
}
