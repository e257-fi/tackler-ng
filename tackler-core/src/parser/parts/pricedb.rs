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

use winnow::{
    ascii::{line_ending, space0, space1},
    combinator::opt,
    error::{StrContext, StrContextValue},
    seq, PResult, Parser,
};

use crate::model::price_entry::PriceEntry;
use crate::parser::{from_error, parts::timestamp::parse_timestamp, Stream};

use super::{comment::p_comment, identifier::p_identifier, number::p_number};

#[allow(clippy::type_complexity)]
pub(crate) fn parse_price_entry(is: &mut Stream<'_>) -> PResult<PriceEntry> {
    let (timestamp, base_commodity, eq_amount, eq_commodity, comments) = seq!(
        _: 'P'.context(StrContext::Expected(StrContextValue::Description("price entry starts with `P`"))),
        _: space1,
        parse_timestamp,
        _: space1,
        p_identifier
            .context(StrContext::Expected(StrContextValue::Description("price entry must have base commodity"))),
        _: space1,
        p_number
            .context(StrContext::Expected(StrContextValue::Description("price entry must have equivalent amount"))),
        _: space1,
        p_identifier
            .context(StrContext::Expected(StrContextValue::Description("price entry must have equivalent commodity"))),
        _: space0,
        opt(p_comment),
        _: line_ending,
    )
    .parse_next(is)?;

    let base_commodity = is
        .state
        .get_or_create_commodity(Some(base_commodity))
        .map_err(|e| from_error(is, &*e))?;

    let eq_commodity = is
        .state
        .get_or_create_commodity(Some(eq_commodity))
        .map_err(|e| from_error(is, &*e))?;

    let comments = comments.map(String::from);

    Ok(PriceEntry {
        timestamp,
        base_commodity,
        eq_amount,
        eq_commodity,
        comments,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::Settings;

    #[test]
    fn test_parse_price_entry() {
        let tests = [
            "P 2024-12-30                XAU 2659.64 USD\n",
            "P 2024-12-30T20:21:22       XAU 2659.64 USD ; space\n",
            "P 2024-12-30                XAU 2659.64 USD; no space\n",
            "P 2024-12-30T20:21:22Z      XAU 2659.64 USD\n",
            "P 2024-12-30T20:21:22+02:00 XAU 2659.64 USD\n",
            "P 2024-12-30T20:21:22.12    XAU 2659.64 USD\n",
        ];

        for s in tests {
            let mut settings = Settings::default();

            let mut is = Stream {
                input: s,
                state: &mut settings,
            };

            let res = parse_price_entry(&mut is);

            assert!(res.is_ok());
        }
    }
}
