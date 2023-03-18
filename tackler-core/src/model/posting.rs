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

use crate::model::Commodity;
use crate::model::{AccountTreeNode, Posts};
use rust_decimal::Decimal;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Posting {
    pub acctn: AccountTreeNode,
    pub amount: Decimal,
    // todo: fix / rename these (position?, exchange? amount, commodity)
    pub txn_amount: Decimal,
    pub is_total_amount: bool,
    pub txn_commodity: Option<Commodity>,
    pub comment: Option<String>,

    // fixme: remove duplicate information, this is kind of available via ATN
    // See fixme in ATN
    pub atn_key: String,
}

impl Posting {
    pub(crate) fn from(
        acctn: AccountTreeNode,
        amount: Decimal,
        txn_amount: Decimal,
        is_total_amount: bool,
        txn_commodity: Option<Commodity>,
        comment: Option<String>,
    ) -> Result<Posting, Box<dyn Error>> {
        if amount.is_zero() {
            let msg = format!("Zero sum postings are not allowed: {}", acctn.account);
            return Err(msg.into());
        }

        let atn_key = acctn.get_full();
        Ok(Posting {
            acctn,
            amount,
            txn_amount,
            is_total_amount,
            txn_commodity,
            comment,
            atn_key,
        })
    }
}
pub fn txn_sum(posts: &Posts) -> Decimal {
    posts.iter().map(|p| p.txn_amount).sum()
}

impl Display for Posting {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let sign_space = if self.amount.is_sign_negative() {
            ""
        } else {
            " "
        };

        write!(
            f,
            "{}  {}{}{}{}{}",
            self.acctn,
            sign_space,
            self.amount,
            self.acctn
                .commodity
                .as_ref()
                .map(|c| format!(" {}", c.name))
                .unwrap_or_default(),
            self.txn_commodity
                .as_ref()
                .map(|txn_c| {
                    #[allow(clippy::collapsible_else_if)]
                    // todo: old-scala comment: fix this
                    if txn_c.name == self.acctn.commodity_str {
                        String::default()
                    } else {
                        if self.is_total_amount {
                            format!(" = {} {}", self.txn_amount, txn_c.name)
                        } else {
                            format!(" @ {} {}", (self.txn_amount / self.amount), txn_c.name)
                        }
                    }
                })
                .unwrap_or_default(),
            self.comment
                .as_ref()
                .map(|c| format!(" ; {c}"))
                .unwrap_or_default()
        )
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    // desc: "reject zero postings"
    fn id_42ad9d32_64aa_4fcd_a4ab_1e8521b921e3__reject_zero_posting() {
        {
            let acctn = AccountTreeNode::from("a:b".to_string(), None).unwrap();
            let p = Posting::from(
                acctn,
                Decimal::new(0, 0),
                Decimal::new(0, 0),
                false,
                None,
                None,
            );
            assert!(p.is_err());
        }
        {
            // check that difference precision doesn't mess Decimal comparisons
            let acctn = AccountTreeNode::from("a:b".to_string(), None).unwrap();
            let p = Posting::from(
                acctn,
                Decimal::new(0, 28),
                Decimal::new(0, 28),
                false,
                None,
                None,
            );
            assert!(p.is_err());
        }
    }

    #[test]
    // desc: "preserve precision - 1E20"
    fn id_e3c97b66_318c_4396_8857_0cd2c1dfb0d2__preserve_precision_1E20() {
        /*
         * val v = //          3         2         1         .         1         2         3         4
         *        TacklerReal("123456789012345678901234567890.123456789012345678901234567890123456789012")
         * val p = Posting(acctn, v, v, false, None, None)
         * assert(p.toString === "a:b   123456789012345678901234567890.123456789012345678901234567890123456789012")
         */
        let v_str =
            //2         1         .         1         2         3         4
             "12345678901234567890.123456789";
        let ref_str = format!("a:b   {}", v_str);
        let v = Decimal::from_str_exact(v_str).unwrap();
        let acctn = AccountTreeNode::from("a:b".to_string(), None).unwrap();
        let p = Posting::from(acctn, v, v, false, None, None).unwrap();

        let p_str = format!("{}", p);
        assert_eq!(p_str, ref_str);
        assert_eq!(p.to_string(), ref_str);
    }

