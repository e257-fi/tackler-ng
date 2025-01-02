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

use winnow::{seq, PResult, Parser};

use crate::parser::Stream;
use winnow::token::take_while;

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
        _: ')'
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
