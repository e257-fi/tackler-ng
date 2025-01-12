/*
 * Tackler-NG 2023-2024
 *
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::config::Scale;
use crate::model::{Posting, Transaction};
use jiff::tz::TimeZone;
use jiff::{tz, Zoned};
use rust_decimal::{Decimal, RoundingStrategy};
use std::cmp::{max, Ordering};
use std::fmt::Write;
use std::fmt::{Display, Formatter};
use tackler_api::txn_ts;

#[derive(Debug, Clone)]
pub struct RegisterPosting<'a> {
    pub post: &'a Posting,
    pub amount: Decimal,
}

impl Eq for RegisterPosting<'_> {}

impl PartialEq<Self> for RegisterPosting<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.post.acctn.atn == other.post.acctn.atn
    }
}

impl PartialOrd<Self> for RegisterPosting<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RegisterPosting<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.post.acctn.cmp(&other.post.acctn)
    }
}

impl Display for RegisterPosting<'_> {
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

impl RegisterEntry<'_> {
    pub(crate) fn fmt_with_tz(
        &self,
        ts_fmtr: fn(&Zoned, TimeZone) -> String,
        tz: TimeZone,
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
impl Display for RegisterEntry<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.fmt_with_tz(
                |ts, _tz| { txn_ts::rfc_3339(ts) },
                tz::TimeZone::UTC,
                &Scale::default()
            )
        )
    }
}
