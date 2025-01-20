/*
 * Tackler-NG 2025
 * SPDX-License-Identifier: Apache-2.0
 */
use crate::commands::init;
use std::error::Error;
use std::fs;
use std::path::Path;

pub(crate) fn exec(name: &str) -> Result<(), Box<dyn Error>> {
    if fs::exists(name)? {
        let p = Path::new(name).canonicalize()?;
        return Err(format!("destination `{}` already exists", &p.display()).into());
    }
    fs::create_dir_all(name)?;

    init::exec(name)?;

    Ok(())
}
