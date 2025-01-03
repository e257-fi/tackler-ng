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
use crate::kernel::Settings;
use crate::parser::parts::identifier::p_multi_part_id;
use crate::parser::Stream;
use itertools::Itertools;
use std::error::Error;
use tackler_api::txn_header::Tags;
use winnow::ascii::{line_ending, space0, space1};
use winnow::combinator::{fail, repeat};
use winnow::{seq, PResult, Parser};

fn handle_tags(v: Vec<&str>, settings: &mut Settings) -> Result<Tags, Box<dyn Error>> {
    let mut tags = Vec::with_capacity(v.len());

    for t in v {
        let at = settings.get_or_create_tag(t)?; // todo: fix
        tags.push(at);
    }

    if tags.len() != tags.iter().unique().count() {
        let msg = if tags.len() < 1024 {
            format!("txn tags contains duplicate tags: {tags:?}")
        } else {
            format!(
                "txn tags contains duplicate tags, and size of tags is: {}",
                tags.len()
            )
        };
        return Err(msg.into());
    }

    Ok(tags)
}

fn p_tags(is: &mut Stream<'_>) -> PResult<Tags> {
    let mut tags = (
        p_multi_part_id,
        repeat(
            0..,
            seq!(
             _: space0,
                _: ',',
                _: space0,
                p_multi_part_id,
            ),
        )
        .fold(Vec::new, |mut acc, x| {
            acc.push(x.0);
            acc
        }),
    )
        .parse_next(is)?;

    let mut v = Vec::with_capacity(tags.1.len());
    v.push(tags.0);
    v.append(&mut tags.1);

    match handle_tags(v, is.state) {
        Ok(tags) => Ok(tags),
        Err(_err) => fail(is),
    }
}

pub(crate) fn parse_meta_tags(is: &mut Stream<'_>) -> PResult<Tags> {
    let tags = seq!(
        _: space1,
        _: '#',
        _: space1,
        _: "tags:",
        _: space1,
        p_tags,
        _: space0,
        _: line_ending
    )
    .parse_next(is)?;

    Ok(tags.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::Settings;

    #[test]
    fn test_p_tags() {
        let mut settings = Settings::default();
        let input = "first, second, third";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        let res = p_tags(&mut is);

        assert!(res.is_ok());
        let tags = res.unwrap(/*:test:*/);
        //assert_eq!(tags, ["first", "second", "third"]);
        assert_eq!(tags.len(), 3);
    }

    #[test]
    fn test_p_tags_err() {
        let mut settings = Settings::default();
        let input = " # tags: first, , third \n";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        let res = parse_meta_tags(&mut is);

        assert!(res.is_err());
    }

    #[test]
    fn test_parse_meta_tags() {
        let mut settings = Settings::default();
        let input = " # tags: a, first:second:third \n";
        let mut is = Stream {
            input,
            state: &mut settings,
        };

        let res = parse_meta_tags(&mut is);

        assert!(res.is_ok());
        let tags = res.unwrap(/*:test:*/);
        //assert_eq!(tags, ["a", "first:second:third"]);
        assert_eq!(tags.len(), 2);
    }
}
