use crate::model::Transaction;
use tackler_api::filters::PropFilter;

use super::FilterTxn;

impl FilterTxn for PropFilter {
    fn filter(&self, txn: &Transaction) -> bool {
        let empty = String::default();
        txn.header
            .description
            .as_ref()
            .unwrap_or(&empty)
            .contains(&self.regex)
    }
}
