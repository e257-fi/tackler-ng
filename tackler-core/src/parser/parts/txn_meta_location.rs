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

use crate::parser::parts::number::p_number;
use crate::parser::Stream;
use tackler_api::location::GeoPoint;
use winnow::ascii::{line_ending, space0, space1};
use winnow::combinator::{cut_err, fail, opt, preceded};
use winnow::error::{StrContext, StrContextValue};
use winnow::{seq, PResult, Parser};

const CTX_LABEL: &str = "txn metadata location";

fn p_geo_uri(is: &mut Stream<'_>) -> PResult<GeoPoint> {
    let (lat, lon, alt) = seq!(
        _: cut_err("geo:")
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description("'geo:' uri specifier'")))
            .context(StrContext::Expected(StrContextValue::Description(" # location: geo: lat, lon [,alt]"))),
        _: space0,
        cut_err(p_number)
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description("valid latitude"))),
        _: space0,
        _: cut_err(',')
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description("',' after latitude"))),
        _: space0,
        cut_err(p_number)
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description("valid longitude"))),
        _: space0,
        opt(preceded(
            ',',
            preceded(
                space0,
                cut_err(p_number)
                    .context(StrContext::Label(CTX_LABEL))
                    .context(StrContext::Expected(StrContextValue::Description("valid altitude"))),
            )))
    )
    .parse_next(is)?;

    match GeoPoint::from(lat, lon, alt) {
        Ok(point) => Ok(point),
        Err(_err) => fail(is),
    }
}

pub(crate) fn parse_meta_location(is: &mut Stream<'_>) -> PResult<GeoPoint> {
    let geo = seq!(
        _: space1,
        _: '#',
        _: cut_err(space1)
            .context(StrContext::Label("txn metadata"))
            .context(StrContext::Expected(StrContextValue::Description("space after '#'"))),
        _: "location:",
        _: cut_err(space1)
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description("space after 'location:'"))),
        cut_err(p_geo_uri)
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description("valid geo uri"))),
        _: space0,
        _: cut_err(line_ending)
            .context(StrContext::Label(CTX_LABEL))
            .context(StrContext::Expected(StrContextValue::Description("line ending"))),
    )
    .parse_next(is)?;

    Ok(geo.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::Settings;

    #[test]
    fn test_p_geo_uri() {
        let mut settings = Settings::default();
        let input = "geo:66.5436,25.84715,160";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        let res = p_geo_uri(&mut is);

        assert!(res.is_ok());
        let _geo = res.unwrap(/*:test:*/);
        //assert_eq!(geo, "geo:66.5436,25.84715,160");
    }

    #[test]
    fn test_parse_meta_location() {
        let mut settings = Settings::default();
        let input = " # location: geo:66.5436,25.84715,160\n";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        let res = parse_meta_location(&mut is);

        assert!(res.is_ok());
        let _geo = res.unwrap(/*:test:*/);
        //assert_eq!(format!("{geo}"), "geo:66.5436,25.84715,160");
    }
}
