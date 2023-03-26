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

use crate::kernel::report_item_selector::BalanceSelector;
use crate::model::balance_tree_node::ord_by_btn;
use crate::model::{AccountTreeNode, BalanceTreeNode, Commodity, TxnData, Txns};
use itertools::Itertools;
use rust_decimal::Decimal;
use std::collections::{HashMap, HashSet};
use tackler_api::Metadata;

pub type Deltas = HashMap<Option<Commodity>, Decimal>;
pub type BTNs = Vec<BalanceTreeNode>;
#[derive(Debug)]
pub struct Balance {
    pub(crate) title: String,
    pub(crate) bal: BTNs,
    pub(crate) deltas: Deltas,
    pub(crate) metadata: Option<Metadata>,
}

impl Balance {
    fn is_empty(&self) -> bool {
        self.bal.is_empty()
    }
}

impl Balance {
    /// Recursive get balance tree nodes for this subtree
    /// starting from and defined by "me"
    ///
    /// Input size is "small";  ~ size of CoA
    /// Output size is "small"; ~ size of CoA
    ///
    /// `me` is name of root account for this sub-tree
    /// `acc_sums` is list of all account sums
    /// `returns` list of balance tree nodes for this sub-tree
    fn get_balance_tree_nodes(
        me: &(AccountTreeNode, Decimal),
        acc_sums: &[(AccountTreeNode, Decimal)],
    ) -> Vec<BalanceTreeNode> {
        let (my_acctn, my_sum) = me;

        // find my children (childs)
        let childs = acc_sums
            .iter()
            .filter(|(atn, _)| my_acctn.is_parent_of(atn));

        // calculate balance tree nodes of my childs
        let childs_balance_trees = childs
            .flat_map(|c| Balance::get_balance_tree_nodes(c, acc_sums))
            .collect::<Vec<BalanceTreeNode>>();

        // calculate top sum of my children's balance trees
        let my_childs_sum = childs_balance_trees
            .iter()
            .filter(|btn| btn.acctn.parent == my_acctn.account)
            .map(|btn| btn.sub_acc_tree_sum)
            .sum::<Decimal>();

        let my_btn = BalanceTreeNode {
            acctn: my_acctn.clone(),
            sub_acc_tree_sum: my_childs_sum + my_sum,
            account_sum: *my_sum,
        };

        let mut x = vec![my_btn];
        x.extend(childs_balance_trees);
        x
    }

    /// Bubble up from leafs to root, and generate any missing (gap)
    /// AccountTreeNode (ATN) for new ATN entry with zero atn sum.
    ///
    /// Input size is "small";  ~ size of CoA
    /// Output size is "small"; ~ size of CoA
    ///
    /// `my_acctn_sum` starting AccTNSum entry
    /// `acc_sums` current incomplete (in sense of Chart of Account) account sums
    /// `returns`  new set of AccTNSums without gaps from this branch to root (from leaf to root)
    fn bubble_up_acctn(
        my_acctn_sum: &(AccountTreeNode, Decimal),
        acc_sums: &[(AccountTreeNode, Decimal)],
    ) -> Vec<(AccountTreeNode, Decimal)> {
        let my_acctn = &my_acctn_sum.0;
        if my_acctn.depth == 1 {
            // we are on top, so either this node (my_acctn) exist already
            // or it has been created by its child;
            // End of recursion
            vec![my_acctn_sum.clone()]
        } else {
            // Not on top => find parent for this node
            let parent = acc_sums
                .iter()
                .filter(|(atn, _)| atn.is_parent_of(my_acctn))
                .collect::<Vec<&(AccountTreeNode, Decimal)>>();

            assert!(parent.is_empty() || parent.len() == 1);

            if parent.is_empty() {
                if my_acctn.depth > 2 {
                    // todo: the perfect tree of CoA
                    let new_parent_atn =
                        AccountTreeNode::from(my_acctn.parent.clone(), my_acctn.commodity.clone())
                            .unwrap();
                    let mut sub_tree = vec![my_acctn_sum.clone()];
                    let mut x =
                        Balance::bubble_up_acctn(&(new_parent_atn, Decimal::ZERO), acc_sums);
                    x.append(&mut sub_tree);
                    x
                } else {
                    // todo: the perfect tree of CoA
                    // This is on depth 2 and it doesn't have parent, => let's create root account
                    // End of Recursion
                    let new_parent_atn =
                        AccountTreeNode::from(my_acctn.parent.clone(), my_acctn.commodity.clone())
                            .unwrap();
                    vec![(new_parent_atn, Decimal::ZERO), my_acctn_sum.clone()]
                }
            } else {
                // Parent of this exists, just bubble them up together
                let mut sub_tree = vec![my_acctn_sum.clone()];
                let mut x = Balance::bubble_up_acctn(parent[0], acc_sums);
                x.append(&mut sub_tree);
                x
            }
        }
    }

