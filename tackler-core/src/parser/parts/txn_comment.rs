/*
 * Tackler-NG 2024-2025
 *
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::parser::parts::comment::p_comment;
use crate::parser::Stream;
use winnow::ascii::{line_ending, space1};
use winnow::{seq, PResult, Parser};

pub(crate) fn parse_txn_comment<'s>(is: &mut Stream<'s>) -> PResult<&'s str> {
    let m = seq!(
        _: space1,
        p_comment,
        _: line_ending
    )
    .parse_next(is)?;
    Ok(m.0)
}