    #[test]
    // desc: "preserve precision - 1E15"
    fn id_26da0769_de5f_4344_b1d4_d3ddbf3f7f5a__preserve_precision_1E15() {
        /*
         * val v = //          3         2         1         .         1         2         3         4
         *        TacklerReal("123456789012345678901234567890.123456789012345678901234567890123456789012")
         * val p = Posting(acctn, v, v, false, None, None)
         * assert(p.toString === "a:b   123456789012345678901234567890.123456789012345678901234567890123456789012")
         */
        let v_str =
            // Quadrillion is 15 digits, e.g. 100 * USA budget
            //2         1         .         1         2         3         4
                  "678901234567890.12345678901234";
        let ref_str = format!("a:b   {}", v_str);
        let v = Decimal::from_str_exact(v_str).unwrap();
        let acctn = AccountTreeNode::from("a:b".to_string(), None).unwrap();
        let p = Posting::from(acctn, v, v, false, None, None).unwrap();

        let p_str = format!("{}", p);
        assert_eq!(p_str, ref_str);
        assert_eq!(p.to_string(), ref_str);
    }

    #[test]
    // desc: "toString e.g. Display"
    fn id_6ce68af4_5349_44e0_8fbc_35bebd8ac1ac__display() {
        let v = Decimal::new(12301, 2);
        let acctn = AccountTreeNode::from("a:b".to_string(), None).unwrap();
        let p = Posting::from(acctn, v, v, false, None, Some("comment".to_string())).unwrap();

        let p_str = format!("{}", p);
        assert_eq!(p_str, "a:b   123.01 ; comment");
    }

    #[test]
    // desc: "unit price"
    fn id_16b54e8c_5ea6_420c_bd72_157dbcc06a49__unit_price() {
        let pv = Decimal::new(12300, 2);
        let tv = Decimal::new(24600, 2);
        let acctn = AccountTreeNode::from("a:b".to_string(), None).unwrap();
        let p = Posting::from(
            acctn,
            pv,
            tv,
            false,
            Some(Commodity {
                name: "€".to_string(),
            }),
            None,
        )
        .unwrap();

        assert_eq!(p.to_string(), "a:b   123.00 @ 2 €");
    }

    #[test]
    // desc: "unit price with comment"
    fn id_22059d1d_7c10_42dc_831f_03bd1f1d6257__unit_price_w_comment() {
        let pv = Decimal::new(12300, 2);
        let tv = Decimal::new(24600, 2);
        let acctn = AccountTreeNode::from("a:b".to_string(), None).unwrap();
        let p = Posting::from(
            acctn,
            pv,
            tv,
            false,
            Some(Commodity {
                name: "€".to_string(),
            }),
            Some("comment".to_string()),
        )
        .unwrap();

        assert_eq!(p.to_string(), "a:b   123.00 @ 2 € ; comment");
    }

    #[test]
    // desc: "total price"
    fn id_0fef204a_19da_418f_b7d0_86b5211c2182__total_price() {
        let pv = Decimal::new(12300, 2);
        let tv = Decimal::new(24600, 2);
        let acctn = AccountTreeNode::from("a:b".to_string(), None).unwrap();
        let p = Posting::from(
            acctn,
            pv,
            tv,
            true,
            Some(Commodity {
                name: "€".to_string(),
            }),
            None,
        )
        .unwrap();

        assert_eq!(p.to_string(), "a:b   123.00 = 246.00 €");
    }

    #[test]
    // desc: "total price with comment"
    fn id_718dd25c_aebc_4f29_9903_67942c6ba531__total_price_w_comment() {
        let pv = Decimal::new(12300, 2);
        let tv = Decimal::new(24600, 2);
        let acctn = AccountTreeNode::from("a:b".to_string(), None).unwrap();
        let p = Posting::from(
            acctn,
            pv,
            tv,
            true,
            Some(Commodity {
                name: "€".to_string(),
            }),
            Some("comment".to_string()),
        )
        .unwrap();

        assert_eq!(p.to_string(), "a:b   123.00 = 246.00 € ; comment");
    }
}
