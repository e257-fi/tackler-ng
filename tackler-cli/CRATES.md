# Tackler-NG: Tackler CLI Application

[![Chat on Matrix](https://tackler.e257.fi/img/badge-matrix.svg)](https://matrix.to/#/#tackler:matrix.org)

This is rusty version of [Tackler](https://tackler.e257.fi/) CLI application.

Tackler is an accounting engine and reporting tool 
for text based double-entry accounting.

## Project Status

The project is in Technology Preview Release phase.

The [Tackler Journal Format](https://tackler.e257.fi/docs/journal/format/) is fully 
supported, as are all transaction backends ([Filesystem](https://tackler.e257.fi/docs/usage/#storage-selector)
and [Git Storage](https://tackler.e257.fi/docs/journal/git-storage/)). See `tackler --help` how to use these.

All reports and exports are supported:
* Reports
  * [Balance](https://tackler.e257.fi/docs/report-balance/)
  * [Balance Group](https://tackler.e257.fi/docs/report-balance-group/)
  * [Register](https://tackler.e257.fi/docs/report-register/)
* Exports
  * [Equity](https://tackler.e257.fi/docs/export-equity/)
  * [Identity](https://tackler.e257.fi/docs/export-equity/)

Other supported notable features are:
* Handling [Commodities](https://tackler.e257.fi/docs/commodities/) and [Shares](https://tackler.e257.fi/docs/currencies/)
* [Transaction Filters](https://tackler.e257.fi/docs/txn-filters/)
* [Transacation Geo Location](https://tackler.e257.fi/docs/gis/txn-geo-location/) and [Transaction Geo Filters](https://tackler.e257.fi/docs/gis/txn-geo-filters/)


**AS THIS IS TECHNOLOGY PREVIEW RELEASE, THERE ARE MISSING FEATURES
AND KNOWN INCONSISTENCIES WITH EXISTING TACKLER IMPLEMENTATION.**

Major missing features are lack of configuration support
and missing support for Chart of Accounts.


## Build and install tackler

You need Rust to build tackler

````bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
git clone --recurse-submodules https://github.com/e257-fi/tackler-ng
````

````bash
cd tackler-ng
# Check the latest relase version
git tag -l
# Get the release, e.g. v24.11.0
git checkout v24.11.0

# Build and install tacker
cargo install tackler
# check that it works
tackler --version
tackler --help
````

## Simple demo

### Create a playground

````bash
mkdir -p tackler/txns; cd tackler
````

### Let's record some transaction data

````bash
cat > txns/journal.txn << EOF
2023-04-01 'It was warm, sunny day
  Expenses:Ice_cream  2
  Assets:Cash

2023-05-01 'Ice cream 'n soda!
 Expenses:BostonCooler 3
 Assets:Cash
 
EOF
````

### Create some reports

#### Simple balance
````bash
tackler --input.file txns/journal.txn --reports balance
````

#### Output

````
BALANCE
-------
                 0.00   -5.00  Assets
                -5.00   -5.00  Assets:Cash
                 0.00    5.00  Expenses
                 3.00    3.00  Expenses:BostonCooler
                 2.00    2.00  Expenses:Ice_cream
=====================
                 0.00
````

#### Balance with account filters

````bash
tackler --input.file txns/journal.txn --reports balance --accounts '^Expenses'
````

#### Output
````
BALANCE
-------
                 0.00   5.00  Expenses
                 3.00   3.00  Expenses:BostonCooler
                 2.00   2.00  Expenses:Ice_cream
=====================
                 5.00
````



## Let's play for real

### Get test vectors and full source code of Tackler

````bash
git clone --recurse-submodules https://github.com/e257-fi/tackler-ng
````

### Use Git repository as Txn storage

#### Reports with Txn Checksum

````bash
tackler \
    --input.git.repo tackler-ng/suite/audit/audit-repo.git \
    --input.git.dir txns \
    --input.git.ref txns-1E1 \
    --reports balance \
    --accounts '^a:.*' \
    --audit.mode true
````

#### Output

````
Git Storage
         commit : 4aa4e9797501c1aefc92f32dff30ab462dae5545
      reference : txns-1E1
      directory : txns
         suffix : txn
        message : txns-1E1: 2016/12

Txn Set Checksum
        SHA-256 : 9b29071e1bf228cfbd31ca2b8e7263212e4b86e51cfee1e8002c9b795ab03f76
       Set size : 10

BALANCE
-------
                 0.00   -161.00  a:ay2016
                -6.00     -6.00  a:ay2016:am02
               -14.00    -14.00  a:ay2016:am03
               -19.00    -19.00  a:ay2016:am04
               -26.00    -26.00  a:ay2016:am05
                -1.00     -1.00  a:ay2016:am07
                -7.00     -7.00  a:ay2016:am08
               -13.00    -13.00  a:ay2016:am09
               -19.00    -19.00  a:ay2016:am10
               -25.00    -25.00  a:ay2016:am11
               -31.00    -31.00  a:ay2016:am12
=====================
              -161.00
````

#### Report with 100_000 Transactions

````bash
tackler \
    --input.git.repo tackler-ng/suite/audit/audit-repo.git \
    --input.git.dir txns \
    --input.git.ref txns-1E5 \
    --reports balance \
    --accounts '^a:.*' \
    --audit.mode true
````

#### Output

````
Git Storage
         commit : cb56fdcdd2b56d41fc08cc5af4a3b410896f03b5
      reference : txns-1E5
      directory : txns
         suffix : txn
        message : txns-1E5: 2016/12

Txn Set Checksum
        SHA-256 : 27060dc1ebde35bebd8f7af2fd9815bc9949558d3e3c85919813cd80748c99a7
       Set size : 100000

BALANCE
-------
                     0.00   -1574609.01  a:ay2016
               -135600.00    -135600.00  a:ay2016:am01
               -118950.00    -118950.00  a:ay2016:am02
               -135631.00    -135631.00  a:ay2016:am03
               -127137.00    -127137.00  a:ay2016:am04
               -135616.00    -135616.00  a:ay2016:am05
               -127154.00    -127154.00  a:ay2016:am06
               -135600.00    -135600.00  a:ay2016:am07
               -135603.00    -135603.00  a:ay2016:am08
               -127140.00    -127140.00  a:ay2016:am09
               -135619.00    -135619.00  a:ay2016:am10
               -127126.00    -127126.00  a:ay2016:am11
               -133433.00    -133433.00  a:ay2016:am12
=========================
              -1574609.01
````

### Transaction Filters

#### Filter definition

````bash
tackler \
    --input.git.repo tackler-ng/suite/audit/audit-repo.git \
    --input.git.dir txns \
    --input.git.ref txns-1E5 \
    --reports balance \
    --accounts '^a:.*' \
    --audit.mode true \
    --api-filter-def '{"txnFilter":{"TxnFilterPostingAccount":{"regex":"^a:ay2016:am12"}}}'
````

The transaction filter definition could be given also as Base64 ascii armored string:

````
--api-filter-def base64:eyJ0eG5GaWx0ZXIiOnsiVHhuRmlsdGVyUG9zdGluZ0FjY291bnQiOnsicmVnZXgiOiJeYTpheTIwMTY6YW0xMiJ9fX0=
````


#### Output

````
Git Storage
         commit : cb56fdcdd2b56d41fc08cc5af4a3b410896f03b5
      reference : txns-1E5
      directory : txns
         suffix : txn
        message : txns-1E5: 2016/12

Txn Set Checksum
        SHA-256 : 51faa6d2133d22d3ff8b60aff57722d1869fc4677911b13161dce558e7498073
       Set size : 8406

Filter:
  Posting Account: "^a:ay2016:am12$"


BALANCE
-------
                    0.00   -133433.00  a:ay2016
              -133433.00   -133433.00  a:ay2016:am12
========================
              -133433.00
````

## Further info

* [Tackler Journal Format](https://tackler.e257.fi/docs/journal/format/)
* [Txn Filters with Shell Script](https://tackler.e257.fi/docs/usage/#txn-filters-shell)
* [Tackler-NG repository](https://github.com/e257-fi/tackler-ng)
* [Tackler website](https://tackler.e257.fi/)
* [Plain Text Accounting](https://plaintextaccounting.org/)


## Tackler components on Crates.io

* Tackler CLI application: [tackler](https://crates.io/crates/tackler)
* Tackler Client API: [tackler-api](https://crates.io/crates/tackler-api)
* Tackler Server API: [tackler-core](https://crates.io/crates/tackler-core)
* Tackler Rusty Services: [tackler-rs](https://crates.io/crates/tackler-rs)
