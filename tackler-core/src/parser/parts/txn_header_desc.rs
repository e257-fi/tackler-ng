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
use winnow::ascii::till_line_ending;
use winnow::combinator::preceded;
use winnow::{PResult, Parser};

pub(crate) fn parse_txn_description<'s>(is: &mut Stream<'s>) -> PResult<&'s str> {
    let desc = preceded('\'', till_line_ending).parse_next(is)?;

    Ok(desc.trim_end())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::Settings;

    #[test]
    fn test_txn_description() {
        let mut settings = Settings::default();
        let input = "''hello winnow!  ";
        let mut is = Stream {
            input,
            state: &mut settings,
        };
        let res = parse_txn_description(&mut is);
        assert_eq!(res.ok(), Some("'hello winnow!"));
    }
}
