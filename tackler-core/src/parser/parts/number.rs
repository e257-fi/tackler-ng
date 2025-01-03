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
use rust_decimal::Decimal;
use winnow::combinator::{fail, opt, preceded};
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
        Err(_err) => fail(is),
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
