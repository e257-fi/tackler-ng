/*
 * Tackler-NG 2024-2025
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::parser::Stream;
use winnow::combinator::cut_err;
use winnow::error::StrContext;
use winnow::token::take_while;
use winnow::{PResult, Parser, seq};

fn valid_code_char(c: char) -> bool {
    !matches!(
        c,
        ')' | '\'' | '(' | '[' | ']' | '{' | '}' | '<' | '>' | '\r' | '\n'
    )
}

pub(crate) fn parse_txn_code<'s>(is: &mut Stream<'s>) -> PResult<&'s str> {
    let code = seq!(
        _: '(',
        take_while(0..,valid_code_char),
        _: cut_err(')')
            .context(StrContext::Label("code")),
    )
    .parse_next(is)?;

    Ok(code.0.trim())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::Settings;

    #[test]
    fn test_txn_code() {
        let mut settings = Settings::default();
        let input = "(#foo)";
        let mut is = Stream {
            input,
            state: &mut settings,
        };
        let res = parse_txn_code(&mut is);
        assert_eq!(res.ok(), Some("#foo"));
    }
}
