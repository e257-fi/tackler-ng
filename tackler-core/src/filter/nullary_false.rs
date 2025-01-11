/*
 * Tackler-NG 2023
 *
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::model::Transaction;
use tackler_api::filters::NullaryFALSE;

use crate::kernel::Predicate;

impl Predicate<Transaction> for NullaryFALSE {
    fn eval(&self, _txn: &Transaction) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tackler_api::filters::TxnFilter;

    #[test]
    fn to_be_or_not_to_be() {
        let txn = Transaction::default();

        let tf = TxnFilter::NullaryFALSE(NullaryFALSE {});

        assert!(!tf.eval(&txn));
    }
}
