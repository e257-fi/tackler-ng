#!/bin/bash
#
# Generate ANTRL parser
#
# Setup:
#  - tmp/antlr4-4.8-2-SNAPSHOT-complete.jar
#  - tackler-ng/tackler-ng/tackler-core/src/parser/txn_antlr
#
java -jar  ../../../../tmp/antlr4-4.8-2-SNAPSHOT-complete.jar -Dlanguage=Rust TxnLexer.g4 TxnParser.g4

# fix warnings
git apply txn_antlr.patch
