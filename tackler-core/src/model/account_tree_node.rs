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

use crate::parser;
use std::cmp::Ordering;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Hash, PartialOrd, PartialEq, Eq)]
pub struct Commodity {
    pub name: String,
}

impl Commodity {
    pub fn from(name: String) -> Result<Commodity, Box<dyn Error>> {
        if !parser::is_valid_id(&name) {
            let msg = format!("This is not a valid commodity/currency: [{name}]");
            return Err(msg.into());
        }
        Ok(Commodity { name })
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Eq)]
pub struct AccountTreeNode {
    pub(crate) depth: usize,
    root: String,
    /// parent account (path)
    pub(crate) parent: String,
    /// account of posting (path)
    pub(crate) account: String,
    /// account name (leaf)
    name: String,
    // fixme: separate actual ATN from commodity part
    // so that pure ATN can be shared
    pub(crate) commodity: Option<Commodity>,
    pub(crate) commodity_str: String,
}

impl Display for AccountTreeNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.account)
    }
}

impl Hash for AccountTreeNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.account.hash(state);
        self.commodity.hash(state);
    }
}
impl Ord for AccountTreeNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // todo: ATN: more sensible ordering without getFull
        self.get_full().cmp(&other.get_full())
    }
}

impl PartialOrd for AccountTreeNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for AccountTreeNode {
    fn eq(&self, other: &Self) -> bool {
        self.get_full() == other.get_full()
    }
}

impl AccountTreeNode {
    pub fn from(
        account: String,
        comm: Option<Commodity>,
    ) -> Result<AccountTreeNode, Box<dyn Error>> {
        {
            let acc = account.trim();

            if acc.len() != account.len() {
                let msg = format!("account name contains whitespaces [{account}]");
                return Err(msg.into());
            }
        }

        let parts: Vec<&str> = account.split(':').collect();

        if parts.is_empty() {
            let msg = format!(
                "Empty account names are not allowed (all sub-components are empty): [{account}]"
            );
            return Err(msg.into());
        }
        if parts
            .iter()
            .map(|subpath| parser::is_valid_sub_id(subpath.trim()))
            .any(|valid| !valid)
        {
            let msg = format!("This is not valid account name: [{account}]");
            return Err(msg.into());
        }

        let depth = parts.len();
        let root = String::from(parts[0]);

        let mut rev_parts = parts;
        rev_parts.reverse();
        let name = String::from(rev_parts.remove(0));

        rev_parts.reverse();
        let parent = rev_parts.join(":");

        let commodity_str = comm
            .as_ref()
            .map(|c| String::from(&c.name))
            .unwrap_or_default();

        Ok(AccountTreeNode {
            depth,
            root,
            parent,
            account,
            name,
            commodity: comm,
            commodity_str,
        })
    }

    // todo: fn group_by (accTN)
    // accTN.getFull

    pub(crate) fn is_parent_of(&self, atn: &AccountTreeNode) -> bool {
        self.account == atn.parent && self.commodity_str == atn.commodity_str
    }

