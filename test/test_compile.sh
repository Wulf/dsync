#!/bin/bash

# This script is to compile all compile-able tests to see if they are correct

# fail on non-0 exit codes, which makes it more obvious if a test has failed
set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

cd $SCRIPT_DIR

cargo build --all
