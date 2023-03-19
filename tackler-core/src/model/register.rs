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
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

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
        writeln!(f, "{}", self.txn.header.timestamp)?;
        for p in &self.posts {
            writeln!(f, "   {}", p)?;
        }
        writeln!(f)
    }
}
