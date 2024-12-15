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


exe_path="$1"
storage="$2"
trg="$3"
report="$4"
formats="$5"
filter="$6"


build="$($exe_path --version | sed 's/^tackler //')"
version="$(echo $build | sed 's/\([^ ]\+\) .*/\1/')"

if [ "$storage" = "fs" ];then
	#inputSelector=" --input.fs.dir data/txns-$trg --input.fs.ext txn "
    # This is using check out from Git, see perf.toml
	inputSelector=""
elif [ "$storage" = "git" ]; then
	inputSelector="--input.git.ref txns-$trg"
else
	echo "unkonwn storage: $storage"
	exit 1
fi



if [ -n "$filter" ]; then 
       flt="flt"
       fltOpts="--api-filter-def ${filter}"
else
       flt="all"
       fltOpts=
fi


report_file=results/hwXX/$version-perf-$flt-$storage-$trg-$report-"$(echo $formats | tr ' ' '_')".txt

(
echo "exe: $exe_path"
echo "build: $build"
echo "storage: $storage"
echo "set: $trg"
echo "version: $version"
echo "report: $report"
echo "formats: $formats"
echo "filter: $filter"
echo ""


for i in 1 2 3 4 5; do 

	rm -f out/perf-*
	/usr/bin/time -f "\nreal\t%es\nuser\t%Us\nsys\t%Ss\nmem\t%Mk (max)\ncpu\t%P" \
	"$exe_path" \
	--config perf.toml \
    --input.storage "$storage" \
	$inputSelector \
	--output.dir out \
	--output.prefix perf-$storage-$trg-$flt \
	--reports $report \
	$fltOpts
	echo
done
) > "$report_file"  2>&1

# clean up path prefix
sed -i 's@/.*perf/@perf/@' "$report_file"

