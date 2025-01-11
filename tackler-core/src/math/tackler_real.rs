/*
 * Tackler-NG 2019-2023
 *
 * SPDX-License-Identifier: Apache-2.0
 */

use rust_decimal::Decimal;

pub fn from_str(num: &str) -> Result<Decimal, rust_decimal::Error> {
    Decimal::from_str_exact(num)
}
