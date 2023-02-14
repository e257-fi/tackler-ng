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
    let dir_entries: Result<Vec<DirEntry>, _> = WalkDir::new(base_dir).follow_links(true)
        .into_iter()
        .collect();

    let paths: Vec<PathBuf> = dir_entries?.iter()
        .filter(|e| is_txn_file(e, extension))
        .map(|x| x.path().to_owned())
        .collect();

    Ok(paths)
}
