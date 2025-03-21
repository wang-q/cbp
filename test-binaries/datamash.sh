#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

echo "==> Testing ${PROJ} installation"

# Test version output
test_version "datamash" "GNU datamash" "--version"

# Test basic functionality
echo "-> Testing basic operations"
echo -e "1\n2\n3" > numbers.txt
RESULT=$($(cbp prefix bin)/datamash sum 1 < numbers.txt)
assert_eq "${RESULT}" "6" "Sum of numbers should be 6"
