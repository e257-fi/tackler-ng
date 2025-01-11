#!/bin/bash
# vim: tabstop=4 shiftwidth=4 smarttab expandtab softtabstop=4 autoindent
#
# Tackler-NG 2018-2024
#
# SPDX-License-Identifier: Apache-2.0
#############################################################################

#
# Calculate total sum of transactions with bc
#
perf=$1

((echo "0.0"; find $perf -type f -exec sed -n 's/.*e:.* \(.*\)$/ + \1/p' {} \; ) | tr -d '\n'; echo) | bc -l | tee $perf-bc.txt
