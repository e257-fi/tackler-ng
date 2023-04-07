/*
 * Copyright 2022-2023 E257.FI
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

//! Rusty services for Tackler
//!
//! This crate is a collection of utilities for Tackler.
//!
#![deny(missing_docs)]
#![forbid(unsafe_code)]

use std::error::Error;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

///
/// Get a list of paths by base dir and file extension
///
pub fn get_paths_by_ext(base_dir: &Path, extension: &str) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    fn is_txn_file(entry: &walkdir::DirEntry, extension: &str) -> bool {
        (entry.file_type().is_file() || entry.file_type().is_symlink())
            && match entry.path().extension() {
                Some(ext) => ext == extension,
                None => false,
            }
    }
    let dir_entries: Result<Vec<DirEntry>, _> = WalkDir::new(base_dir)
        .follow_links(true)
        .into_iter()
        .collect();

    let paths: Vec<PathBuf> = dir_entries?
        .iter()
        .filter(|e| is_txn_file(e, extension))
        .map(|x| x.path().to_owned())
        .collect();

    Ok(paths)
}

/// Extensions to be used with [Indoc](https://docs.rs/indoc/latest/indoc/)
pub trait IndocUtils {
    #[allow(clippy::needless_doctest_main)]
    /// Strip away `|` -- prefix marker
    ///
    /// For full documentation, see  [`indoc!` -- docs](https://docs.rs/indoc/latest/indoc/).
    ///
    /// ```
    /// fn main() {
    ///     use indoc::indoc;
    ///     use tackler_rs::IndocUtils;
    ///     let testing = indoc! {
    ///         "|def hello():
    ///          |    print('Hello, bar!')
    ///          |
    ///          |hello()
    ///          |"
    ///     }.strip_margin();
    ///     let expected = "def hello():\n    print('Hello, bar!')\n\nhello()\n";
    ///     assert_eq!(testing, expected);
    ///
    ///     let second = indoc! {
    ///          "def hello():
    ///          |    print('Hello, bar!')
    ///          |
    ///          |hello()
    ///          |"
    ///     }.strip_margin();
    ///     assert_eq!(second, testing);
    /// }
    /// ```
    fn strip_margin(&self) -> String;
}

impl IndocUtils for str {
    fn strip_margin(&self) -> String {
        match self.strip_prefix('|') {
            Some(s) => s.to_string().replace("\n|", "\n"),
            None => self.replace("\n|", "\n"),
        }
    }
}
