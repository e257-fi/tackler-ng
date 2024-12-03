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
### REPORTING / EQUITY
###
module=reporting/equity

#####################################################################
#
# equity-acc-01
#
# test: 29d24d2e-702f-4c27-b5de-82ac88ca68ca
rm -f $OUTPUT_DIR/*
test_name=equity-acc-01-ng
echo "test: $module/$test_name: "

$TACKLER_SH \
    --input.file $SUITE_PATH/$module/../ok/reporting.txn \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/../ok.equity-acc.toml 

echo -n "check:"
cmp_result $module $test_name txt bal "."
cmp_result $module $test_name txt balgrp "."
cmp_result $module $test_name txt reg "."
cmp_result $module $test_name txn equity "."
cmp_result $module $test_name txn identity "."
echo ": ok"

#####################################################################
#
# equity-tep1014-02
#
# This is variant of c0b34ebb-a5d2-4b09-96cb-2594b3635650 with
# maximum precision points of rust-decimal
#
# test: 8542680a-2e94-437f-b0a5-cdef46ef8521
rm -f $OUTPUT_DIR/*
test_name=equity-tep1014-01-ng
echo "test: $module/$test_name: "

$TACKLER_SH \
    --input.file $SUITE_PATH/$module/../big-and-small.txn \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/../ok.equity-tep1014.toml 

echo -n "check:"
cmp_result $module $test_name txt bal "."
cmp_result $module $test_name txn equity "."
echo ": ok"


#####################################################################
#
# equity-tep1014-as-input
#
# This is variant of 78982e06-4f11-4f52-8689-df400b8c3a93 with
# maximum precision points of rust-decimal
#
# test: c2e27940-a141-41ed-adc2-da294d519804
rm -f $OUTPUT_DIR/*
test_name=equity-tep1014-as-input
echo "test: $module/$test_name: "

mkdir -p $OUTPUT_DIR/txnsout

$TACKLER_SH \
    --config $SUITE_PATH/$module/../ok.equity-tep1014.toml \
    --input.file $SUITE_PATH/$module/../big-and-small.txn \
    --output.dir $OUTPUT_DIR/txnsout \
    --output.prefix equity-tep1014-input-01 \

$TACKLER_SH \
    --config $SUITE_PATH/$module/../ok.equity-tep1014.toml \
    --input.file $SUITE_PATH/$module/equity-tep1014-as-input.txn \
    --output.dir $OUTPUT_DIR/txnsout \
    --output.prefix equity-tep1014-input-02 \
    --accounts 'a(:.*)?' \

$TACKLER_SH \
    --config $SUITE_PATH/$module/../ok.equity-tep1014.toml \
    --input.file $SUITE_PATH/$module/equity-tep1014-as-input2.txn \
    --output.dir $OUTPUT_DIR/txnsout \
    --output.prefix equity-tep1014-input-03 \
    --accounts 'a(:.*)?' 'e(:.*)?' \

$TACKLER_SH \
    --config $SUITE_PATH/$module/../ok.equity-tep1014.toml \
    --input.fs.dir  $OUTPUT_DIR/txnsout \
    --input.fs.ext "txn" \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name-ng \
    --accounts 'a(:.*)?' 'e(:.*)?' \
    --exports "identity" \


echo -n "check:"
cmp_result $module $test_name-ng txn identity "."
echo ": ok"
rm -rf $OUTPUT_DIR/*
