#!/bin/bash
#
# Generate ANTRL parser
#
java -jar ../../tmp/antlr4-4.8-2-SNAPSHOT-complete.jar -Dlanguage=Rust TxnLexer.g4 TxnParser.g4

# fix warnings
git apply txnparser.patch
