/*
 * Copyright 2023-2024 E257.FI
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

///
/// Predicate to test if item x is part of set or not
///
/// `x` item to be tested
/// `returns` true if it's selected, false if it's rejected
pub use settings::Settings;
pub(crate) mod accumulator;
pub mod balance;
pub mod hash;
pub mod report_item_selector;
pub mod settings;

pub trait Predicate<T> {
    fn eval(&self, item: &T) -> bool;
}