    // todo: make this static data
    // todo-perf: this is on hot path (for all Txns)
    pub fn get_full(&self) -> String {
        match &self.commodity {
            Some(c) => String::from(&c.name) + "@" + &self.account,
            None => String::from("@") + &self.account,
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn id_270d505b_76f6_4e49_a24c_2fbdfb6e5adf__commodity_ok() {
        let res = Commodity::from("He·bar".to_string());
        assert!(res.is_ok());
        let c = res.unwrap(/*:test:*/);
        assert_eq!(c.name, "He·bar".to_string());

        assert!(Commodity::from("$".to_string()).is_ok());
        assert!(Commodity::from("¢".to_string()).is_ok());
        assert!(Commodity::from("£".to_string()).is_ok());
        assert!(Commodity::from("¤".to_string()).is_ok());
        assert!(Commodity::from("¥".to_string()).is_ok());
    }

    #[test]
    fn id_699aadb1_d1ba_44b6_ae6a_158cf5be13e5__commodity_err() {
        assert!(Commodity::from("123".to_string()).is_err());
        assert!(Commodity::from("-USD".to_string()).is_err());
        assert!(Commodity::from("_USD".to_string()).is_err());
        assert!(Commodity::from("·USD".to_string()).is_err());
        assert!(Commodity::from("He:bar".to_string()).is_err());
        assert!(Commodity::from("He bar".to_string()).is_err());
    }

    #[test]
    fn id_e11d5d26_c149_4d8c_b150_5cb2e2f80608__atn_commodity() {
        let atn = AccountTreeNode::from(
            "a".to_string(),
            Some(Commodity::from("He·bar".to_string()).unwrap(/*:test:*/)),
        )
        .unwrap(/*:test:*/);

        assert!(atn.commodity.is_some());
        assert_eq!(atn.commodity.unwrap(/*:test:*/).name, "He·bar".to_string());
        assert_eq!(atn.commodity_str, String::from("He·bar"));
    }

    #[test]
    fn atn_ok_display() {
        let atn = AccountTreeNode::from(
            "a:b:c".to_string(),
            Some(Commodity::from("He·bar".to_string()).unwrap(/*:test:*/)),
        )
        .unwrap(/*:test:*/);

        let atn_str = format!("{}", atn);
        assert_eq!(atn_str, String::from("a:b:c"));
    }

    #[test]
    fn id_88c5cb23_5995_4b93_8c26_a3f7374e96d9__atn_root() {
        let atn = AccountTreeNode::from("a".to_string(), None).unwrap(/*:test:*/);

        assert_eq!(atn.depth, 1);
        assert_eq!(atn.root, "a");
        assert_eq!(atn.parent, "");
        assert_eq!(atn.account, "a");
        assert_eq!(atn.name, "a");
        assert!(atn.commodity.is_none());
        assert_eq!(atn.commodity_str, String::from(""));
    }

    #[test]
    fn id_fc69f9b2_1faf_425c_87d3_aed63d66171b__atn_two() {
        let atn = AccountTreeNode::from("a:b".to_string(), None).unwrap(/*:test:*/);

        assert_eq!(atn.depth, 2);
        assert_eq!(atn.root, "a");
        assert_eq!(atn.parent, "a");
        assert_eq!(atn.account, "a:b");
        assert_eq!(atn.name, "b");
        assert!(atn.commodity.is_none());
        assert_eq!(atn.commodity_str, String::from(""));
    }

    #[test]
    fn id_38c103d3_4cc7_4af7_86cd_bf24ca37d026__atn_three() {
        let atn = AccountTreeNode::from("a:b:c".to_string(), None).unwrap(/*:test:*/);

        assert_eq!(atn.depth, 3);
        assert_eq!(atn.root, "a");
        assert_eq!(atn.parent, "a:b");
        assert_eq!(atn.account, "a:b:c");
        assert_eq!(atn.name, "c");
        assert!(atn.commodity.is_none());
        assert_eq!(atn.commodity_str, String::from(""));
    }

    #[test]
    fn id_76a6c300_5569_4e1d_a0a1_ae2ee31d919a__atn_more() {
        let atn = AccountTreeNode::from("a:b:c:leaf".to_string(), None).unwrap(/*:test:*/);

        assert_eq!(atn.depth, 4);
        assert_eq!(atn.root, "a");
        assert_eq!(atn.parent, "a:b:c");
        assert_eq!(atn.account, "a:b:c:leaf");
        assert_eq!(atn.name, "leaf");
        assert!(atn.commodity.is_none());
        assert_eq!(atn.commodity_str, String::from(""));
    }

    #[test]
    fn id_55407835_34d8_4de9_a362_4172f0e4d54b__err_empty() {
        let atn = AccountTreeNode::from("".to_string(), None);
        assert!(atn.is_err());
    }

    #[test]
    fn id_0609e72f_c509_4b62_950e_fce432122d10__err_empty_sub() {
        // new tests
        assert!(AccountTreeNode::from("a:".to_string(), None).is_err());
        assert!(AccountTreeNode::from(":a".to_string(), None).is_err());
        assert!(AccountTreeNode::from("a::b".to_string(), None).is_err());
        assert!(AccountTreeNode::from("::".to_string(), None).is_err());
        // old tests
        assert!(AccountTreeNode::from(":".to_string(), None).is_err());
        assert!(AccountTreeNode::from(": :".to_string(), None).is_err());
    }
}
