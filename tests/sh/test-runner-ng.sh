#!/bin/bash
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

TEST_DIR="$(readlink -f $(dirname $(realpath $0)))"
export TEST_DIR

TACKLER_ROOT=$(realpath "$TEST_DIR/../..")
export TACKLER_ROOT

TACKLER_SH=$TACKLER_ROOT/target/release/tackler
export TACKLER_SH

SUITE_PATH=$TACKLER_ROOT/suite
export SUITE_PATH

OUTPUT_DIR=$(mktemp -d $TACKLER_ROOT/target/tmp-XXX)
export OUTPUT_DIR

source $TEST_DIR/lib/utils.sh


#echo > $OUTPUT_DIR/$test_name.bal.txt
$TEST_DIR/accumulator.sh
$TEST_DIR/audit.sh
$TEST_DIR/commodity.sh
$TEST_DIR/gas-blender.sh
$TEST_DIR/tags.sh

echo
echo "All good"

rm -rf $OUTPUT_DIR
exit 0
