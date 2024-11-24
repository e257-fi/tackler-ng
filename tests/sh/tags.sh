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
### TAGS
###
module=tags

#
# basic-01
#
# test: a9e1ef57-18e7-488a-baa2-b7b05a376801
rm -f $OUTPUT_DIR/*
test_name=basic-01
echo "test: $module/$test_name: "

$TACKLER_SH \
    --config $SUITE_PATH/$module/ok.toml \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --input.file $SUITE_PATH/$module/ok/basic-01.txn

echo -n "check:"
cmp_result $module $test_name txt reg
cmp_result $module $test_name txn identity
echo ": ok"

#
# txn-tags-filter-01
#
# test: 5ea202ba-01d1-4d37-b8cd-544187b56d22
rm -f $OUTPUT_DIR/*
test_name=txn-tags-filter-01
echo "test: $module/$test_name: "

$TACKLER_SH \
    --config $SUITE_PATH/$module/ok.toml \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --input.file $SUITE_PATH/$module/ok/txn-tags-filter.txn \
    --api-filter-def '{ "txnFilter": { "TxnFilterNOT": { "txnFilter": { "TxnFilterTxnTags": { "regex": "reindeer:Normal·Nose" }}}}}'

echo -n "check:"
cmp_result $module $test_name txt reg
cmp_result $module $test_name txn identity
echo ": ok"

#
# txn-tags-filter-02
#
# test: b00719e0-a04d-47ec-b0b9-a3fb00fa6208
rm -f $OUTPUT_DIR/*
test_name=txn-tags-filter-02
echo "test: $module/$test_name: "

$TACKLER_SH \
    --config $SUITE_PATH/$module/ok.toml \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --input.file $SUITE_PATH/$module/ok/txn-tags-filter.txn \
    --api-filter-def '{ "txnFilter": { "TxnFilterAND" : { "txnFilters": [ { "TxnFilterTxnTags": { "regex": "reindeer:.*" } },{ "TxnFilterNOT": { "txnFilter": { "TxnFilterTxnTags": { "regex": "reindeer:Normal·Nose" } } } } ] } }}'

echo -n "check:"
cmp_result $module $test_name txt reg
cmp_result $module $test_name txn identity
echo ": ok"

#
# identity-01
#
# test: 14f69076-b06d-475c-90d5-cfcd6b1ea581
rm -f $OUTPUT_DIR/*
test_name=identity-01
echo "test: $module/$test_name: "

$TACKLER_SH \
    --config $SUITE_PATH/$module/ok.toml \
    --output.dir $OUTPUT_DIR \
    --output.prefix ${test_name}-step1 \
    --input.file $SUITE_PATH/$module/ok/basic-01.txn

$TACKLER_SH \
    --config $SUITE_PATH/tags/ok.toml \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --input.file $OUTPUT_DIR/${test_name}-step1.identity.txn \

echo -n "check:"
cmp_result $module $test_name txt reg
cmp_result $module $test_name txn identity
echo ": ok"

