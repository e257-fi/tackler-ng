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
### REPORTING / TIME AND ZONES
###
module=reporting/time-and-zones

#####################################################################
#
# date
#
# test: 539b2c43-54b1-47e9-98cc-a5a97a76b83c
rm -f $OUTPUT_DIR/*
test_name=date
echo "test: $module/$test_name: "

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name-ng \
    --config $SUITE_PATH/$module/date.toml

echo -n "check:"
cmp_result $module $test_name-ng txt balgrp "."
cmp_result $module $test_name-ng txt reg "."
echo ": ok"

#####################################################################
#
# iso-week-date
#
# test: 2173f557-4f58-4483-b959-dc29f4f98ce4
rm -f $OUTPUT_DIR/*
test_name=iso-week-date
echo "test: $module/$test_name: "

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/iso-week-date.toml

echo -n "check:"
cmp_result $module $test_name txt balgrp "."
echo ": ok"

#####################################################################
#
# iso-week
#
# test: c97ac3bf-7561-4a79-9d00-a582d9949ea8
rm -f $OUTPUT_DIR/*
test_name=iso-week
echo "test: $module/$test_name: "

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/iso-week.toml

echo -n "check:"
cmp_result $module $test_name txt balgrp "."
cmp_result $module $test_name txt reg "."
echo ": ok"

#####################################################################
#
# month
#
# test: c97ac3bf-7561-4a79-9d00-a582d9949ea8
rm -f $OUTPUT_DIR/*
test_name=month
echo "test: $module/$test_name: "

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name \
    --config $SUITE_PATH/$module/month.toml

echo -n "check:"
cmp_result $module $test_name txt balgrp "."
cmp_result $module $test_name txt reg "."
echo ": ok"


#####################################################################
#
# year
#
# test: 982dc27f-674e-47a7-bb20-cef06ea6b5fe
rm -f $OUTPUT_DIR/*
test_name=year
echo "test: $module/$test_name: "

$TACKLER_SH \
    --output.dir $OUTPUT_DIR \
    --output.prefix $test_name-ng \
    --config $SUITE_PATH/$module/year.toml

echo -n "check:"
cmp_result $module $test_name-ng txt balgrp "."
cmp_result $module $test_name-ng txt reg "."
echo ": ok"

