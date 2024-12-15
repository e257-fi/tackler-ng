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

exe_dir=$(dirname $0)

data_dir="$exe_dir/../data/perf-data"

reports="balance balance-group register"
#reports="balance"

#sets="1E1 1E2 1E3 1E4 1E5 1E6"
sets="1E3 1E4 1E5 1E6"
#sets="1E3"

versions="24.12.1"

fltStr="base64:"$(cat << EOF | base64 --wrap=0
{ "txnFilter": { "TxnFilterAND" : { "txnFilters" : [ { "TxnFilterTxnCode": { "regex": "#.*" }},  { "TxnFilterTxnDescription": { "regex": "txn-.*" } } ] } } }
EOF
)

for s in $sets; do
	(cd $data_dir; git checkout txns-$s)
for v in $versions; do
for r in $reports; do

#for frmt in txt json; do
for frmt in txt; do

for filter in "" "$fltStr"; do
#for filter in ""; do

if [ -n "$filter" ]; then
       flt="filter"
else
       flt="all"
fi


echo "run: $v fs $s $r $frmt $flt"
$exe_dir/perf-run.sh dist/tackler-$v fs $s $r "$frmt" "$filter"

done
done
done
done
done


for v in $versions; do
for s in $sets; do
for r in balance; do
for frmt in txt; do

filter=""

echo "run: $v git $s $r $frmt all"
$exe_dir/perf-run.sh dist/tackler-$v git $s $r "$frmt" "$filter"

done
done
done
done

