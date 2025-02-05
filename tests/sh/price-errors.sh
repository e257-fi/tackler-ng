#
# Tackler-NG 2025
# SPDX-License-Identifier: Apache-2.0
#

set -eu

source $TEST_DIR/lib/utils.sh

###
### PRICE: ERRORS
###
module=price
mode="error"

#
# ERR: GIVEN-TIME-01
#
# test: ccccb29e-3229-4461-abfd-ae6ad65058a6
# desc: given time, no timestamp
test_name=given-time-01
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/price.toml \
    --input.file $SUITE_PATH/$module/ok/price.txn \
    --price.lookup-type "given-time" \
    2>&1 | grep 'Tackler error:.* "given-time" .* no timestamp'

echo "check: ok"

#
# ERR: GIVEN-TIME-02
#
# test: 0a7fa119-797f-4744-821e-44af958675c9
# desc: wrong type (txn-time) by conf vs. given-time
test_name=given-time-02
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/price-ts.toml \
    --input.file $SUITE_PATH/$module/ok/price.txn \
    2>&1 | grep 'Tackler error:.* "given-time" .* no timestamp'

echo "check: ok"


#
# ERR: GIVEN-TIME-03
#
# test: 2b3d0fb2-2a55-4604-bf89-fc50e52aa69d
# desc: wrong type (txn-time) by cli vs. given-time
test_name=given-time-03
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/price.toml \
    --input.file $SUITE_PATH/$module/ok/price.txn \
    --price.lookup-type "txn-time" \
    --price.before "2025-01-01" \
    2>&1 | grep 'Tackler error:.* "before timestamp" .* "txn-time"'

echo "check: ok"

#
# ERR: GIVEN-TIME-04
#
# test: f8c64b01-66c5-4541-b3d5-24b83f44666b
# desc: wrong type (last-price) by cli vs. given-time
test_name=given-time-04
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/price.toml \
    --input.file $SUITE_PATH/$module/ok/price.txn \
    --price.lookup-type "last-price" \
    --price.before "2025-01-01" \
    2>&1 | grep 'Tackler error:.* "before timestamp" .* "last-price"'

echo "check: ok"

#
# ERR: GIVEN-TIME-05
#
# test: adae9328-c7b8-42f5-8d85-099b012c4138
# desc: wrong type (none) by cli vs. given-time
test_name=given-time-05
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/price.toml \
    --input.file $SUITE_PATH/$module/ok/price.txn \
    --price.lookup-type "none" \
    --price.before "2025-01-01" \
    2>&1 | grep 'Tackler error:.* "before timestamp" .* "none"'

echo "check: ok"
