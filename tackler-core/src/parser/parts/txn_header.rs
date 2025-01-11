/*
 * Tackler-NG 2024-2025
 *
 * SPDX-License-Identifier: Apache-2.0
 */

use std::fmt::Write;
use time::OffsetDateTime;
use winnow::{seq, PResult, Parser};

use crate::parser::parts::timestamp::parse_timestamp;
use crate::parser::parts::txn_comment::parse_txn_comment;
use crate::parser::parts::txn_header_code::parse_txn_code;
use crate::parser::parts::txn_header_desc::parse_txn_description;
use crate::parser::parts::txn_metadata::{parse_txn_meta, TxnMeta};
use crate::parser::{from_error, make_semantic_error, Stream};
use tackler_api::txn_header::TxnHeader;
use winnow::ascii::{line_ending, space1};
use winnow::combinator::{cut_err, opt, preceded, repeat};
use winnow::error::{StrContext, StrContextValue};

#[allow(clippy::type_complexity)]
pub(crate) fn parse_txn_header(is: &mut Stream<'_>) -> PResult<TxnHeader> {
    let (ts, code, desc, meta, comments): (
        jiff::Zoned,
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
        let _ = write!(msg, "\n   txn date: {}", ts); // todo: format this with rcf3339
        let _ = write!(
            msg,
            "{}",
            code.map(|c| format!("\n   txn code: {c}"))
                .unwrap_or_default()
        );
        return Err(make_semantic_error(is, msg.as_str()));
    }

    Ok(TxnHeader {
        timestamp: jiff_to_time(is, ts)?,
        code: code.map(String::from),
        description: desc.map(String::from),
        uuid: meta.as_ref().and_then(|t| t.uuid),
        location: meta.as_ref().and_then(|t| t.location.clone()),
        tags: meta.and_then(|t| t.tags.clone()),
        comments: comments.map(|v| v.into_iter().map(String::from).collect()),
    })
}

// todo: remove this once transition to jiff is done
fn jiff_to_time(is: &mut Stream<'_>, ts: jiff::Zoned) -> PResult<OffsetDateTime> {
    let y = ts.date().year() as i32;
    let m = ts.date().month();
    let d = ts.date().day() as u8;

    let month = match time::Month::try_from(m as u8) {
        Ok(m) => m,
        Err(err) => return Err(from_error(is, &err)),
    };

    let date = match time::Date::from_calendar_date(y, month, d) {
        Ok(d) => d,
        Err(err) => return Err(from_error(is, &err)),
    };

    let time = match time::Time::from_hms_nano(
        ts.time().hour() as u8,
        ts.time().minute() as u8,
        ts.time().second() as u8,
        ts.time().subsec_nanosecond() as u32,
    ) {
        Ok(t) => t,
        Err(err) => return Err(from_error(is, &err)),
    };
    let offset = ts.offset().seconds();
    let offset_time = match time::UtcOffset::from_whole_seconds(offset) {
        Ok(t) => t,
        Err(err) => return Err(from_error(is, &err)),
    };

    let dt = OffsetDateTime::new_in_offset(date, time, offset_time);

    Ok(dt)
}
