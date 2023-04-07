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
