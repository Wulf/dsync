#!/bin/bash

# fail on non-0 exit codes, which makes it more obvious if a test has failed
set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

cd $SCRIPT_DIR

for D in *; do 
    if [ -d "${D}" ]; then
        echo "Running codegen and compiling '$D'"
        cd $SCRIPT_DIR/$D
        cargo run --bin dsync
        cargo run --bin example
        echo ""
    fi
done
