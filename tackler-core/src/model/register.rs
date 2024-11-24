/*
 * Copyright 2023-2024 E257.FI
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

use crate::config::Scale;
use crate::model::{Posting, Transaction};
use rust_decimal::{Decimal, RoundingStrategy};
use std::cmp::{max, Ordering};
use std::fmt::{Display, Formatter};
use tackler_api::txn_ts;
use time::OffsetDateTime;
use time_tz::Tz;

#[derive(Debug, Clone)]
pub struct RegisterPosting<'a> {
    pub post: &'a Posting,
    pub amount: Decimal,
}

impl<'a> Eq for RegisterPosting<'a> {}

impl<'a> PartialEq<Self> for RegisterPosting<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.post.acctn.atn == other.post.acctn.atn
    }
}

impl<'a> PartialOrd<Self> for RegisterPosting<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for RegisterPosting<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.post.acctn.cmp(&other.post.acctn)
    }
}

impl<'a> Display for RegisterPosting<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.post.acctn.atn.account, self.post.amount, self.amount
        )
    }
}

#[derive(Debug)]
pub(crate) struct RegisterEntry<'a> {
    pub txn: &'a Transaction,
    pub posts: Vec<RegisterPosting<'a>>,
}

impl<'a> RegisterEntry<'a> {
    pub fn fmt_with_tz(
        &self,
        ts_fmtr: fn(OffsetDateTime, &'static Tz) -> String,
        tz: &'static Tz,
        scale: &Scale,
    ) -> String {
        let indent = " ".repeat(12);
        let mut line_len = 0;
        let mut s = self.txn.header.to_string_with_indent(&indent, ts_fmtr, tz);

        for p in &self.posts {
            let comm = &p.post.acctn.comm;

            let prec_1 = scale.get_precision(&p.post.amount);
            let prec_2 = scale.get_precision(&p.amount);

            let line = format!(
                "{}{:<33}{:>18.prec_1$} {:>18.prec_2$}{}",
                indent,
                p.post.acctn.atn.account,
                p.post
                    .amount
                    .round_dp_with_strategy(prec_1 as u32, RoundingStrategy::MidpointAwayFromZero),
                p.amount
                    .round_dp_with_strategy(prec_2 as u32, RoundingStrategy::MidpointAwayFromZero),
                match &comm.is_some() {
                    true => format!(" {}", comm.name),
                    false => String::new(),
                },
            );
            line_len = max(line_len, line.chars().count());
            s += &line;
            s += "\n";
        }
        format!("{s}{}\n", "-".repeat(line_len))
    }
}
impl<'a> Display for RegisterEntry<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.fmt_with_tz(
                |ts, _tz| { txn_ts::rfc_3339(ts) },
                txn_ts::TZ_UTC,
                &Scale::default()
            )
        )
    }
}
