#!/bin/bash

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

cd $SCRIPT_DIR

for D in *; do 
    if [ -d "${D}" ]; then
        echo "Testing $D"
        ${D}/test.sh
        # output separator
        echo ""
    fi
done
