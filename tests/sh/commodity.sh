#
# Copyright 2024 E257.FI
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#

set -e

source $TEST_DIR/lib/utils.sh

###
### COMMODITY
###
module=commodity

commodity_test () {
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
    cmp_result $module $test_name txn identity
    echo ": ok"
}

# test: a5318e06-309e-4c52-996b-b07aefac4c46
commodity_test basic-01

# test: 04af8049-355d-4c6c-ac63-c153b0141c48
commodity_test basic-02

# test: 8a7a8c60-aab9-4700-b081-9804c1ec537f
commodity_test basic-03

# test: c5493e99-d0f3-49b3-8450-3ae6ef970b10
commodity_test basic-04

# test: aeffbcc2-bcc4-40f3-9b37-03dbca3bff88
commodity_test basic-05

# test: 0dae7dfd-c816-4a77-afff-a372e1802026
commodity_test basic-06

# test: 54cb6baa-6c3d-4196-94bc-b6fd37ed0b57
commodity_test valpos-01

# test: 2501f96c-46d8-4711-9e7e-dc70e4af329d
commodity_test valpos-02

# test: d0dd6b75-238e-49cf-a048-a77baafba9bd
commodity_test valpos-03

#
# valpos-01-01
#
# test: 773106ca-d6e5-4514-b19b-3e4d34283152
rm -f $OUTPUT_DIR/*
test_name=valpos-01-01
echo "test: $module/$test_name: "

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix ${test_name}-step1 \
    --config $SUITE_PATH/$module/ok.toml \
    --input.file $SUITE_PATH/$module/ok/valpos-01.txn

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/ok.toml \
    --input.file $OUTPUT_DIR/${test_name}-step1.identity.txn

echo -n "check:"
cmp_result $module $test_name txt bal
cmp_result $module $test_name txt balgrp
cmp_result $module $test_name txt reg
cmp_result $module $test_name txn equity
cmp_result $module $test_name txn identity
echo ": ok"



commodity_filter_test () {
    local test_name=$1

    echo "test: $module/filter-$test_name: "

    rm -f $OUTPUT_DIR/*
    $TACKLER_SH \
        --config $SUITE_PATH/$module/ok.toml \
        --output.dir $OUTPUT_DIR \
        --output.prefix filter-$test_name \
        --input.file "$SUITE_PATH/$module/ok/basic-$test_name".txn \
        --accounts '^a.*'

    echo -n "check:"
    cmp_result $module filter-$test_name txt bal
    cmp_result $module filter-$test_name txt balgrp
    cmp_result $module filter-$test_name txt reg
    cmp_result $module filter-$test_name txn equity
    cmp_result $module filter-$test_name txn identity
    echo ": ok"
}

# test: 1df0fb31-6604-4679-9ec4-7a1d65e49574
commodity_filter_test "01"

# test: 633da495-ef30-4484-8b14-fbd1fdcc9684
commodity_filter_test "02"

# test: 09e9a673-4730-4a03-b73a-5959730450f6
commodity_filter_test "03"
