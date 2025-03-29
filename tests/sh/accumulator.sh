#
# Tackler-NG 2024-2025
# SPDX-License-Identifier: Apache-2.0
#

set -e -o pipefail

source $TEST_DIR/lib/utils.sh

###
### ACCUMULATOR
###
module=accumulator

accumulator_test () {
    local test_name=$1

    echo "test: $module/$test_name: "

    rm -f $OUTPUT_DIR/*
    $TACKLER_SH \
        --config $SUITE_PATH/$module/ok.toml \
        --output.dir $OUTPUT_DIR \
        --output.prefix $test_name \
        --input.file $SUITE_PATH/$module/ok/$test_name.txn

    echo -n "check:"
    cmp_result $module $test_name txt bal
    cmp_result $module $test_name txt balgrp
    cmp_result $module $test_name txt reg
    cmp_result $module $test_name txn equity
    echo ": ok"
}

# test: e094ee52-1742-4da1-bc54-15662c293a69
accumulator_test bal-01

# test: fb5c09d2-dfbf-41b7-9412-81bcf37f0c76
accumulator_test bal-02

# test: 9220c02f-1b2a-4f7b-9ae5-8dc16bf1f071
accumulator_test bal-gap-01

# test: cb7e3f4c-a709-4f0e-9f22-336755dfe6b2
accumulator_test bal-gap-02

# test: 9e00e09f-8ac4-4fd7-b3c1-e383d711d48c
accumulator_test bal-gap-03

# test: 2abde44e-929a-467b-9f57-87c8945451c7
accumulator_test eq-zeros

#####################################################################
#
# test: 76da1ba6-b17d-4260-bc2d-7d1dcca54d50
# test-ref: 25d62f31-e420-4c46-8397-664983753c7f
rm -f $OUTPUT_DIR/*
test_name=eq-uuid-01
echo "test: $module/$test_name: "

$TACKLER_SH \
    --config $SUITE_PATH/$module/ok.toml \
    --input.file $SUITE_PATH/$module/ok/eq-uuid.txn \
    --output.dir $OUTPUT_DIR \
    --output.prefix ${test_name}-step1 \
    --accounts "^a.*"

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/ok.toml \
    --input.file $OUTPUT_DIR/${test_name}-step1.equity.txn \
    --reports balance balance-group register \
    --exports equity identity \

echo -n "check:"
cmp_result $module $test_name txt bal
cmp_result $module $test_name txt balgrp
cmp_result $module $test_name txt reg
cmp_result $module $test_name txn equity
cmp_result $module $test_name txn identity
echo ": ok"

#####################################################################
#
# test: f46b00a3-b4dc-44e0-a8ae-b8039e2a33a7
rm -f $OUTPUT_DIR/*
test_name=eq-uuid-02
echo "test: $module/$test_name: "

$TACKLER_SH \
    --config $SUITE_PATH/$module/ok.toml \
    --input.file $SUITE_PATH/$module/ok/eq-uuid.txn \
    --output.dir $OUTPUT_DIR \
    --output.prefix ${test_name} \
    --accounts "^a.*"

echo -n "check:"
cmp_result $module $test_name txt bal
cmp_result $module $test_name txt balgrp
cmp_result $module $test_name txt reg
cmp_result $module $test_name txn equity
echo ": ok"
