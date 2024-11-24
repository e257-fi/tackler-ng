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

