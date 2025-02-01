/*
 * Tackler-NG 2023-2025
 * SPDX-License-Identifier: Apache-2.0
 */

///
/// Predicate to test if item x is part of set or not
///
/// `x` item to be tested
/// `returns` true if it's selected, false if it's rejected
pub use settings::Settings;
pub(crate) mod accumulator;
pub mod balance;
pub mod hash;
pub mod price_lookup;
pub mod report_item_selector;
pub mod settings;

pub trait Predicate<T> {
    fn eval(&self, item: &T) -> bool;
}
