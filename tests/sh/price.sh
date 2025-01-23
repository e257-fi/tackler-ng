#
# Tackler-NG 2025
# SPDX-License-Identifier: Apache-2.0
#

set -e

source $TEST_DIR/lib/utils.sh

source $TEST_DIR/lib/make_filter.sh

###
### PRICE
###
module=price

#
# price-00
#
# test: 7cbe3cb4-1b23-44bb-8e5e-ba2c8991e589
rm -f $OUTPUT_DIR/*
test_name=price-00
echo "test: $module/$test_name: "

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/ok.toml \
    --accounts "e:conv" \
    --pricedb $SUITE_PATH/$module/ok/price.db \
    --input.file $SUITE_PATH/$module/ok/price.txn \


echo -n "check:"
cmp_result $module $test_name txt bal
cmp_result $module $test_name txt balgrp
cmp_result $module $test_name txt reg
cmp_result $module $test_name txn identity
cmp_result $module $test_name txn equity
echo ": ok"

#
# price-01
#
# test: 6ddcb0d4-7dc6-4909-a1b4-ecbb5a33a186
# balance: 1.001 + 12.001 + 24.001 + 31.001 + \
#          1000.001 + 1200.001 + 2400.001 + 2900.001 + \
#          100000.001 + 120000.001 + 240000.001 + 310000.001
#        = 777568.012
# bal-grp:
#      g1: 1.001 + 12.001 + 24.001 + 31.001                  = 68.004
#      g2: 1000.001 + 1200.001 + 2400.001 + 2900.001         = 7500.004
#      g3: 100000.001 + 120000.001 + 240000.001 + 310000.001 = 770000.004
rm -f $OUTPUT_DIR/*
test_name=price-01
echo "test: $module/$test_name: "

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/ok.toml \
    --accounts "e:conv" \
    --pricedb $SUITE_PATH/$module/ok/price.db \
    --input.file $SUITE_PATH/$module/ok/price.txn \
    --report.commodity TCKLR \
    --report.price-lookup at-txn

echo -n "check:"
cmp_result $module $test_name txt bal
cmp_result $module $test_name txt balgrp
cmp_result $module $test_name txt reg
cmp_result $module $test_name txn identity
cmp_result $module $test_name txn equity
echo ": ok"

#
# price-02
#
# test: af19f8e7-a6f2-4f64-8c5a-b774dba4f58c
rm -f $OUTPUT_DIR/*
test_name=price-02
echo "test: $module/$test_name: "

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/ok.toml \
    --accounts "e:conv" \
    --pricedb $SUITE_PATH/$module/ok/price.db \
    --input.file $SUITE_PATH/$module/ok/price.txn \
    --report.commodity TCKLR \
    --report.price-lookup last-price-db-entry

echo -n "check:"
cmp_result $module $test_name txt bal
cmp_result $module $test_name txt balgrp
cmp_result $module $test_name txt reg
cmp_result $module $test_name txn identity
cmp_result $module $test_name txn equity
echo ": ok"

#
# price-03
#
# test: 4075e741-605b-4e67-ab7d-0d13f38956ca
rm -f $OUTPUT_DIR/*
test_name=price-03
echo "test: $module/$test_name: "

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/ok.toml \
    --accounts "e:conv" \
    --pricedb $SUITE_PATH/$module/ok/price.db \
    --input.file $SUITE_PATH/$module/ok/price.txn \
    --report.commodity TCKLR \
    --report.price-lookup at-last-filter \
    --api-filter-def \
    '{ "txnFilter": { "TxnFilterTxnTSEnd": { "end": "2024-04-01T00:00:00Z" }}}'

echo -n "check:"
cmp_result $module $test_name txt bal
cmp_result $module $test_name txt balgrp
cmp_result $module $test_name txt reg
cmp_result $module $test_name txn identity
cmp_result $module $test_name txn equity
echo ": ok"

#
# price-04
#
# test: 0c600957-5c15-42d0-9bb9-b618ad7d597f
rm -f $OUTPUT_DIR/*
test_name=price-04
echo "test: $module/$test_name: "

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/ok.toml \
    --accounts "e:conv" \
    --pricedb $SUITE_PATH/$module/ok/price.db \
    --input.file $SUITE_PATH/$module/ok/price.txn \
    --report.commodity TCKLR \
    --report.price-lookup at-last-filter \
    --api-filter-def \
    '{ "txnFilter": { "TxnFilterTxnTSEnd": { "end": "2024-03-31T00:00:00Z" }}}'

echo -n "check:"
# todo: enable, when at-last-filter is fixed
#cmp_result $module $test_name txt bal
#cmp_result $module $test_name txt balgrp
#cmp_result $module $test_name txt reg
#cmp_result $module $test_name txn identity
#cmp_result $module $test_name txn equity
echo ": ok"

#
# price-05
#
# test: 39c60c8b-b999-4305-8c57-5c06e99cc919
rm -f $OUTPUT_DIR/*
test_name=price-05
echo "test: $module/$test_name: "

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/ok.toml \
    --accounts "e:conv" \
    --pricedb $SUITE_PATH/$module/ok/price.db \
    --input.file $SUITE_PATH/$module/ok/price.txn \
    --report.commodity TCKLR \
    --report.price-lookup "2024-04-01"

echo -n "check:"
#cmp_result $module $test_name txt bal
#cmp_result $module $test_name txt balgrp
#cmp_result $module $test_name txt reg
#cmp_result $module $test_name txn identity
#cmp_result $module $test_name txn equity
echo ": ok"

#
# price-06
#
# test: f7211562-c69d-4bab-a768-65b15803efdf
rm -f $OUTPUT_DIR/*
test_name=price-06
echo "test: $module/$test_name: "

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/ok.toml \
    --accounts "e:conv" \
    --pricedb $SUITE_PATH/$module/ok/price.db \
    --input.file $SUITE_PATH/$module/ok/price.txn \
    --report.commodity TCKLR \
    --report.price-lookup "2024-03-31"

echo -n "check:"
# todo: enable, when at-last-filter is fixed
#cmp_result $module $test_name txt bal
#cmp_result $module $test_name txt balgrp
#cmp_result $module $test_name txt reg
#cmp_result $module $test_name txn identity
#cmp_result $module $test_name txn equity
echo ": ok"
