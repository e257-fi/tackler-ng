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
use uuid::Uuid;
use winnow::ascii::{line_ending, space1, till_line_ending};
use winnow::combinator::fail;
use winnow::Parser;
use winnow::{seq, PResult};

fn p_uuid(is: &mut Stream<'_>) -> PResult<Uuid> {
    // todo: fix this and check uuid from bytes
    let uuid_str = till_line_ending.parse_next(is)?;

    match Uuid::parse_str(uuid_str.trim()) {
        Ok(uuid) => Ok(uuid),
        Err(_err) => fail(is),
    }
}

pub(crate) fn parse_meta_uuid(is: &mut Stream<'_>) -> PResult<Uuid> {
    let uuid = seq!(
        _: space1,
        _: '#',
        _: space1,
        _: "uuid:",
        _: space1,
        p_uuid,
        _: line_ending
    )
    .parse_next(is)?;

    Ok(uuid.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::Settings;

    #[test]
    fn test_p_uuid() {
        let mut settings = Settings::default();
        let input = "e009c181-45f3-4286-bd4c-b0e091c3ba47";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        let res = p_uuid(&mut is);

        assert!(res.is_ok());
        let uuid = res.unwrap(/*:test:*/);
        assert_eq!(
            uuid,
            Uuid::parse_str("e009c181-45f3-4286-bd4c-b0e091c3ba47").unwrap(/*:test:*/)
        );
    }

    #[test]
    fn test_parse_meta_uuid() {
        let mut settings = Settings::default();
        let input = " # uuid: c51270e7-305d-40a3-a132-f9ed4b135da7\n";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        let res = parse_meta_uuid(&mut is);

        assert!(res.is_ok());
        let uuid = res.unwrap(/*:test:*/);
        assert_eq!(
            uuid,
            Uuid::parse_str("c51270e7-305d-40a3-a132-f9ed4b135da7").unwrap(/*:test:*/)
        );
    }
}
