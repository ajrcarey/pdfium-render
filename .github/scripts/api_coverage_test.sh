#!/bin/bash

get_api_listing() {
    # $1 contains the term to search for

    grep -E -r -i "${1}\S+\(" include/pdfium_future/*.h
}

check_api_coverage() {
    # $1 contains the list of candidate functions to check
    # $2 contains the function prefix to search for

    prefix="(${2}.+)\("

    while IFS= read -r line ; do
        fn=""

        if [[ $line != *"// "* ]] ; then
            IFS=" " read -r -a tokens <<< $line

            for token in ${tokens[@]} ; do
                if [[ $token =~ $prefix ]] ; then
                    fn=${BASH_REMATCH[1]}
                fi
            done

            if [[ $fn != "" ]] ; then
                # echo "Checking bindings contains $fn"

                if [[ $(grep "fn $fn(" src/bindings.rs | wc -l) == 0 ]]; then
                    echo "$fn missing from bindings"

                    let "missing_count++"
                fi

                let "api_count++"
            fi
        fi
    done <<< "$1"
}

api_count=0
missing_count=0

fpdf_candidates=$(get_api_listing "FPDF")
check_api_coverage "$fpdf_candidates" "FPDF"

form_candidates=$(get_api_listing "FORM_Get")
check_api_coverage "$form_candidates" "FORM_"

echo "$api_count total functions in Pdfium API, $missing_count missing from bindings"

exit $missing_count # Any non-zero value indicates failure