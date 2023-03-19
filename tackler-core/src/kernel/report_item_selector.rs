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

use crate::model::{BalanceTreeNode, RegisterEntry, RegisterPosting};
use regex::RegexSet;
use std::error::Error;
use tackler_api::Checksum;

pub trait Filtering<T> {
    ///
    /// Predicate to test if item x is part of set or not
    ///
    /// `x` item to be tested
    /// `returns` true if it's selected, false if it's rejected
    fn predicate(&self, x: &T) -> bool;
}

pub trait ReportItemSelector {
    fn checksum(&self) -> Result<Checksum, Box<dyn Error>>;
}

pub trait BalanceItemSelector: Filtering<BalanceTreeNode> {}

#[derive(Default)]
pub struct BalanceAllSelector {}

impl BalanceItemSelector for BalanceAllSelector {}

impl ReportItemSelector for BalanceAllSelector {
    fn checksum(&self) -> Result<Checksum, Box<dyn Error>> {
        todo!()
    }
}

impl Filtering<BalanceTreeNode> for BalanceAllSelector {
    fn predicate(&self, _: &BalanceTreeNode) -> bool {
        true
    }
}

pub struct BalanceByAccountSelector {
    regexs: RegexSet,
}

impl BalanceByAccountSelector {
    pub fn from(patterns: &[&str]) -> Result<BalanceByAccountSelector, Box<dyn Error>> {
        let bfa = BalanceByAccountSelector {
            regexs: RegexSet::new(patterns)?,
        };
        Ok(bfa)
    }
}

impl BalanceItemSelector for BalanceByAccountSelector {}

impl Filtering<BalanceTreeNode> for BalanceByAccountSelector {
    fn predicate(&self, btn: &BalanceTreeNode) -> bool {
        self.regexs.is_match(&btn.acctn.account)
    }
}

impl ReportItemSelector for BalanceByAccountSelector {
    fn checksum(&self) -> Result<Checksum, Box<dyn Error>> {
        todo!()
    }
}

pub trait RegisterItemSelector<'a>: Filtering<RegisterPosting<'a>> {}

pub struct RegisterByAccountSelector {
    regexs: RegexSet,
}

impl RegisterByAccountSelector {
    pub fn from(patterns: &[&str]) -> Result<RegisterByAccountSelector, Box<dyn Error>> {
        let ras = RegisterByAccountSelector {
            regexs: RegexSet::new(patterns)?,
        };
        Ok(ras)
    }
}

impl<'a> RegisterItemSelector<'a> for RegisterByAccountSelector {}

impl<'a> Filtering<RegisterPosting<'a>> for RegisterByAccountSelector {
    fn predicate(&self, rep: &RegisterPosting) -> bool {
        self.regexs.is_match(&rep.post.acctn.account)
    }
}

impl ReportItemSelector for RegisterByAccountSelector {
    fn checksum(&self) -> Result<Checksum, Box<dyn Error>> {
        todo!()
    }
}
