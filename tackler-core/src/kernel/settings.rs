/*
 * Copyright 2023 E257.FI
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

use crate::kernel::hash::Hash;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Default)]
pub struct Audit {
    pub hash: Option<Hash>,
}

#[derive(Debug)]
pub struct Settings {
    pub basedir: Box<Path>,
    pub accounts: Option<Vec<String>>,
    pub audit: Audit,
}

impl Settings {
    pub fn default_audit() -> Self {
        Settings {
            basedir: PathBuf::default().into_boxed_path(),
            accounts: None,
            audit: Audit {
                hash: Some(Hash::default()),
            },
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            basedir: PathBuf::default().into_boxed_path(),
            accounts: None,
            audit: Audit { hash: None },
        }
    }
}
