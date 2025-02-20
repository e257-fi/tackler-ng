# vim: tabstop=4 shiftwidth=4 softtabstop=4 smarttab expandtab autoindent
#
# Tackler-NG 2025
# SPDX-License-Identifier: Apache-2.0
#
default:
    @just --list

alias c  := check
alias ut := unit-test
alias it := integration-test

alias qa := release-qa

alias db := debug-build
alias rb := release-build

clean:
    cargo clean

check: clippy fmt

clippy:
    cargo clippy --workspace --all-targets --no-deps -- -D warnings

fmt:
    cargo fmt --all -- --style-edition 2024

test: (_test "debug")

release-qa: clean check audit (_test "release")

_test target: unit-test (_build target) (_examples-test target) (_integration-test target)

unit-test:
    cargo test

integration-test: (_build "debug") (_integration-test "debug")

_integration-test target:
    sh tests/sh/test-runner-ng.sh --{{target}}

_examples-test target: (_build target)
    target/{{ target }}/tackler --config "{{justfile_directory()}}/examples/simple.toml"
    target/{{ target }}/tackler --config "{{justfile_directory()}}/examples/audit.toml"

_build target:
    @if [ "debug" = "{{ target }}" ]; then cargo build --bin tackler; else cargo build --release --bin tackler; fi


debug-build: (_build "debug")

release-build: (_build "release")

bench: git_bench
    cargo bench parser


git_bench:
    cargo run --release -p tackler-core

audit:
    cargo deny check advisories bans licenses sources

