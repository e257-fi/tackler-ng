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

use crate::model::{Posting, Transaction};
use rust_decimal::Decimal;
use std::cmp::{max, Ordering};
use std::fmt::{Display, Formatter};
use tackler_api::txn_ts;

#[derive(Debug, Clone)]
pub struct RegisterPosting<'a> {
    pub post: &'a Posting,
    pub amount: Decimal,
}

impl<'a> Eq for RegisterPosting<'a> {}

impl<'a> PartialEq<Self> for RegisterPosting<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.post.acctn == other.post.acctn
    }
}

impl<'a> PartialOrd<Self> for RegisterPosting<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.post.acctn.partial_cmp(&other.post.acctn)
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
            self.post.acctn.account, self.post.amount, self.amount
        )
    }
}

#[derive(Debug)]
pub(crate) struct RegisterEntry<'a> {
    pub txn: &'a Transaction,
    pub posts: Vec<RegisterPosting<'a>>,
}

impl<'a> Display for RegisterEntry<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let indent = " ".repeat(12);
        let mut line_len = 0;

        write!(
            f,
            "{}",
            self.txn
                .header
                .to_string_with_indent(&indent, txn_ts::rfc_3339)
        )?;
        for p in &self.posts {
            let line = format!(
                "{}{:<33}{:>18.prec$} {:>18.prec$}{}",
                indent,
                p.post.acctn.account,
                p.post.amount,
                p.amount,
                p.post
                    .acctn
                    .commodity
                    .as_ref()
                    .map_or(String::default(), |c| format!(" {}", c.name)),
                prec = 2,
            );
            line_len = max(line_len, line.len());
            writeln!(f, "{line}")?;
        }
        writeln!(f, "{}", "-".repeat(line_len))
    }
}
