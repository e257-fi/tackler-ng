/*
 * Tackler-NG 2025
 * SPDX-License-Identifier: Apache-2.0
 */

pub(crate) fn get_txt(name: &str) -> String {
    format!(
        r"
2025-01-01 'Welcome to Tackler-NG!
    ; See Tackler Documentation for more information:
    ; * Documentation:  https://tackler.e257.fi/docs/
    ; * Journal Format: https://tackler.e257.fi/docs/journal/format/
    ;
    ; This setup has following structure:
    ; {}
    ; ├── conf
    ; │   ├── tackler.toml
    ; │   ├── accounts.toml
    ; │   ├── commodities.toml
    ; │   └── tags.toml
    ; └── txns
    ;     ├── welcome.txn
    ;     └── journal.txn
    ;
    ; Happy accounting!
    Welcome:Message 1
    Messages
",
        name
    )
}
