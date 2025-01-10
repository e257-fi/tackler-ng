/*
 * Copyright 2024-2025 E257.FI
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
