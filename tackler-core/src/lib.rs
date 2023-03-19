/*
 * Copyright 2022 E257.FI
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
#![feature(hash_raw_entry)]

pub mod filter;
pub mod kernel;
pub mod math;
pub mod model;
pub mod parser;
pub mod report;

#[cfg(test)]
mod tests {
    pub(crate) trait IndocWithMarker {
        fn strip_margin(&self) -> String;
    }

    impl IndocWithMarker for str {
        fn strip_margin(&self) -> String {
            match self.strip_prefix('|') {
                Some(s) => s.to_string().replace("\n|", "\n"),
                None => self.replace("\n|", "\n"),
            }
        }
    }
}
