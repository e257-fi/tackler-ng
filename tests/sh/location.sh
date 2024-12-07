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
### LOCATION
###
module=location

#####################################################################
#
# basic-01
#
# test: 8adbd77d-dd49-41a7-9412-fa9189ce3db6
rm -f $OUTPUT_DIR/*
test_name=basic-01
echo "test: $module/$test_name: "

$TACKLER_SH \
    --config $SUITE_PATH/$module/ok.toml \
    --input.file $SUITE_PATH/$module/ok/basic-01.txn \
    --output.dir $OUTPUT_DIR \
    --output.prefix ${test_name} \

echo -n "check:"
cmp_result $module ${test_name} txt reg
cmp_result $module ${test_name} txn identity
echo ": ok"


#####################################################################
#
# identity-01
#
# test: bb9cee1f-a0e6-45fc-9815-9ad9875e4bd4
rm -f $OUTPUT_DIR/*
test_name=identity-01
echo "test: $module/$test_name: "

$TACKLER_SH \
    --config $SUITE_PATH/$module/ok.toml \
    --input.file $SUITE_PATH/$module/ok/basic-01.txn \
    --output.dir $OUTPUT_DIR \
    --output.prefix ${test_name}-step1 \

$TACKLER_SH \
    --config $SUITE_PATH/$module/ok.toml \
    --input.file $OUTPUT_DIR/${test_name}-step1.identity.txn \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \

echo -n "check:"
cmp_result $module $test_name txt reg
cmp_result $module $test_name txn identity
echo ": ok"
