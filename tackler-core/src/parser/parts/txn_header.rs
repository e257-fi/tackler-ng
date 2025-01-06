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
use time::OffsetDateTime;
use winnow::{seq, PResult, Parser};

use crate::parser::parts::timestamp::parse_timestamp;
use crate::parser::parts::txn_comment::parse_txn_comment;
use crate::parser::parts::txn_header_code::parse_txn_code;
use crate::parser::parts::txn_header_desc::parse_txn_description;
use crate::parser::parts::txn_metadata::{parse_txn_meta, TxnMeta};
use crate::parser::{make_semantic_error, Stream};
use tackler_api::txn_header::TxnHeader;
use tackler_api::txn_ts;
use winnow::ascii::{line_ending, space1};
use winnow::combinator::{cut_err, opt, preceded, repeat};
use winnow::error::{StrContext, StrContextValue};

#[allow(clippy::type_complexity)]
pub(crate) fn parse_txn_header(is: &mut Stream<'_>) -> PResult<TxnHeader> {
    let (ts, code, desc, meta, comments): (
        OffsetDateTime,
        Option<&str>,
        Option<&str>,
        Option<TxnMeta>,
        Option<Vec<&str>>,
    ) = seq!(
        parse_timestamp,
        opt(preceded(space1, parse_txn_code)),
        opt(preceded(space1, parse_txn_description)),
        _: preceded(opt(space1),
            cut_err(line_ending)
                .context(StrContext::Label("Txn Header"))
                .context(StrContext::Expected(StrContextValue::Description("format: timestamp [(code)] ['description]"))),
        ),
        opt(parse_txn_meta),
        opt(repeat(1.., parse_txn_comment))
    )
    .parse_next(is)?;

    if is.state.audit_mode && meta.as_ref().is_none_or(|m| m.uuid.is_none()) {
        let mut msg = "Audit mode is activated and there is a txn without UUID".to_string();
        let _ = write!(msg, "\n   txn date: {}", txn_ts::rfc_3339(ts));
        let _ = write!(
            msg,
            "{}",
            code.map(|c| format!("\n   txn code: {c}"))
                .unwrap_or_default()
        );
        return Err(make_semantic_error(is, msg.as_str()));
    }

    Ok(TxnHeader {
        timestamp: ts,
        code: code.map(String::from),
        description: desc.map(String::from),
        uuid: meta.as_ref().and_then(|t| t.uuid),
        location: meta.as_ref().and_then(|t| t.location.clone()),
        tags: meta.and_then(|t| t.tags.clone()),
        comments: comments.map(|v| v.into_iter().map(String::from).collect()),
    })
}
