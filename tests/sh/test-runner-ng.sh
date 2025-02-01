#!/bin/bash
#
# Tackler-NG 2024
#
# SPDX-License-Identifier: Apache-2.0
#

set -e

TEST_DIR="$(readlink -f $(dirname $(realpath $0)))"
export TEST_DIR

TACKLER_ROOT=$(realpath "$TEST_DIR/../..")
export TACKLER_ROOT

target="unknown"
if [ "$1" == "--debug" ]; then
	target="debug"
else
	target="release"
fi
TACKLER_SH=$TACKLER_ROOT/target/$target/tackler
export TACKLER_SH

SUITE_PATH=$TACKLER_ROOT/suite
export SUITE_PATH

OUTPUT_DIR=$(mktemp -d $TACKLER_ROOT/target/tmp-XXX)
export OUTPUT_DIR

source $TEST_DIR/lib/utils.sh


$TEST_DIR/accumulator.sh
$TEST_DIR/audit.sh
$TEST_DIR/commodity.sh
$TEST_DIR/core.sh
$TEST_DIR/gas-blender.sh
$TEST_DIR/location.sh
$TEST_DIR/parser.sh
$TEST_DIR/reporting-equity.sh
$TEST_DIR/reporting-group_by.sh
$TEST_DIR/reporting-ok.sh
$TEST_DIR/reporting-time_zones.sh
$TEST_DIR/tags.sh
$TEST_DIR/new-and-init.sh
$TEST_DIR/price.sh
$TEST_DIR/price-strict.sh
$TEST_DIR/price-multi.sh
$TEST_DIR/price-multi-miss.sh

echo
echo "target: $target - All good"

rm -rf $OUTPUT_DIR
exit 0
