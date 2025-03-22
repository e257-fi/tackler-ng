#
# Tackler-NG 2024
#
# SPDX-License-Identifier: Apache-2.0
#

set -e -o pipefail

source $TEST_DIR/lib/utils.sh


###
### REPORTING
###
module=reporting

#####################################################################
#
# big-01
#
# test: 0f862997-95b1-4e06-bc5f-bc170c7594ff
rm -f $OUTPUT_DIR/*
test_name=big-01
echo "test: $module/$test_name: "

$TACKLER_SH \
    --config $SUITE_PATH/$module/ok.toml \
    --input.file $SUITE_PATH/$module/ok/big.txn \
    --output.dir $OUTPUT_DIR \
    --output.prefix ${test_name} \

echo -n "check:"
cmp_result $module ${test_name} txt bal
cmp_result $module ${test_name} txt balgrp
cmp_result $module ${test_name} txt reg
cmp_result $module ${test_name} txn equity
cmp_result $module ${test_name} txn identity
echo ": ok"

#####################################################################
#
# big-02
#
# test: 92780169-2419-4a88-8cf5-84994dbca782
rm -f $OUTPUT_DIR/*
test_name=big-02
echo "test: $module/$test_name: "

$TACKLER_SH \
    --config $SUITE_PATH/$module/big-and-small.toml \
    --input.file $SUITE_PATH/$module/big-and-small.txn \
    --output.dir $OUTPUT_DIR \
    --output.prefix ${test_name}-ng \

echo -n "check:"
cmp_result $module ${test_name}-ng txt bal
cmp_result $module ${test_name}-ng txt balgrp
cmp_result $module ${test_name}-ng txt reg
cmp_result $module ${test_name}-ng txn equity
cmp_result $module ${test_name}-ng txn identity
echo ": ok"

#####################################################################
#
# bal-zero
#
# test: e242f20d-4b96-4b9b-8eb3-2eb7b6e2dc6b
rm -f $OUTPUT_DIR/*
test_name=bal-zero
echo "test: $module/$test_name: "

$TACKLER_SH \
    --config $SUITE_PATH/$module/bal-zero.toml \
    --input.file $SUITE_PATH/$module/ok/bal-zero.txn \
    --output.dir $OUTPUT_DIR \
    --output.prefix ${test_name} \

echo -n "check:"
cmp_result $module ${test_name} txt bal
cmp_result $module ${test_name} txt balgrp
cmp_result $module ${test_name} txt reg
cmp_result $module ${test_name} txn equity
cmp_result $module ${test_name} txn identity
echo ": ok"

#####################################################################
#
# bal-acc-01
#
# test: 53f67fea-6307-44ca-9834-7a2f9b71a15a
rm -f $OUTPUT_DIR/*
test_name=bal-acc-01
echo "test: $module/$test_name: "

$TACKLER_SH \
    --config $SUITE_PATH/$module/ok.bal-acc.toml \
    --input.file $SUITE_PATH/$module/ok/reporting.txn \
    --output.dir $OUTPUT_DIR \
    --output.prefix ${test_name}

echo -n "check:"
cmp_result $module ${test_name} txt balgrp
cmp_result $module ${test_name} txt reg
cmp_result $module ${test_name} txn equity
cmp_result $module ${test_name} txn identity
echo ": ok"

# https://github.com/paupino/rust-decimal/issues/695
$TACKLER_SH \
    --config $SUITE_PATH/$module/ok.bal-acc.toml \
    --input.file $SUITE_PATH/$module/ok/reporting.txn \
    --output.dir $OUTPUT_DIR \
    --output.prefix ${test_name}-ng \
    --reports balance

echo -n "check:"
cmp_result $module ${test_name}-ng txt bal
echo ": ok"


#####################################################################
#
# balgrp-acc-01
#
# test: 3ec3e091-dc23-455b-963a-4ba66db7223f
rm -f $OUTPUT_DIR/*
test_name=balgrp-acc-01
echo "test: $module/$test_name: "

$TACKLER_SH \
    --config $SUITE_PATH/$module/ok.balgrp-acc.toml \
    --input.file $SUITE_PATH/$module/ok/reporting.txn \
    --output.dir $OUTPUT_DIR \
    --output.prefix ${test_name} \

echo -n "check:"
cmp_result $module ${test_name} txt bal
cmp_result $module ${test_name} txt reg
cmp_result $module ${test_name} txn equity
cmp_result $module ${test_name} txn identity
echo ": ok"

# https://github.com/paupino/rust-decimal/issues/695
$TACKLER_SH \
    --config $SUITE_PATH/$module/ok.balgrp-acc.toml \
    --input.file $SUITE_PATH/$module/ok/reporting.txn \
    --output.dir $OUTPUT_DIR \
    --output.prefix ${test_name}-ng \
    --reports balance-group

echo -n "check:"
cmp_result $module ${test_name}-ng txt balgrp
echo ": ok"


#####################################################################
#
# register-acc-01
#
# test: 7d95bef8-6aaa-4706-a276-d206752d017b
rm -f $OUTPUT_DIR/*
test_name=register-acc-01
echo "test: $module/$test_name: "

$TACKLER_SH \
    --config $SUITE_PATH/$module/ok.register-acc.toml \
    --input.file $SUITE_PATH/$module/ok/reporting.txn \
    --output.dir $OUTPUT_DIR \
    --output.prefix ${test_name} \

echo -n "check:"
cmp_result $module ${test_name} txt bal
cmp_result $module ${test_name} txt balgrp
cmp_result $module ${test_name} txt reg
cmp_result $module ${test_name} txn equity
cmp_result $module ${test_name} txn identity
echo ": ok"

#####################################################################
#
# rep-01
#
# test: c6da0aef-125f-4d33-9780-ffaa9e724499
rm -f $OUTPUT_DIR/*
test_name=rep-01
echo "test: $module/$test_name: "

$TACKLER_SH \
    --config $SUITE_PATH/$module/ok.toml \
    --input.file $SUITE_PATH/$module/ok/reporting.txn \
    --output.dir $OUTPUT_DIR \
    --output.prefix ${test_name} \

echo -n "check:"
cmp_result $module ${test_name} txt bal
cmp_result $module ${test_name} txt balgrp
cmp_result $module ${test_name} txt reg
cmp_result $module ${test_name} txn equity
cmp_result $module ${test_name} txn identity
echo ": ok"

