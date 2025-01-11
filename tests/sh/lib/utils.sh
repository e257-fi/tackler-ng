#
# Tackler-NG 2024
#
# SPDX-License-Identifier: Apache-2.0
#

cmp_result () {
    local module=$1
    local test_name=$2
    local suffix=$3
    local target=$4
    local ok_path="$5"

    if [ "x$ok_path" == "x" ]; then
	ok="ok"
    else
	ok=$ok_path
    fi
    ref=$SUITE_PATH/$module/$ok/$test_name.ref.$target.$suffix
    out=$OUTPUT_DIR/$test_name.$target.$suffix

    cmp $ref $out || {
        diff -u $ref $out
        echo
        echo "ref: $ref"
        echo "out: $out"
        exit 1
    }
    echo -n " $target"
}
