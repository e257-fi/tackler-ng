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
use std::fmt::Write;
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
        fn amount_to_string(amount: &Decimal, scale: &Scale, width: usize) -> String {
            let prec = scale.get_precision(amount);
            let amount_txt = format!(
                "{:.prec$}",
                amount.round_dp_with_strategy(prec as u32, RoundingStrategy::MidpointAwayFromZero)
            );

            if amount.is_sign_positive() && amount_txt.chars().count() >= width {
                format!(" {}", amount_txt)
            } else {
                amount_txt
            }
        }

        let indent = " ".repeat(12);
        let mut line_len = 0;
        let mut reg_entry_txt = self.txn.header.to_string_with_indent(&indent, ts_fmtr, tz);

        for p in &self.posts {
            let comm = &p.post.acctn.comm;

            let line = format!(
                "{}{:<33}{:>18} {:>18}{}",
                indent,
                p.post.acctn.atn.account,
                amount_to_string(&p.post.amount, scale, 18),
                amount_to_string(&p.amount, scale, 18),
                match &comm.is_some() {
                    true => format!(" {}", comm.name),
                    false => String::new(),
                },
            );
            line_len = max(line_len, line.chars().count());
            let _ = writeln!(reg_entry_txt, "{}", line);
        }
        let _ = writeln!(reg_entry_txt, "{}", "-".repeat(line_len));
        reg_entry_txt
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
