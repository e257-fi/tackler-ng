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

use std::fmt::Write;
use winnow::{seq, PResult, Parser};

use crate::model::{Transaction, Txns};
use crate::parser::parts::txn_header::parse_txn_header;
use crate::parser::parts::txn_postings::parse_txn_postings;
use crate::parser::Stream;
use std::error::Error;
use winnow::ascii::{line_ending, multispace0, space0};
use winnow::combinator::{cut_err, eof, fail, opt, preceded, repeat, repeat_till};
use winnow::error::StrContext;

fn multispace0_line_ending<'s>(is: &mut Stream<'s>) -> PResult<&'s str> {
    // space0 can't be multispace0 as it's greedy and eats away the last line ending
    repeat(1.., (space0, line_ending))
        .map(|()| ())
        .parse_next(is)?;
    Ok("")
}

fn parse_txn(is: &mut Stream<'_>) -> PResult<Transaction> {
    let txn = seq!(
        cut_err(parse_txn_header)
            .context(StrContext::Label("Txn Header")),
        cut_err(parse_txn_postings)
            .context(StrContext::Label("Txn Postings")),
        _: multispace0,
    )
    .context(StrContext::Label("Transaction"))
    .parse_next(is)?;

    match Transaction::from(txn.0, txn.1) {
        Ok(txn) => Ok(txn),
        Err(_err) => fail(is),
    }
}

pub(crate) fn parse_txns(input: &mut Stream<'_>) -> Result<Txns, Box<dyn Error>> {
    let txns: PResult<(Vec<Transaction>, &str)> = preceded(
        opt(multispace0_line_ending),
        repeat_till(1.., parse_txn, eof),
    )
    .parse_next(input);

    match txns {
        Ok(txns) => Ok(txns.0),
        Err(err) => {
            let mut msg = "Failed to parse transaction\n".to_string();
            //let _ = writeln!(msg, "Error: {}", err);
            let i = input.input.lines().next().unwrap_or(input.input);
            let i_err = if i.chars().count() < 1024 {
                i.to_string()
            } else {
                i.chars().take(1024).collect::<String>()
            };

            let _ = write!(msg, "Failed input:\n{}\n\n", i_err);
            match err.into_inner() {
                Some(ce) => {
                    let _ = writeln!(msg, "Detailed error:");
                    for c in ce.context() {
                        let _ = writeln!(msg, "   {}", c);
                    }
                }
                None => {
                    let _ = write!(msg, "No detailed error information available");
                }
            }

            Err(msg.into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::Settings;
    use indoc::indoc;
    use tackler_rs::IndocUtils;

    #[test]
    fn test_parse_txns() {
        #[rustfmt::skip]
        let pok_txns = vec![
            (indoc!(
               "|2025-01-03
                | e 1
                | a -1
                |"
            ).strip_margin(), 1usize),
            (indoc!(
               "|
                |2025-01-03
                | e 1
                | a
                |"
            ).strip_margin(), 1usize),
            (indoc!(
               "|  \t \n\
                |2025-01-03
                | e 1
                | a
                |"
            ).strip_margin(), 1usize),
            (indoc!(
               "|\t \n\
                | \t  \t
                |2025-01-03
                | e 1
                | a
                |"
            ).strip_margin(), 1usize),
            (indoc!(
               "|2025-01-03
                | e 1
                | a
                |
                |"
            ).strip_margin(), 1usize),
            (indoc!(
               "|2025-01-03
                | e 1
                | a
                |\t \n\
                | \t  \t
                |"
            ).strip_margin(), 1usize),
            (indoc!(
               "|2025-01-03
                | e 1
                | a -1
                |
                |2025-01-03
                | e 1
                | a
                |
                |2025-01-03
                | e 1
                | a
                |"
            ).strip_margin(), 3usize),
            (indoc!(
               "|2025-01-03
                | e 1
                | a -1
                |\t \n\
                | \t  \t
                |2025-01-03
                | e 1
                | a
                |"
            ).strip_margin(), 2usize),
        ];

        let mut count = 0;
        for t in pok_txns {
            let mut settings = Settings::default();
            let mut is = Stream {
                input: t.0.as_str(),
                state: &mut settings,
            };

            let res = parse_txns(&mut is);
            assert!(
                res.is_ok(),
                "\nPOK is error: Offending test vector item: {}\n",
                count + 1
            );

            let txns = res.unwrap(/*:test:*/);
            assert_eq!(
                txns.len(),
                t.1,
                "\nWrong Txns count: Offending test vector item: {}",
                count + 1
            );

            count += 1;
        }
        assert_eq!(count, 8);
    }
}
