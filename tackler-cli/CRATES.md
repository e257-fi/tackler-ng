# Tackler-NG

This is rusty [Tackler](https://tackler.e257.fi/)

cargo run -- \
    --help


cargo run  -- \
    --input.git.repo ../suite/audit/audit-repo.git \
    --input.git.dir txns \
    --input.git.ref txns-1E1 \
    --reports balance

cargo run  -- \
    --input.git.repo ../suite/audit/audit-repo.git \
    --input.git.dir txns \
    --input.git.ref txns-1E1 \
    --reports balance \
    --audit.mode true

cargo run  -- \
    --input.fs.dir ../suite/core/ok/txns-id2id/ \
    --input.fs.ext txn \
    --reports register

cargo run  -- \
    --input.file ../suite/reporting/ok/reporting.txn \
    --reports balance register


## Tackler components at Crates.io

* Tackler CLI [application](https://crates.io/crates/tackler)
* Tackler [Client API](https://crates.io/crates/tackler-api)
* Tackler [Server API](https://crates.io/crates/tackler-core)
* Tackler [Rusty Services](https://crates.io/crates/tackler-rs)

