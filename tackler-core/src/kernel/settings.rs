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

use std::error::Error;
use std::ops::Index;
use std::path::{Path, PathBuf};

struct Auditing {}

struct CfgKeys {}

impl CfgKeys {
    const TIMEZONE: &'static str = "timezone";
    const BASEDIR: &'static str = "tackler.kernel.basedir";
}

#[derive(Debug)]
pub struct Settings {
    pub basedir: Box<Path>,
    pub accounts: Vec<String>,
}

impl Settings {
    /// TODO
    pub fn from(cfg_file: &str) -> Result<Settings, Box<dyn Error>> {
        Ok(Settings {
            basedir: PathBuf::from("foo").into_boxed_path(),
            accounts: vec![],
        })
    }
}
