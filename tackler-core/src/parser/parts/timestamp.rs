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
use std::error::Error;
use winnow::{seq, PResult, Parser};

use std::str::FromStr;
use time::{Date, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset};
use winnow::combinator::{alt, fail, opt, preceded};
use winnow::error::{StrContext, StrContextValue};
use winnow::stream::AsChar;
use winnow::token::take_while;

use crate::parser::Stream;

fn p_date(is: &mut Stream<'_>) -> PResult<Date> {
    let (y, m_u8, d) = seq!(
        take_while(4, AsChar::is_dec_digit).try_map(i32::from_str),
        _: "-",
        take_while(2, AsChar::is_dec_digit).try_map(u8::from_str),
        _: "-",
        take_while(2, AsChar::is_dec_digit).try_map(u8::from_str)
    )
    .parse_next(is)?;

    let m = match time::Month::try_from(m_u8) {
        Ok(m) => m,
        Err(_err) => return fail(is),
    };
    match Date::from_calendar_date(y, m, d) {
        Ok(d) => Ok(d),
        Err(_err) => fail(is),
    }
}

fn parse_date(is: &mut Stream<'_>) -> PResult<OffsetDateTime> {
    let date = p_date(is)?;

    match is.state.get_offset_date(date) {
        Ok(date) => Ok(date),
        Err(_err) => fail(is),
    }
}

fn handle_time(h: u8, m: u8, s: u8, ns_opt: Option<&str>) -> Result<Time, Box<dyn Error>> {
    let t = match ns_opt {
        Some(ns_str) => {
            let left_ns = u32::from_str(ns_str)?;
            let ns_len = ns_str.chars().count();
            assert!(ns_len <= 9);

            match ns_len {
                0..=3 => {
                    let ms = left_ns * 10u32.pow(3 - ns_len as u32);
                    Time::from_hms_milli(h, m, s, ms as u16)?
                }
                4..=6 => {
                    let micro_s = left_ns * 10u32.pow(6 - ns_len as u32);
                    Time::from_hms_micro(h, m, s, micro_s)?
                }
                7..=9 => {
                    let ns = left_ns * 10u32.pow(9 - ns_len as u32);
                    Time::from_hms_nano(h, m, s, ns)?
                }
                _ => {
                    unreachable!()
                }
            }
        }
        None => Time::from_hms(h, m, s)?,
    };
    Ok(t)
}

fn p_datetime(is: &mut Stream<'_>) -> PResult<PrimitiveDateTime> {
    let (date, h, m, s, ns_opt) = seq!(
        p_date,
        _: "T",
        take_while(2, AsChar::is_dec_digit).try_map(u8::from_str),
        _: ":",
        take_while(2, AsChar::is_dec_digit).try_map(u8::from_str),
        _: ":",
        take_while(2, AsChar::is_dec_digit).try_map(u8::from_str),
        opt(preceded('.',
            take_while(1..=9, AsChar::is_dec_digit),
        ))
    )
    .parse_next(is)?;

    let time = match handle_time(h, m, s, ns_opt) {
        Ok(t) => t,
        Err(_err) => return fail(is),
    };

    Ok(PrimitiveDateTime::new(date, time))
}

fn parse_datetime(is: &mut Stream<'_>) -> PResult<OffsetDateTime> {
    let dt = p_datetime(is)?;

    match is.state.get_offset_datetime(dt) {
        Ok(dt) => Ok(dt),
        Err(_err) => fail(is),
    }
}

fn p_datetime_tz(is: &mut Stream<'_>) -> PResult<UtcOffset> {
    let (sign, h, m) = alt((
        'Z'.map(|_| (1i8, 0i8, 0i8)),
        seq!(
            alt(('+'.value(1i8), '-'.value(-1i8))),
            take_while(2, AsChar::is_dec_digit).try_map(i8::from_str),
            _: ":",
            take_while(2, AsChar::is_dec_digit).try_map(i8::from_str),
        ),
    ))
    .parse_next(is)?;

    match UtcOffset::from_hms(sign * h, sign * m, 0) {
        Ok(offset) => Ok(offset),
        Err(_err) => fail(is),
    }
}

fn parse_datetime_tz(is: &mut Stream<'_>) -> PResult<OffsetDateTime> {
    let (ts, tz) = seq!(p_datetime, p_datetime_tz,).parse_next(is)?;

    let ts_tz = ts.assume_offset(tz);

    Ok(ts_tz)
}

pub(crate) fn parse_timestamp(is: &mut Stream<'_>) -> PResult<OffsetDateTime> {
    let ts = alt((
        parse_datetime_tz.context(StrContext::Expected(StrContextValue::Description("ts_tz"))),
        parse_datetime.context(StrContext::Expected(StrContextValue::Description("ts"))),
        parse_date.context(StrContext::Expected(StrContextValue::Description("date"))),
    ))
    .parse_next(is)?;
    Ok(ts)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::Settings;

    #[test]
    fn test_p_date() {
        let mut settings = Settings::default();
        let input = "2024-12-30";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        assert!(p_date(&mut is).is_ok());
    }

    #[test]
    fn test_p_datetime() {
        let mut settings = Settings::default();
        let input = "2024-12-30T20:21:22";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        assert!(p_datetime(&mut is).is_ok());
    }

    #[test]
    fn test_p_datetime_zulu() {
        let mut settings = Settings::default();
        let input = "2024-12-30T20:21:22Z";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        assert!(p_datetime(&mut is).is_ok());
    }

    #[test]
    fn test_p_datetime_offset() {
        let mut settings = Settings::default();
        let input = "2024-12-30T20:21:22+02:00";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        assert!(p_datetime(&mut is).is_ok());
    }

    #[test]
    fn test_p_datetime_milli() {
        let mut settings = Settings::default();
        let input = "2024-12-30T20:21:22.12";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        assert!(p_datetime(&mut is).is_ok());
    }
    #[test]
    fn test_p_datetime_micro() {
        let mut settings = Settings::default();
        let input = "2024-12-30T20:21:22.12345";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        assert!(p_datetime(&mut is).is_ok());
    }
    #[test]
    fn test_p_datetime_nano() {
        let mut settings = Settings::default();
        let input = "2024-12-30T20:21:22.12345678";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        assert!(p_datetime(&mut is).is_ok());
    }
    #[test]
    fn test_p_datetime_nano_offset() {
        let mut settings = Settings::default();
        let input = "2024-12-30T20:21:22.123456789+02:00";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        assert!(p_datetime(&mut is).is_ok());
    }
    #[test]
    fn test_p_datetime_nano_zulu() {
        let mut settings = Settings::default();
        let input = "2024-12-30T20:21:22.123456789Z";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        assert!(p_datetime(&mut is).is_ok());
    }

    #[test]
    fn test_p_datetime_nano_err() {
        let mut settings = Settings::default();
        let input = "2024-12-30T20:21:22.1234567890+02:00";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        assert!(parse_datetime_tz(&mut is).is_err());
    }
}
