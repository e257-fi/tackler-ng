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
use tackler_api::filters::FilterDefinition;

use super::FilterTxn;

impl FilterTxn for FilterDefinition {
    fn filter(&self, txn: &Transaction) -> bool {
        self.txn_filter.filter(txn)
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

        assert_eq!(tf_def.filter(&txn), true);
    }

    #[test]
    fn filter_false() {
        let txn = Transaction::default();

        let tf_def = FilterDefinition {
            txn_filter: TxnFilter::NullaryFALSE(NullaryFALSE {}),
        };

        assert_eq!(tf_def.filter(&txn), false);
    }
}
