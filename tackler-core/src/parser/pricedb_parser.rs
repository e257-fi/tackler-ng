/*
 * Tackler-NG 2025
 *
 * SPDX-License-Identifier: Apache-2.0
 */

use winnow::{
    combinator::{eof, opt, preceded, repeat_till},
    Parser,
};

use crate::kernel::Settings;
use crate::model::price_entry::PriceEntry;
use crate::parser::Stream;

use super::parts::{pricedb::parse_price_entry, txns::multispace0_line_ending};

use std::error::Error;
use std::path::Path;

pub(crate) fn pricedb_from_str(
    input: &mut &str,
    settings: &mut Settings,
) -> Result<Vec<PriceEntry>, Box<dyn Error>> {
    let is = Stream {
        input,
        state: settings,
    };

    preceded(
        opt(multispace0_line_ending),
        repeat_till(1.., parse_price_entry, eof),
    )
    .parse(is)
    .map(|(price_entries, _)| price_entries)
    .map_err(|err| err.to_string().into())
    // .map_err(|err| {
    //     let mut msg = "Failed to process txn input\n".to_string();
    //     //let _ = writeln!(msg, "Error: {}", err);
    //     match err.into_inner() {
    //         Some(ce) => {
    //             if let Some(cause) = ce.cause() {
    //                 let _ = writeln!(msg, "Cause:\n{}\n", cause);
    //             }
    //             let _ = writeln!(msg, "Error backtrace:");
    //             for c in ce.context() {
    //                 let _ = writeln!(msg, "   {}", c);
    //             }
    //         }
    //         None => {
    //             let _ = write!(msg, "No detailed error information available");
    //         }
    //     }
    //     let i = is.input.lines().next().unwrap_or(is.input);
    //     let i_err = if i.chars().count() < 1024 {
    //         i.to_string()
    //     } else {
    //         i.chars().take(1024).collect::<String>()
    //     };

    //     let _ = write!(msg, "Failed input:\n{}\n\n", i_err);

    //     msg.into()
    // })
}

pub(crate) fn pricedb_from_file(
    path: &Path,
    settings: &mut Settings,
) -> Result<Vec<PriceEntry>, Box<dyn Error>> {
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
