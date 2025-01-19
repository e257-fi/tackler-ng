/*
 * Tackler-NG 2025
 * SPDX-License-Identifier: Apache-2.0
 */
use std::error::Error;
use std::fs;
use std::path::Path;

mod accounts_toml;
mod commodities_toml;
mod journal_txn;
mod tackler_toml;
mod tags_toml;
mod welcome_txn;

pub(crate) fn exec(name: &str) -> Result<(), Box<dyn Error>> {
    let conf_dir = Path::new(name).join("conf");
    let txns_dir = Path::new(name).join("txns");

    if fs::exists(&conf_dir)? {
        let p = Path::new(&conf_dir).canonicalize()?;
        return Err(format!("'conf' destination `{}` already exists", &p.display()).into());
    }
    if fs::exists(&txns_dir)? {
        let p = Path::new(&txns_dir).canonicalize()?;
        return Err(format!("'txns' destination `{}` already exists", &p.display()).into());
    }
    fs::create_dir_all(&conf_dir)?;
    fs::create_dir_all(&txns_dir)?;

    let conf_files = [
        ("tackler.toml", tackler_toml::TXT),
        ("accounts.toml", accounts_toml::TXT),
        ("commodities.toml", commodities_toml::TXT),
        ("tags.toml", tags_toml::TXT),
    ];
    for (file_name, content) in &conf_files {
        let file_path = conf_dir.join(file_name);
        fs::write(file_path, content)?;
    }

    let txns_files = [("journal.txn", journal_txn::TXT)];
    for (file_name, content) in &txns_files {
        let file_path = txns_dir.join(file_name);
        fs::write(file_path, content)?;
    }
    let file_path = txns_dir.join("welcome.txn");
    fs::write(file_path, welcome_txn::get_txt(name))?;

    Ok(())
}
