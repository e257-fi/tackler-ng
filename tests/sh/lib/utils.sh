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

cmp_result () {
    local module=$1
    local test_name=$2
    local suffix=$3
    local target=$4

    ref=$SUITE_PATH/$module/ok/$test_name.ref.$target.$suffix
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
