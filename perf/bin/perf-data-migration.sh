#!/bin/bash
# vim: tabstop=4 shiftwidth=4 smarttab expandtab softtabstop=4 autoindent
#
# Copyright 2018-2024 E257.FI
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
#############################################################################

ds_path=$1
version=$2
ds=$3

hdr_file=$ds_path/$version-perf-all-git-$ds-balance-txt.txt

get_value() {
    local f=$1
    local key=$2

    grep -h "$key:" "$f" | sed "s/$key: //"
}

# collect header
cat <<EOF
exe: "$(get_value $hdr_file "exe")"
build: "$(get_value $hdr_file "build")"
version: "$(get_value $hdr_file "version")"
set: "$ds"
runs:
EOF

get_v () {
    local f=$1
    local key=$2
    local indent="$3"
    grep -h ^$key $f | sed -E "s/^(.*)/$indent - \"\1\"/"
}

get_v2 () {
    local f=$1
    local key=$2
    local unit=$3
    local indent="$4"

    grep $key $f | sed "s/$key\t\(.*\)$unit.*/$indent - \1/"
}

handle_file() {

    local f=$1
cat <<EOF
  - run:
      storage: "$(echo $f | sed -E 's/.*perf-((all)|(flt))-(.*)-'$ds'.*/\4/')"
      report: "$(echo $f | sed -E "s/.*-((balance)|(balance-group)|(register))-((json)|(txt)|(txt_json))\.txt$/\1/")"
      formats: "$(echo $f | sed -E 's/.*-((json)|(txt)|(txt_json))\.txt$/\1/')"
      filter: |
          $(get_value $f "filter")
      result:
        size:
          logs:
EOF
cat << EOF
$(get_v $f "Txns" "           ")
EOF

cat << EOF
        prosessing:
          logs:
EOF

cat << EOF
$(get_v $f "Total " "           ")
EOF

cat << EOF
        times:
          real:
            unit: "s"
            values:
EOF
cat << EOF
$(get_v2 $f "real" "s" "             ")
EOF
cat << EOF
          user:
            unit: "s"
            values:
EOF
cat << EOF
$(get_v2 $f "user" "s" "             ")
EOF
cat << EOF
          sys:
            unit: "s"
            values:
EOF
cat << EOF
$(get_v2 $f "sys" "s" "             ")
EOF


cat << EOF
        mem:
          unit: "k (max)"
          values:
EOF
cat << EOF
$(get_v2 $f "mem" "k (max)" "            ")
EOF

cat << EOF
        cpu:
          unit: "%"
          values:
EOF
cat << EOF
$(get_v2 $f "cpu" "%" "            ")
EOF

}


for i in $ds_path/$version*-$ds-*
do
    handle_file $i
    #echo $i
done
