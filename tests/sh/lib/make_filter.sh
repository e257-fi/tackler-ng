#
# Tackler-NG 2017-2024
#
# SPDX-License-Identifier: Apache-2.0
#

ZoneId="UTC"
sh_tzselect=""

sh_date () {
    if [ "x$sh_tzselect" != "x" ]; then
        TZ=$ZoneId date "$@"
    else
        date "$@"
    fi
}

account_flt_body () {
    local account="$1"

    cat << EOF
    {
        "TxnFilterPostingAccount" : {
            "regex" : "$account"
        }
    }
EOF
}

tag_flt_body () {
    local tag="$1"

    cat << EOF
    {
        "TxnFilterTxnTags" : {
            "regex" : "$tag"
        }
    }
EOF
}

time_end_flt_body () {
    local end="$1"

    cat << EOF
    {
        "TxnFilterTxnTSEnd" : {
            "end" : "$end"
        }
    }
EOF
}


time_span_flt_body () {
    local begin=$1
    local end=$2

    cat << EOF
         {
             "TxnFilterTxnTSBegin" : {
                 "begin" : "$begin"
             }
         },
         $(time_end_flt_body "$end")
EOF
}

#
# 1: begin ts
# 2: end ts
time_span_filter () {
    local begin=$1
    local end=$2

    flt=$(cat << EOF | base64 --wrap=0
{
    "txnFilter" : {
        "TxnFilterAND" : {
            "txnFilters" : [
                $(time_span_flt_body $begin $end)
            ]
        }
    }
}
EOF
)
    echo "base64:$flt"
}

#
# 1: Start date
# 2: Time window "+1 month"
time_window_filter () {
    local ts1=$(sh_date --date=$1 --iso-8601=s)
    local ts2=$(sh_date --date="$ts1 $2" --iso-8601=s)

    local begin=$(echo -e "$ts1\n$ts2" | sort -n | head -n1)
    local end=$(echo   -e "$ts1\n$ts2" | sort -n | tail -n1)

    time_span_filter "$begin" "$end"
}

#
# 1: Filter to make
# 2: Filter test
make_filter () {
    local filter_body=$1
    local filter_test="$2"

    fltdef=$(cat << EOF | base64 --wrap=0
{
    "txnFilter" : $($filter_body "$filter_test")
}
EOF
)
    echo "base64:$fltdef"
}

#
# 1: Filter type to use
# 2: filtering test (e.g. regex)
# 3: start date
# 4: end date
make_filter_with_time_span () {
    local filter_body="$1"
    local filter_test="$2"
    local begin="$3"
    local end="$4"

    flt=$(cat << EOF | base64 --wrap=0
{
    "txnFilter" : {
        "TxnFilterAND" : {
            "txnFilters" : [
                $($filter_body "$filter_test"),
                $(time_span_flt_body $begin $end)
            ]
        }
    }
}
EOF
)
    echo "base64:$flt"
}

#
# 1: Filter type to use
# 2: Filtering test (e.g. regex)
# 3: start date
# 4: time window "+1 month"
make_filter_with_time_window () {
    local filter_body="$1"
    local filter_test="$2"
    local ts1=$(sh_date --date=$3 --iso-8601=s)
    local ts2=$(sh_date --date="$ts1 $4" --iso-8601=s)

    local begin=$(echo -e "$ts1\n$ts2" | sort -n | head -n1)
    local end=$(echo   -e "$ts1\n$ts2" | sort -n | tail -n1)

    make_filter_with_time_span "$filter_body" "$filter_test" "$begin" "$end"
}


# fltdef=$(time_span_filter $startts $endts)
# opts="--reporting.reports balance --api-filter-def $fltdef"
#
#
# fltdef=$(time_window_filter "$1" "$2")
# opts="--reporting.reports balance --api-filter-def $fltdef"
#
# fltdef=$(make_filter "account_flt_body" "$account")
# opts="--api-filter-def $fltdef"
#
# fltdef=$(make_filter "tag_flt_body" "$test")
# opts="--api-filter-def $fltdef"
#
# fltdef=$(make_filter_with_time_window "tag_flt_body" "$test" "$tsMonth" "1 month")
# opts="--api-filter-def $fltdef"
#
# fltdef=$(make_filter_with_time_span "tag_flt_body" "$test" "$startts" "$endts")
# opts="--api-filter-def $fltdef"
