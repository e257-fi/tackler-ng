/*
 * Tackler-NG 2024-2025
 *
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::parser::Stream;
use winnow::combinator::{alt, cut_err, peek};
use winnow::stream::AsChar;
use winnow::token::one_of;
use winnow::{
    ascii::line_ending,
    ascii::till_line_ending,
    error::{StrContext, StrContextValue},
};
use winnow::{seq, PResult, Parser};

pub(crate) fn p_comment<'s>(is: &mut Stream<'s>) -> PResult<&'s str> {
    let m = seq!(
        _: ';',
        cut_err(alt((
            // allow totally empty comment ";\n" - this is important for
            // txn body comments as some editors removes spaces at the end of line
            peek(line_ending).map(|_| {("",)}),
            seq!(
                // this can not be space1 as we must preserve space for equity and identity reports
                _: one_of(AsChar::is_space),
                till_line_ending
            )
        )).map(|x| x.0))
            .context(StrContext::Expected(StrContextValue::Description("comment begins with a `;` and a space character"))),
    )
    .map(|x| x.0)
    .parse_next(is)?;
    Ok(m)
}

// The line_end handling must work with outer contex,
// so for testing, see txn_comment.rs
