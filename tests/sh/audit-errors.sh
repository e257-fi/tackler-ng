#
# Tackler-NG 2025
# SPDX-License-Identifier: Apache-2.0
#

set -eu

###
### AUDIT: ERRORS
###
module=audit
mode="error"

#####################################################################
#
# test: 24f154c7-9681-497b-a28c-c90cfb0533a5
# desc: cli: error propagation: missing UUID
test_name=duplicate-uuid
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --config $SUITE_PATH/$module/audit.toml \
    --input.git.repository $SUITE_PATH/$module/audit-repo.git \
    --input.git.ref errs-1E2 \
    --input.git.dir txns/2016/04 \
    2>&1 | grep 'Semantic error: Audit .* without UUID'

echo "check: ok"

#####################################################################
#
# test: 78f82210-7e9b-4741-9a0f-cc69636c70c4
# desc: cli: error propagation: duplicate UUID
test_name=duplicate-uuid
echo "test: $module/$test_name: $mode"

$TACKLER_SH \
    --config $SUITE_PATH/$module/audit.toml \
    --input.git.repository $SUITE_PATH/$module/audit-repo.git \
    --input.git.ref errs-1E2 \
    --input.git.dir txns/2016/05 \
    2>&1 | grep 'f1e3d709-fab4-380b-bd36-fd671f7b0299'

echo "check: ok"
