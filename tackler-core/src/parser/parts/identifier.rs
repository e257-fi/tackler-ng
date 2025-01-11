/*
 * Tackler-NG 2024-2025
 *
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::parser::Stream;
use winnow::combinator::repeat;
use winnow::token::{one_of, take_while};
use winnow::{PResult, Parser};
/*
ID: NameStartChar (NameChar)*;

SUBID: (NameStartChar | DIGIT) (NameChar)*;

fragment
NameChar
   : NameStartChar
   | DIGIT
   | '_'
   | '-'
   | '\u00B7'
   | '\u0300'..'\u036F'
   | '\u203F'..'\u2040'
   ;

fragment
NameStartChar
   : '$' | '¢' | '£' | '¤' | '¥' // common currency symbols which are not in block 20A0-20CF
   | '\u00B5' //  Micro Sign
   | '\u00B9' | '\u00B2' | '\u00B3' // Superscript 1, 2, 3 (Latin-1 Supplement)
   | '\u00B0' // Degree Sign
   | '\u00BC' | '\u00BD' | '\u00BE' // Vulgar Fraction: 1/4, 1/2, 3/4 (Latin-1 Supplement)
   | 'A'..'Z' | 'a'..'z'
   | '\u00C0'..'\u00D6'
   | '\u00D8'..'\u00F6'
   | '\u00F8'..'\u02FF'
   | '\u0370'..'\u037D'
   | '\u037F'..'\u1FFF'
   | '\u200C'..'\u200D'
   | '\u2070'..'\u218F'
   | '\u2C00'..'\u2FEF'
   | '\u3001'..'\uD7FF'
   | '\uF900'..'\uFDCF'
   | '\uFDF0'..'\uFFFD'
   ;
 */

fn id_char(c: char) -> bool {
    id_start_char(c)
        | matches!(
            c,
            |'0'..='9' // rustfmt
            | '_' | '-' | '\u{00B7}' // middle tod
            | '\u{0300}'..='\u{036F}' // rustfmt
            | '\u{203F}'..='\u{2040}' // rustfmt
        )
}

fn id_start_char(c: char) -> bool {
    matches!(c,
        'a'..='z'
        | 'A'..='Z'
        | '$' | '¢' | '£' | '¤' | '¥' // common currency symbols which are not in block 20A0-20CF
        | '\u{00C0}'..='\u{00D6}'
        | '\u{00D8}'..='\u{00F6}'
        | '\u{00F8}'..='\u{02FF}'
        | '\u{0370}'..='\u{037D}'
        | '\u{037F}'..='\u{1FFF}'
        | '\u{200C}'..='\u{200D}'
        | '\u{2070}'..='\u{218F}'
        | '\u{2C00}'..='\u{2FEF}'
        | '\u{3001}'..='\u{D7FF}'
        | '\u{F900}'..='\u{FDCF}'
        | '\u{FDF0}'..='\u{FFFD}'
        | '\u{00B5}' //  Micro Sign
        | '\u{00B9}' | '\u{00B2}' | '\u{00B3}'  // Superscript 1, 2, 3 (Latin-1 Supplement)
        | '\u{00B0}' // Degree Sign
        | '\u{00BC}' | '\u{00BD}' | '\u{00BE}' // Vulgar Fraction: 1/4, 1/2, 3/4 (Latin-1 Supplement)
    )
}

pub(crate) fn p_id_part<'s>(is: &mut Stream<'s>) -> PResult<&'s str> {
    take_while(1.., id_char).take().parse_next(is)
}

pub(crate) fn p_identifier<'s>(is: &mut Stream<'s>) -> PResult<&'s str> {
    let res_str = (one_of(id_start_char), take_while(0.., id_char))
        .take()
        .parse_next(is)?;
    Ok(res_str)
}

fn p_id_part_helper<'s>(is: &mut Stream<'s>) -> PResult<&'s str> {
    (take_while(1, ':'), p_id_part).take().parse_next(is)
}

pub(crate) fn p_multi_part_id<'s>(is: &mut Stream<'s>) -> PResult<&'s str> {
    let dec_str = (
        p_identifier,
        repeat(0.., p_id_part_helper).fold(String::new, |mut string, s| {
            string.push_str(s);
            string
        }),
    )
        .take()
        .parse_next(is)?;

    Ok(dec_str)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::Settings;

    #[test]
    fn test_p_id() {
        let mut settings = Settings::default();
        let input = "abcABCäöåÄÖÅ$€£";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        let res = p_identifier(&mut is);

        assert!(res.is_ok());
        assert_eq!(input, res.unwrap(/*:test:*/));
    }
    #[test]
    fn test_p_sub_id() {
        let mut settings = Settings::default();
        let input = "1234abcABCäöåÄÖÅ$€£";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        let res = p_id_part(&mut is);

        assert!(res.is_ok());
        assert_eq!(input, res.unwrap(/*:test:*/));
    }
}
