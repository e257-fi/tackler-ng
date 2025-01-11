/*
 * Tackler-NG 2023
 *
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::model::Transaction;
use tackler_api::filters::FilterDefinition;

use crate::kernel::Predicate;

impl Predicate<Transaction> for FilterDefinition {
    fn eval(&self, txn: &Transaction) -> bool {
        self.txn_filter.eval(txn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tackler_api::filters::{NullaryFALSE, NullaryTRUE, TxnFilter};

    #[test]
    fn filter_true() {
        let txn = Transaction::default();

        let tf_def = FilterDefinition {
            txn_filter: TxnFilter::NullaryTRUE(NullaryTRUE {}),
        };

        assert!(tf_def.eval(&txn));
    }

    #[test]
    fn filter_false() {
        let txn = Transaction::default();

        let tf_def = FilterDefinition {
            txn_filter: TxnFilter::NullaryFALSE(NullaryFALSE {}),
        };

        assert!(!tf_def.eval(&txn));
    }
}
