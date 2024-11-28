#!/bin/bash

# This script is to run "dsync" generation on all test-cases

# fail on non-0 exit codes, which makes it more obvious if a test has failed
set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

cd $SCRIPT_DIR

echo "Testing Generation"
./test_generate.sh

# extra separator
echo ""

echo "Testing Compilation"
./test_compile.sh
