/*
 * This file is licensed under either of
 *  - Apache License, Version 2.0
 * OR
 *  - MIT license
 * at your option.
 *
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 *
 **************************************************************************
 *
 * Apache License header
 *
 * Copyright 2024 E257.FI
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
 ***************************************************************************
 *
 * MIT License
 *
 * Copyright 2024 E257.FI
 *
 * Permission is hereby granted, free of charge, to any person obtaining
 * a copy of this software and associated documentation files (the “Software”),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom
 * the Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included
 * in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
 * INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR
 * PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE
 * FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
 * TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE
 * OR OTHER DEALINGS IN THE SOFTWARE.
 */

//
// This code is based on: https://github.com/tailhook/serde-regex,
// which is licensed as Apache-2.0 OR MIT
//

use regex::Regex;
use std::{
    borrow::Cow,
    hash::Hash,
    ops::{Deref, DerefMut},
};

use crate::regex::{new_full_haystack_regex, peeled_pattern};
use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

/// A wrapper type which implements `Serialize` and `Deserialize` for
/// types involving `Regex`
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Serde<T>(pub T);

impl<'de> Deserialize<'de> for Serde<Regex> {
    fn deserialize<D>(d: D) -> Result<Serde<Regex>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <Cow<'_, str>>::deserialize(d)?;

        match new_full_haystack_regex(s.as_ref()) {
            Ok(regex) => Ok(Serde(regex)),
            Err(err) => Err(D::Error::custom(err)),
        }
    }
}

/// Deserialize function, see crate docs to see how to use it
pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    Serde<T>: Deserialize<'de>,
{
    Serde::deserialize(deserializer).map(|x| x.0)
}

/// Serialize function, see crate docs to see how to use it
pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    for<'a> Serde<&'a T>: Serialize,
{
    Serde(value).serialize(serializer)
}

impl<T> Deref for Serde<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for Serde<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> Serde<T> {
    /// Consumes the `Serde`, returning the inner value.
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> From<T> for Serde<T> {
    fn from(val: T) -> Serde<T> {
        Serde(val)
    }
}

impl Serialize for Serde<&Regex> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        peeled_pattern(self.0).serialize(serializer)
    }
}

impl Serialize for Serde<Regex> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        peeled_pattern(&self.0).serialize(serializer)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::regex::into_full_haystack_pattern;
    use regex::Regex;
    use serde_json::{from_str, to_string};

    const SAMPLE: &str = r#"[a-z"\]]+\d{1,10}""#;
    const SAMPLE_JSON: &str = r#""[a-z\"\\]]+\\d{1,10}\"""#;

    #[test]
    fn test_regex() {
        let re: Serde<Regex> = from_str(SAMPLE_JSON).unwrap();

        assert_eq!(re.as_str(), into_full_haystack_pattern(SAMPLE));
        assert_eq!(to_string(&re).unwrap(), SAMPLE_JSON);
    }
}
