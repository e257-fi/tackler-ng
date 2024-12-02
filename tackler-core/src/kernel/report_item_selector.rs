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

use crate::kernel::hash::Hash;
use crate::kernel::Predicate;
use crate::model::{BalanceTreeNode, RegisterPosting};
use regex::RegexSet;
use std::error::Error;
use tackler_api::metadata::Checksum;

pub trait ReportItemSelector {
    fn checksum(&self, _: Hash) -> Result<Checksum, Box<dyn Error>>;
}

pub trait BalanceItemSelector: Predicate<BalanceTreeNode> {}
pub trait BalanceSelector: BalanceItemSelector + ReportItemSelector {}

#[derive(Default)]
pub struct BalanceAllSelector {}

impl BalanceSelector for BalanceAllSelector {}
impl BalanceItemSelector for BalanceAllSelector {}

impl ReportItemSelector for BalanceAllSelector {
    fn checksum(&self, _hash: Hash) -> Result<Checksum, Box<dyn Error>> {
        Ok(Checksum {
            algorithm: "None".to_string(),
            value: "select all".to_string(),
        })
    }
}

impl Predicate<BalanceTreeNode> for BalanceAllSelector {
    fn eval(&self, _: &BalanceTreeNode) -> bool {
        true
    }
}

#[derive(Default)]
pub struct BalanceNonZeroSelector {}
impl BalanceSelector for BalanceNonZeroSelector {}
impl BalanceItemSelector for BalanceNonZeroSelector {}

impl ReportItemSelector for BalanceNonZeroSelector {
    fn checksum(&self, _hash: Hash) -> Result<Checksum, Box<dyn Error>> {
        Ok(Checksum {
            algorithm: "None".to_string(),
            value: "select all non-zero".to_string(),
        })
    }
}

impl Predicate<BalanceTreeNode> for BalanceNonZeroSelector {
    fn eval(&self, btn: &BalanceTreeNode) -> bool {
        !btn.account_sum.is_zero()
    }
}

pub struct BalanceNonZeroByAccountSelector {
    acc_sel: BalanceByAccountSelector,
}
impl BalanceSelector for BalanceNonZeroByAccountSelector {}
impl BalanceItemSelector for BalanceNonZeroByAccountSelector {}

impl ReportItemSelector for BalanceNonZeroByAccountSelector {
    fn checksum(&self, hash: Hash) -> Result<Checksum, Box<dyn Error>> {
        self.acc_sel.checksum(hash)
    }
}

impl Predicate<BalanceTreeNode> for BalanceNonZeroByAccountSelector {
    fn eval(&self, btn: &BalanceTreeNode) -> bool {
        !btn.account_sum.is_zero() && self.acc_sel.eval(btn)
    }
}

impl BalanceNonZeroByAccountSelector {
    pub fn from(patterns: &[&str]) -> Result<BalanceNonZeroByAccountSelector, Box<dyn Error>> {
        let bfa = BalanceByAccountSelector {
            regexs: RegexSet::new(patterns)?,
        };
        let bnza = BalanceNonZeroByAccountSelector { acc_sel: bfa };
        Ok(bnza)
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
impl BalanceSelector for BalanceByAccountSelector {}

impl Predicate<BalanceTreeNode> for BalanceByAccountSelector {
    fn eval(&self, btn: &BalanceTreeNode) -> bool {
        self.regexs.is_match(&btn.acctn.atn.account)
    }
}

impl ReportItemSelector for BalanceByAccountSelector {
    fn checksum(&self, hash: Hash) -> Result<Checksum, Box<dyn Error>> {
        let mut accsel = self.regexs.patterns().to_vec();
        accsel.sort();
        let h = hash.checksum(&accsel, "\n".as_bytes())?;
        Ok(h)
    }
}

pub trait RegisterItemSelector<'a>: Predicate<RegisterPosting<'a>> {}
pub trait RegisterSelector<'a>: RegisterItemSelector<'a> + ReportItemSelector {}

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

impl RegisterSelector<'_> for RegisterByAccountSelector {}
impl RegisterItemSelector<'_> for RegisterByAccountSelector {}

impl Predicate<RegisterPosting<'_>> for RegisterByAccountSelector {
    fn eval(&self, rep: &RegisterPosting) -> bool {
        self.regexs.is_match(&rep.post.acctn.atn.account)
    }
}

impl ReportItemSelector for RegisterByAccountSelector {
    fn checksum(&self, hash: Hash) -> Result<Checksum, Box<dyn Error>> {
        let mut accsel = self.regexs.patterns().to_vec();
        accsel.sort();
        let h = hash.checksum(&accsel, "\n".as_bytes())?;
        Ok(h)
    }
}

#[derive(Default)]
pub struct RegisterAllSelector {}

impl Predicate<RegisterPosting<'_>> for RegisterAllSelector {
    fn eval(&self, _: &RegisterPosting) -> bool {
        true
    }
}

impl RegisterItemSelector<'_> for RegisterAllSelector {}
impl RegisterSelector<'_> for RegisterAllSelector {}

impl ReportItemSelector for RegisterAllSelector {
    fn checksum(&self, _hash: Hash) -> Result<Checksum, Box<dyn Error>> {
        Ok(Checksum {
            algorithm: "None".to_string(),
            value: "select all".to_string(),
        })
    }
}
