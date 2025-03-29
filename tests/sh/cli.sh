#
# Tackler-NG 2025
# SPDX-License-Identifier: Apache-2.0
#

set -e -o pipefail

source $TEST_DIR/lib/utils.sh

###
### CLI
###
module=cli

#
# test: cd54250a-8af2-4daa-9d8e-7870b5987da9
# desc: unknown account ok case
rm -f $OUTPUT_DIR/*
test_name=unknown-acc-01
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/basic.toml \
    --input.file $SUITE_PATH/$module/ex/unknown-acc-01.txn \
    --exports identity \

echo -n "check:"
cmp_result $module $test_name txn identity
echo ": ok"
