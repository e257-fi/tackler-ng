# Tackler-NG: Tackler CLI Application

[![Build Status](https://github.com/e257-fi/tackler-ng/actions/workflows/ci.yml/badge.svg)](https://github.com/e257-fi/tackler-ng/actions)
[![Github Releases](https://img.shields.io/github/v/release/e257-fi/tackler-ng?include_prereleases&color=%230868da)](https://github.com/e257-fi/tackler-ng/releases)
[![Chat on Matrix](https://tackler.e257.fi/img/badge-matrix.svg)](https://matrix.to/#/#tackler:matrix.org)
[![Tackler Docs](https://img.shields.io/badge/tackler-documentation-%23ffcb00)](https://tackler.e257.fi/docs)


Tackler is fast, reliable bookkeeping engine with native GIT SCM 
support for plain text accounting written in rust. 
Tackler-NG is rusty version of [Tackler](https://tackler.e257.fi/).

## Project Status

The rusty Tackler-NG is in [feature](https://tackler.e257.fi/features/)
parity with old scala based Tackler  CLI, and this will be 
the basis of all future Tackler development.


Basic functionality is fully supported, including 
[Tackler Journal Format](https://tackler.e257.fi/docs/journal/format/), 
transaction storages ([FS backend](https://tackler.e257.fi/docs/usage/#storage-selector), 
[Gitoxide](https://github.com/GitoxideLabs/gitoxide/) based [Git backend](https://tackler.e257.fi/docs/journal/git-storage/)), 
all reports 
([Balance](https://tackler.e257.fi/docs/report-balance/), 
[Balance Group](https://tackler.e257.fi/docs/report-balance-group/), 
[Register](https://tackler.e257.fi/docs/report-register/))
and all exports 
([Equity](https://tackler.e257.fi/docs/export-equity/), 
[Identity](https://tackler.e257.fi/docs/export-equity/)).

Other notable supported features are:

* [Commodities](https://tackler.e257.fi/docs/commodities/) and [Shares](https://tackler.e257.fi/docs/currencies/)
* All [Transaction Filters](https://tackler.e257.fi/docs/txn-filters/)
* Accounting based on [Geo Location](https://tackler.e257.fi/docs/gis/txn-geo-location/) and [Transaction GIS Filters](https://tackler.e257.fi/docs/gis/txn-geo-filters/)

See `tackler --help` and [Tackler Configuration](https://github.com/e257-fi/tackler-ng/blob/main/examples/tackler.toml) how to use tackler-ng.

**NOTE: Tackler-NG is tested with the most essential tackler tests vectors at the moment.**

## Build and install tackler


````bash
# Get the source code
git clone --recurse-submodules https://github.com/e257-fi/tackler-ng

cd tackler-ng

# The main branch should build and pass all tests.
# To build a release, check relased versions:
git tag -l

# Get the release, e.g. v24.11.2
git checkout v24.11.2

# Build tackler
cargo build --release --locked --bin tackler

# the binary is located at 'target/release/tackler'
````

## Simple example


This setup doesn't have any checks enabled and it uses plain filesystem as transaction storage.

#### Command
````bash
target/release/tackler --config examples/simple.toml
````
#### Output

````
**********************************************************************************

Balance Report
--------------
                 0.00   12.00  Expenses:Food
                12.00   12.00  Expenses:Food:Groceries
                 0.00    3.32  Expenses:Sweets
                 2.12    2.12  Expenses:Sweets:Ice·cream
                 1.20    1.20  Expenses:Sweets:Salmiakki
=====================
                15.32
##################################################################################
**********************************************************************************
...
...
...
````

## Let's play for real

Following examples use bare git repository as transaction storage, 
and also strict and audit mode is activated by configuration.

The triplet of git commit id, Txn Set Checksum and 
Account Selector Checksum provides auditable (cryptographic)
proof of transactions used by reports.

### Use Git repository as Transaction storage

#### Reports with Txn Checksum

````bash
target/release/tackler \
    --config examples/audit.toml \
````

#### Output

````
Git Storage
         commit : 4aa4e9797501c1aefc92f32dff30ab462dae5545
      reference : txns-1E1
      directory : txns
         suffix : .txn
        message : txns-1E1: 2016/12

Txn Set Checksum
        SHA-256 : 9b29071e1bf228cfbd31ca2b8e7263212e4b86e51cfee1e8002c9b795ab03f76
       Set size : 10

**********************************************************************************
Account Selector Checksum
        SHA-256 : 19d31a48bf9a8604a1128ccfd281511f961c5469748a97897a21fc0fa2a5f519

Balance Report
--------------
                 0.00   -161.0000  a:ay2016
              -6.0000     -6.0000  a:ay2016:am02
             -14.0000    -14.0000  a:ay2016:am03
             -19.0000    -19.0000  a:ay2016:am04
             -26.0000    -26.0000  a:ay2016:am05
              -1.0000     -1.0000  a:ay2016:am07
              -7.0000     -7.0000  a:ay2016:am08
             -13.0000    -13.0000  a:ay2016:am09
             -19.0000    -19.0000  a:ay2016:am10
             -25.0000    -25.0000  a:ay2016:am11
             -31.0000    -31.0000  a:ay2016:am12
=====================
            -161.0000
##################################################################################
````

#### Report with 100_000 Transactions

There is git ref 'txns-1E5' inside the example audit -repository.

````bash
target/release/tackler \
    --config examples/audit.toml \
    --input.git.ref txns-1E5
````

#### Output

````
Git Storage
         commit : cb56fdcdd2b56d41fc08cc5af4a3b410896f03b5
      reference : txns-1E5
      directory : txns
         suffix : .txn
        message : txns-1E5: 2016/12

Txn Set Checksum
        SHA-256 : 27060dc1ebde35bebd8f7af2fd9815bc9949558d3e3c85919813cd80748c99a7
       Set size : 100000

**********************************************************************************
Account Selector Checksum
        SHA-256 : 19d31a48bf9a8604a1128ccfd281511f961c5469748a97897a21fc0fa2a5f519

Balance Report
--------------
                     0.00   -1574609.0100  a:ay2016
             -135600.0008    -135600.0008  a:ay2016:am01
             -118950.0008    -118950.0008  a:ay2016:am02
             -135631.0008    -135631.0008  a:ay2016:am03
             -127137.0008    -127137.0008  a:ay2016:am04
             -135616.0008    -135616.0008  a:ay2016:am05
             -127154.0008    -127154.0008  a:ay2016:am06
             -135600.0008    -135600.0008  a:ay2016:am07
             -135603.0008    -135603.0008  a:ay2016:am08
             -127140.0008    -127140.0008  a:ay2016:am09
             -135619.0008    -135619.0008  a:ay2016:am10
             -127126.0008    -127126.0008  a:ay2016:am11
             -133433.0008    -133433.0008  a:ay2016:am12
=========================
            -1574609.0100
##################################################################################
````

### Transaction Filters

#### Filter definition

````bash
target/release/tackler \
    --config examples/audit.toml \
    --input.git.ref txns-1E5 \
    --api-filter-def '{"txnFilter":{"TxnFilterPostingAccount":{"regex":"^a:ay2016:am12"}}}'
````

The transaction filter definition could be given also 
as Base64 ascii armored string:

````
--api-filter-def \
base64:eyJ0eG5GaWx0ZXIiOnsiVHhuRmlsdGVyUG9zdGluZ0FjY291bnQiOnsicmVnZXgiOiJeYTpheTIwMTY6YW0xMiJ9fX0=
````


#### Output

````
Git Storage
         commit : cb56fdcdd2b56d41fc08cc5af4a3b410896f03b5
      reference : txns-1E5
      directory : txns
         suffix : .txn
        message : txns-1E5: 2016/12

Txn Set Checksum
        SHA-256 : 51faa6d2133d22d3ff8b60aff57722d1869fc4677911b13161dce558e7498073
       Set size : 8406

Filter
  Posting Account: "^a:ay2016:am12"

**********************************************************************************
Account Selector Checksum
        SHA-256 : 19d31a48bf9a8604a1128ccfd281511f961c5469748a97897a21fc0fa2a5f519

Balance Report
--------------
                    0.00   -133433.0008  a:ay2016
            -133433.0008   -133433.0008  a:ay2016:am12
========================
            -133433.0008
##################################################################################
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
