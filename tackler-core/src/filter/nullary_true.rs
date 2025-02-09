/*
 * Tackler-NG 2023
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::model::Transaction;
use tackler_api::filters::NullaryTRUE;

use crate::kernel::Predicate;

impl Predicate<Transaction> for NullaryTRUE {
    fn eval(&self, _txn: &Transaction) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tackler_api::filters::TxnFilter;

    #[test]
    fn to_be_or_not_to_be() {
        let txn = Transaction::default();

        let tf = TxnFilter::NullaryTRUE(NullaryTRUE {});

        assert!(tf.eval(&txn));
    }
}