    /// Calculate balance items
    ///
    /// Input size is "big";     ~ all transactions
    /// Output size is "small";  ~ size of CoA
    ///
    /// `txns` sequence of transactions
    /// `returns` unfiltered sequence of BalanceTreeNodes
    fn balance(txns: &Txns) -> Vec<BalanceTreeNode> {
        // Calculate sum of postings for each account.
        //
        // Input size: is "big",    ~ all transactions
        // Output size: is "small", ~ size of CoA
        let account_sums: Vec<_> = txns
            .iter()
            .flat_map(|txn| &txn.posts)
            .sorted_by_key(|p| p.acctn.get_full())
            .group_by(|p| p.acctn.get_full())
            .into_iter()
            .map(|(_, postings)| {
                let mut ps = postings.peekable();
                // checked: unwrap: this is inside map, hence there must be at least one element
                let acctn = ps.peek().unwrap().acctn.clone();
                let acc_sum = ps.map(|p| p.amount).sum::<Decimal>();
                (acctn, acc_sum)
            })
            .collect();

        // From every account bubble up and insert missing parent AccTNs.
        //
        // This will generate duplicate forks and roots, because we are arriving
        // from different branches to the same fork in the trunk. So the set must be made
        // distinct before it can be used, so we won't duplicate sub_tree_account_sums
        //
        // Why duplicates? This is using old incomplete set of AccTNSums, not the new,
        // complete set, which will be the result of this function,
        // so the same fork in trunk will be "missing" multiple times.
        //
        //
        // Input size: is "small",  ~ size of CoA
        // Output size: is "small", ~ size of CoA
        let complete_coa_sum_tree = account_sums
            .iter()
            .flat_map(|acc| Balance::bubble_up_acctn(acc, &account_sums))
            .collect::<HashSet<_>>() // make it distinct
            .into_iter()
            .collect::<Vec<(AccountTreeNode, Decimal)>>();

        // Get all root accounts
        // Input size: is "small",  ~ size of CoA
        // Output size: is "small", ~ size of CoA
        let roots = complete_coa_sum_tree
            .iter()
            .filter(|(acctn, _)| acctn.depth == 1);

        // Start from all roots and get all subtree BalanceTreeNodes
        // Input size: is "small",  ~ size of CoA
        // Output size: is "small", ~ size of CoA
        let mut bal = roots
            .flat_map(|root_acc_sum| {
                Balance::get_balance_tree_nodes(root_acc_sum, &complete_coa_sum_tree)
            })
            .collect::<Vec<BalanceTreeNode>>();

        bal.sort_by(ord_by_btn);
        bal
    }

    pub fn from<T>(title: &str, txn_data: &TxnData, accounts: Box<T>) -> Balance
    where
        T: BalanceSelector + ?Sized,
    {
        let bal = Balance::balance(&txn_data.txns);

        let filt_bal: Vec<_> = bal
            .iter()
            .filter(|b| accounts.predicate(b))
            .cloned()
            .collect();

        if filt_bal.is_empty() {
            Balance {
                title: title.to_string(),
                bal: Default::default(),
                deltas: Default::default(),
                metadata: txn_data.metadata.clone(),
            }
        } else {
            let deltas = filt_bal
                .iter()
                .group_by(|btn| &btn.acctn.commodity_str)
                .into_iter()
                .map(|(c, bs)| {
                    let dsum = bs.map(|b| b.account_sum).sum();
                    if c.is_empty() {
                        (None, dsum)
                    } else {
                        (Some(Commodity { name: c.clone() }), dsum)
                    }
                })
                .collect();

            Balance {
                title: title.to_string(),
                bal: filt_bal,
                deltas,
                metadata: None, // todo: txnData.getMetadata(accounts))
            }
        }
    }
}
