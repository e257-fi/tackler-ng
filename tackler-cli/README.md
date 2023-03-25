# Tackler-NG

This will be rusty [Tackler](https://tackler.e257.fi/)

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
