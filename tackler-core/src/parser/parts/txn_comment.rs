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
#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::Settings;

    #[test]
    fn test_txn_comment() {
        let comments = vec![
            (" ; comment\n", "comment"),
            ("\t;\tcomment\n", "comment"),
            (" \t; \tcomment\n", "\tcomment"),
            ("\t ;\t comment\n", " comment"),
            (" ; comment \n", "comment "),
            ("\t;\tcomment\t\n", "comment\t"),
            ("\t ;\n", ""),
            (" ;    \n", "   "),
            (" ;\t\t\n", "\t"),
            (" ; \t \n", "\t "),
        ];
        let mut settings = Settings::default();

        for c in comments {
            let mut is = Stream {
                input: c.0,
                state: &mut settings,
            };

            let res = parse_txn_comment(&mut is);

            assert!(res.is_ok());
            assert_eq!(c.1, res.unwrap(/*:test:*/));
        }
    }
}
