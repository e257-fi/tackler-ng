/*
 * Tackler-NG 2024-2025
 *
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::parser::Stream;
use winnow::stream::AsChar;
use winnow::token::one_of;
use winnow::{
    ascii::till_line_ending,
    error::{StrContext, StrContextValue},
};
use winnow::{seq, PResult, Parser};

pub(crate) fn p_comment<'s>(is: &mut Stream<'s>) -> PResult<&'s str> {
    let m = seq!(
        _: (
            ';',
            // this can not be space1 as we must preserve space for equity and identity reports
            one_of(AsChar::is_space)
        ).context(StrContext::Expected(StrContextValue::Description("comment begins with a `;` and a space character"))),
        till_line_ending,
    )
    .parse_next(is)?;
    Ok(m.0)
}
