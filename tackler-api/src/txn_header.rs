/*
 * Copyright 2023 E257.FI
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

use chrono::{DateTime, FixedOffset};
use std::cmp::Ordering;
use uuid::Uuid;

use crate::{GeoPoint, Tags};

#[derive(Default, Debug)]
pub struct TxnHeader {
    pub timestamp: DateTime<FixedOffset>,
    pub code: Option<String>,
    pub description: Option<String>,
    pub uuid: Option<Uuid>,
    pub location: Option<GeoPoint>,
    pub tags: Option<Tags>,
    pub comments: Option<Vec<String>>,
}

impl TxnHeader {
    pub fn tags_to_string(&self) -> String {
        match &self.tags {
            Some(t) => t.join(", "),
            None => "".to_string(),
        }
    }
    pub fn location_to_string(&self) -> String {
        match &self.location {
            Some(t) => t.to_string(),
            None => "".to_string(),
        }
    }
}
impl Ord for TxnHeader {
    fn cmp(&self, other: &Self) -> Ordering {
        let date_comp = self.timestamp.cmp(&other.timestamp);
        if date_comp.is_ne() {
            date_comp
        } else {
            let empty = String::default();

            let code_cmp = self
                .code
                .as_ref()
                .unwrap_or(&empty)
                .cmp(other.code.as_ref().unwrap_or(&empty));
            if code_cmp.is_ne() {
                code_cmp
            } else {
                let desc_cmp = self
                    .description
                    .as_ref()
                    .unwrap_or(&empty)
                    .cmp(other.description.as_ref().unwrap_or(&empty));
                if desc_cmp.is_ne() {
                    desc_cmp
                } else {
                    let uuid_this = self
                        .uuid
                        .as_ref()
                        .map(|u| u.to_string())
                        .unwrap_or_default();
                    let uuid_other = other
                        .uuid
                        .as_ref()
                        .map(|u| u.to_string())
                        .unwrap_or_default();

                    uuid_this.cmp(&uuid_other)
                }
            }
        }
    }
}

impl PartialOrd for TxnHeader {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for TxnHeader {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp
            && self.code == other.code
            && self.description == other.description
            && self.uuid == other.uuid
    }
}

impl Eq for TxnHeader {}

impl TxnHeader {
    pub fn to_string_with_indent(
        &self,
        indent: &str,
        ts_formatter: fn(DateTime<FixedOffset>) -> String,
    ) -> String {
        format!(
            "{}{}{}\n{}{}{}{}",
            // txn header line: ts, code, desc
            ts_formatter(self.timestamp),
            self.code
                .as_ref()
                .map_or_else(String::default, |c| format!(" ({c})")),
            self.description
                .as_ref()
                .map_or_else(String::default, |desc| format!(" '{desc}")),
            // metadata
            self.uuid
                .as_ref()
                .map_or_else(String::default, |uuid| format!("{indent}# uuid: {uuid}\n")),
            self.location
                .as_ref()
                .map_or_else(String::default, |geo| format!(
                    "{indent}# location: {geo}\n"
                )),
            self.tags
                .as_ref()
                .map_or_else(String::default, |tags| format!(
                    "{}# tags: {}\n",
                    indent,
                    tags.join(", ")
                )),
            // txn comments
            self.comments
                .as_ref()
                .map_or_else(String::default, |comments| {
                    comments
                        .iter()
                        .map(|c| format!("{indent}; {c}\n"))
                        .collect()
                })
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::IndocWithMarker;
    use indoc::formatdoc;
    use indoc::indoc;

    use crate::{txn_ts, TxnHeader};

    #[test]
    fn txn_header_display() {
        let ts = "2023-02-04T14:03:05.047974+02:00"
            .parse::<DateTime<FixedOffset>>()
            .unwrap(/*:test:*/);

        let uuid_str = "ed6d4110-f3c0-4770-87fc-b99e46572244";
        let uuid = Uuid::parse_str(uuid_str).unwrap(/*:test:*/);

        let geo = GeoPoint::from(60.167, 24.955, Some(5.0)).unwrap(/*:test:*/);

        let txn_tags = vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "a:b:c".to_string(),
        ];
        let comments = vec![
            "z 1st line".to_string(),
            "c 2nd line".to_string(),
            "a 3rd line".to_string(),
        ];

        let tests: Vec<(TxnHeader, String)> = vec![
            (
                TxnHeader {
                    timestamp: ts.clone(),
                    code: None,
                    description: None,
                    uuid: None,
                    location: None,
                    tags: None,
                    comments: None,
                },
                indoc!(
                    "|2023-02-04T14:03:05.047974+02:00
                     |"
                )
                .strip_margin(),
            ),
            (
                TxnHeader {
                    timestamp: ts.clone(),
                    code: Some("#123".to_string()),
                    description: None,
                    uuid: None,
                    location: None,
                    tags: None,
                    comments: None,
                },
                indoc!(
                    "|2023-02-04T14:03:05.047974+02:00 (#123)
                     |"
                )
                .strip_margin(),
            ),
            (
                TxnHeader {
                    timestamp: ts.clone(),
                    code: Some("#123".to_string()),
                    description: Some("desc".to_string()),
                    uuid: None,
                    location: None,
                    tags: None,
                    comments: None,
                },
                indoc! {
                    "|2023-02-04T14:03:05.047974+02:00 (#123) 'desc
                     |"
                }
                .strip_margin(),
            ),
            (
                TxnHeader {
                    timestamp: ts.clone(),
                    code: None,
                    description: Some("desc".to_string()),
                    uuid: None,
                    location: None,
                    tags: None,
                    comments: None,
                },
                indoc!(
                    "|2023-02-04T14:03:05.047974+02:00 'desc
                     |"
                )
                .strip_margin(),
            ),
            (
                TxnHeader {
                    timestamp: ts.clone(),
                    code: None,
                    description: Some("desc".to_string()),
                    uuid: Some(uuid.clone()),
                    location: None,
                    tags: None,
                    comments: None,
                },
                formatdoc!(
                    "|2023-02-04T14:03:05.047974+02:00 'desc
                     |   # uuid: {uuid_str}
                     |"
                )
                .strip_margin(),
            ),
            (
                TxnHeader {
                    timestamp: ts.clone(),
                    code: None,
                    description: Some("desc".to_string()),
                    uuid: None,
                    location: Some(geo.clone()),
                    tags: None,
                    comments: None,
                },
                indoc!(
                    "|2023-02-04T14:03:05.047974+02:00 'desc
                     |   # location: geo:60.167,24.955,5
                     |"
                )
                .strip_margin(),
            ),
            (
                TxnHeader {
                    timestamp: ts.clone(),
                    code: None,
                    description: Some("desc".to_string()),
                    uuid: None,
                    location: None,
                    tags: Some(txn_tags.clone()),
                    comments: None,
                },
                indoc!(
                    "|2023-02-04T14:03:05.047974+02:00 'desc
                     |   # tags: a, b, c, a:b:c
                     |"
                )
                .strip_margin(),
            ),
            (
                TxnHeader {
                    timestamp: ts.clone(),
                    code: None,
                    description: Some("desc".to_string()),
                    uuid: None,
                    location: None,
                    tags: None,
                    comments: Some(comments.clone()),
                },
                indoc!(
                    "|2023-02-04T14:03:05.047974+02:00 'desc
                     |   ; z 1st line
                     |   ; c 2nd line
                     |   ; a 3rd line
                     |"
                )
                .strip_margin(),
            ),
            (
                TxnHeader {
                    timestamp: ts.clone(),
                    code: None,
                    description: Some("desc".to_string()),
                    uuid: Some(uuid.clone()),
                    location: Some(geo.clone()),
                    tags: Some(txn_tags.clone()),
                    comments: Some(comments.clone()),
                },
                formatdoc!(
                    "|2023-02-04T14:03:05.047974+02:00 'desc
                     |   # uuid: {uuid_str}
                     |   # location: geo:60.167,24.955,5
                     |   # tags: a, b, c, a:b:c
                     |   ; z 1st line
                     |   ; c 2nd line
                     |   ; a 3rd line
                     |"
                )
                .strip_margin(),
            ),
        ];

        let mut count = 0;
        let should_be_count = tests.len();
        for t in tests {
            let txn_hdr_str = format!("{}", t.0.to_string_with_indent("   ", txn_ts::iso_zoned_ts));
            assert_eq!(txn_hdr_str, t.1);
            count = count + 1;
        }
        assert_eq!(count, should_be_count);
    }
}
