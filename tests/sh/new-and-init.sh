#
# Tackler-NG 2025
# SPDX-License-Identifier: Apache-2.0
#

set -e

source $TEST_DIR/lib/utils.sh

###
### CMD: NEW and INIT
###
module=new-and-init

#
# new-01
#
# test: a405a6f4-be40-429d-a027-1ec80734dec2
rm -rf $OUTPUT_DIR/*
test_name=new-01
echo "test: $module/$test_name: "

journal_path=$OUTPUT_DIR/$module/$test_name

$TACKLER_SH \
    new $journal_path

echo "check:"
$TACKLER_SH \
    --config $journal_path/conf/tackler.toml

echo ": ok"

#
# init-01
#
# test: 720220ba-ba95-44f4-86d0-9390afa462de
rm -rf $OUTPUT_DIR/*
test_name=init-01
echo "test: $module/$test_name: "

journal_path=$OUTPUT_DIR/$module/$test_name
(
	mkdir -p $journal_path
	cd $journal_path
	$TACKLER_SH init
)

echo "check:"
$TACKLER_SH \
    --config $journal_path/conf/tackler.toml

echo ": ok"

rm -rf $OUTPUT_DIR/*
