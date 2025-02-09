/*
 * Tackler-NG 2024-2025
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::parser::{Stream, make_semantic_error};
use rust_decimal::Decimal;
use winnow::combinator::{opt, preceded};
use winnow::stream::AsChar;
use winnow::token::take_while;
use winnow::{PResult, Parser};

pub(crate) fn p_number(is: &mut Stream<'_>) -> PResult<Decimal> {
    let dec_str: &str = (
        opt('-'),
        take_while(1.., AsChar::is_dec_digit),
        opt(preceded('.', take_while(1.., AsChar::is_dec_digit))),
    )
        .take()
        .parse_next(is)?;

    match Decimal::from_str_exact(dec_str) {
        Ok(d) => Ok(d),
        Err(err) => Err(make_semantic_error(is, err.to_string().as_str())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::Settings;

    #[test]
    fn test_p_number_integer() {
        let mut settings = Settings::default();
        let input = "123";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        let res = p_number(&mut is);

        assert!(res.is_ok());
        let dec = res.unwrap(/*:test:*/);
        assert_eq!(dec, Decimal::from_str_exact("123").unwrap(/*:test:*/));
    }

    #[test]
    fn test_p_number_positive() {
        let mut settings = Settings::default();
        let input = "1.23";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        let res = p_number(&mut is);

        assert!(res.is_ok());
        let dec = res.unwrap(/*:test:*/);
        assert_eq!(dec, Decimal::from_str_exact("1.23").unwrap(/*:test:*/));
    }

    #[test]
    fn test_p_number_negative() {
        let mut settings = Settings::default();
        let input = "-123456789.987654321";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        let res = p_number(&mut is);

        assert!(res.is_ok());
        let dec = res.unwrap(/*:test:*/);
        assert_eq!(
            dec,
            Decimal::from_str_exact("-123456789.987654321").unwrap(/*:test:*/)
        );
    }
}
