/*
 * Tackler-NG 2024-2025
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::parser::Stream;
use winnow::ascii::till_line_ending;
use winnow::combinator::preceded;
use winnow::{ModalResult, Parser};

pub(crate) fn parse_txn_description<'s>(is: &mut Stream<'s>) -> ModalResult<&'s str> {
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
