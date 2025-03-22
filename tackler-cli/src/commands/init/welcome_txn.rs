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
    ; {name}
    ; ├── conf
    ; │   ├── tackler.toml
    ; │   ├── accounts.toml
    ; │   ├── commodities.toml
    ; │   └── tags.toml
    ; └── txns
    ;     ├── price.db
    ;     ├── welcome.txn
    ;     └── journal.txn
    ;
    Welcome:Message 1
    Messages

2025-03-21 'Tackler has support for commodity prices!
    ;
    ;  tackler \
    ;      --config {name}/conf/tackler.toml \
    ;      --reports balance \
    ;      --price.lookup-type last-price
    ;
    ; Happy accounting!
    Welcome:Message 1
    Messages

"
    )
}
