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

