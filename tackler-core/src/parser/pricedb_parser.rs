/*
 * Tackler-NG 2025
 * SPDX-License-Identifier: Apache-2.0
 */

use itertools::Itertools;
use winnow::{
    Parser,
    combinator::{eof, opt, preceded, repeat_till},
};

use crate::kernel::Settings;
use crate::model::price_entry::PriceDb;
use crate::parser::Stream;

use super::parts::{pricedb::parse_price_entry, txns::multispace0_line_ending};

use std::error::Error;
use std::path::Path;

pub fn pricedb_from_str(
    input: &mut &str,
    settings: &mut Settings,
) -> Result<PriceDb, Box<dyn Error>> {
    let is = Stream {
        input,
        state: settings,
    };

    preceded(
        opt(multispace0_line_ending),
        repeat_till(1.., parse_price_entry, eof),
    )
    .parse(is)
    .map(|(price_entries, _): (Vec<_>, _)| price_entries.into_iter().sorted().dedup().collect())
    .map_err(|err| err.to_string().into())
}

pub fn pricedb_from_file(path: &Path, settings: &mut Settings) -> Result<PriceDb, Box<dyn Error>> {
    let pricedb_str = std::fs::read_to_string(path)
        .map_err(|err| format!("Can't open file: '{}' - {}", path.display(), err))?;

    // todo: error log
    pricedb_from_str(&mut &*pricedb_str, settings)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::Settings;

    #[test]
    fn test_parse_pricedb() {
        let test = r#"
P 2024-01-09 XAU 2659.645203 USD
P 2024-01-09 USD  121.306155 BDT
P 2024-01-09 XAG  3652.77663 BDT
"#;

        let mut settings = Settings::default();

        let res = pricedb_from_str(&mut &*test, &mut settings);

        assert!(res.is_ok());
    }
}
