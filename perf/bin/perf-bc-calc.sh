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

#
# Calculate total sum of transactions with bc
#
perf=$1

((echo "0.0"; find $perf -type f -exec sed -n 's/.*e:.* \(.*\)$/ + \1/p' {} \; ) | tr -d '\n'; echo) | bc -l | tee $perf-bc.txt
