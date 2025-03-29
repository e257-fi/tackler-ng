#
# Tackler-NG 2025
# SPDX-License-Identifier: Apache-2.0
#

set -eu

###
### PARSER: Errors
###
module=parser
mode="error"

#####################################################################
#
# test: 0e2a5c79-0594-4c4f-aa65-b31bff877b53
# desc: parse error with shard
test_name=shard-01
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --config $SUITE_PATH/$module/ok.toml \
    --input.fs.dir $SUITE_PATH/$module/ex/perr-txns \
    --input.fs.ext txn \
    2>&1 | grep 'Tackler error: Txn Data: parse error at line 2, column 3'

echo "check: ok"
