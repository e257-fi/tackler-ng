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

/// Serialization and Deserialization for full haystack regex matchers
pub mod serde;

use regex::{Regex, RegexSet};

fn into_full_haystack_pattern<S>(re: S) -> String
where
    S: AsRef<str>,
{
    format!("^(?:{})$", re.as_ref())
}

fn peel_full_haystack_pattern(re: &str) -> &str {
    match re.strip_prefix("^(?:") {
        Some(prefix_clean) => prefix_clean.strip_suffix(r")$").unwrap_or(re),
        None => re,
    }
}

/// Compiles a full haystack regular expression
///
/// This will augment (anchor) the given re so that it will match against
/// full haystack.
///
/// See `Regex::Regex::new` for actual documentation of this method.
///
/// See `peeled_pattern_as_str` how to get back the original string
///
/// # Examples
/// ```rust
/// # use std::error::Error;
/// use tackler_rs::regex::new_full_haystack_regex;
///
/// let re_foo = new_full_haystack_regex("foo")?;
/// let re_bar = new_full_haystack_regex("bar")?;
///
/// assert!(re_foo.is_match("foo"));
/// assert!(re_bar.is_match("bar"));
///
/// assert!(!re_foo.is_match("foobar"));
/// assert!(!re_bar.is_match("foobar"));
/// # Ok::<(), Box<dyn Error>>(())
/// ```
pub fn new_full_haystack_regex(re: &str) -> Result<Regex, regex::Error> {
    Regex::new(into_full_haystack_pattern(re).as_str())
}

/// Returns the original string of this regex.
/// # Examples
/// ```rust
/// # use std::error::Error;
/// use tackler_rs::regex::new_full_haystack_regex;
/// use tackler_rs::regex::peeled_pattern;
///
/// let re_foo = new_full_haystack_regex(r"foo.*")?;
///
/// assert_eq!(peeled_pattern(&re_foo), r"foo.*");
/// # Ok::<(), Box<dyn Error>>(())
/// ```
pub fn peeled_pattern(regex: &Regex) -> &str {
    peel_full_haystack_pattern(regex.as_str())
}

/// Compiles a set of full haystack regular expressions
///
/// This will augment (anchor) the given expressions so
/// that each of those will match against full haystack.
///
/// See `Regex::RegexSet::new` for actual documentation of this method.
///
/// See `peeled_pattern` how to get back the original string
///
/// # Examples
/// ```rust
/// # use std::error::Error;
/// use tackler_rs::regex::new_full_haystack_regex_set;
///
/// let re_set = new_full_haystack_regex_set(["foo", "bar"])?;
///
/// assert!(re_set.is_match("foo"));
/// assert!(re_set.is_match("bar"));
///
/// assert!(!re_set.is_match("foobar"));
/// assert!(!re_set.is_match("foobar"));
/// # Ok::<(), Box<dyn Error>>(())
/// ```
pub fn new_full_haystack_regex_set<I, S>(exprs: I) -> Result<RegexSet, regex::Error>
where
    S: AsRef<str>,
    I: IntoIterator<Item = S>,
{
    RegexSet::new(exprs.into_iter().map(|re| into_full_haystack_pattern(re)))
}

/// Returns the peeled regex patterns that this regex set was constructed from.
///
/// # Examples
/// ```rust
/// # use std::error::Error;
/// use tackler_rs::regex::new_full_haystack_regex_set;
/// use tackler_rs::regex::peeled_patterns;
///
/// let re_set = new_full_haystack_regex_set(["foo", "bar"])?;
///
/// assert_eq!(peeled_patterns(&re_set), vec!["foo", "bar"]);
/// # Ok::<(), Box<dyn Error>>(())
/// ```
pub fn peeled_patterns(regex_set: &RegexSet) -> Vec<String> {
    regex_set
        .patterns()
        .iter()
        .map(|re| peel_full_haystack_pattern(re).to_string())
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peel_full_haystack_pattern() {
        assert_eq!(peel_full_haystack_pattern("abc"), "abc");
        assert_eq!(peel_full_haystack_pattern(".*"), ".*");
        assert_eq!(peel_full_haystack_pattern("(.*)"), "(.*)");
        assert_eq!(peel_full_haystack_pattern("^(?:.*)"), "^(?:.*)");
        assert_eq!(peel_full_haystack_pattern("(.*)$"), "(.*)$");
        assert_eq!(peel_full_haystack_pattern("^(?:.*)$"), ".*");
    }

    #[test]
    fn test_full_haystack_pattern() {
        let re = new_full_haystack_regex(r"o.a").unwrap(/*:test:*/);
        assert_eq!(re.as_str(), r"^(?:o.a)$");

        assert!(!re.is_match("foobar"));
        assert!(!re.is_match("ooba"));
        assert!(!re.is_match("obar"));
        assert!(re.is_match("oba"));
    }

    #[test]
    fn test_full_haystack_pattern_anchored() {
        let re = new_full_haystack_regex(r"^o.a$").unwrap(/*:test:*/);
        assert_eq!(re.as_str(), r"^(?:^o.a$)$");

        assert!(!re.is_match("foobar"));
        assert!(!re.is_match("ooba"));
        assert!(!re.is_match("obar"));
        assert!(re.is_match("oba"));
    }

    #[test]
    fn test_full_haystack_pattern_peeled() {
        let re_str = r"^(?:o.a)$";
        let re = new_full_haystack_regex(re_str).unwrap(/*:test:*/);
        assert_eq!(re.as_str(), r"^(?:^(?:o.a)$)$");

        assert!(!re.is_match("foobar"));
        assert!(!re.is_match("ooba"));
        assert!(!re.is_match("obar"));
        assert!(re.is_match("oba"));

        assert_eq!(peeled_pattern(&re), re_str);
    }

    #[test]
    fn test_full_haystack_patterns() {
        let re_set = new_full_haystack_regex_set([r".*foo", r"bar.*"]).unwrap(/*:test:*/);
        assert_eq!(re_set.patterns(), [r"^(?:.*foo)$", r"^(?:bar.*)$"]);

        assert!(!re_set.is_match("foobar"));
        assert!(re_set.is_match("foo"));
        assert!(re_set.is_match("bar"));
    }

    #[test]
    fn test_full_haystack_patterns_anchored() {
        let re_set = new_full_haystack_regex_set([r"^.*foo$", r"^bar.*$"]).unwrap(/*:test:*/);
        assert_eq!(re_set.patterns(), [r"^(?:^.*foo$)$", r"^(?:^bar.*$)$"]);

        assert!(!re_set.is_match("foobar"));
        assert!(re_set.is_match("foo"));
        assert!(re_set.is_match("bar"));
    }

    #[test]
    fn test_full_haystack_patterns_peeled() {
        let re_set_str = [r"^(?:.*foo)$", r"^(?:bar.*)$"];
        let re_set = new_full_haystack_regex_set(re_set_str).unwrap(/*:test:*/);
        assert_eq!(
            re_set.patterns(),
            [r"^(?:^(?:.*foo)$)$", r"^(?:^(?:bar.*)$)$"]
        );

        assert!(!re_set.is_match("foobar"));
        assert!(re_set.is_match("foo"));
        assert!(re_set.is_match("bar"));

        assert_eq!(peeled_patterns(&re_set), re_set_str);
    }
}
