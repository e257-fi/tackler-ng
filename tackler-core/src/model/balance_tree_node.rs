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

use crate::model::TxnAccount;
use rust_decimal::Decimal;
use std::cmp::Ordering;

#[derive(Debug, Eq, Clone)]
pub struct BalanceTreeNode {
    pub(crate) acctn: TxnAccount,
    pub(crate) sub_acc_tree_sum: Decimal,
    pub(crate) account_sum: Decimal,
}

pub(crate) fn ord_by_btn(before: &BalanceTreeNode, after: &BalanceTreeNode) -> Ordering {
    before.cmp(after)
}

impl Ord for BalanceTreeNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.acctn.cmp(&other.acctn)
    }
}

impl PartialOrd for BalanceTreeNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.acctn.cmp(&other.acctn))
    }
}

impl PartialEq for BalanceTreeNode {
    fn eq(&self, other: &Self) -> bool {
        self.acctn == other.acctn
    }
}
