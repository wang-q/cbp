#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

echo "==> Testing ${PROJ} installation"

# Create test files
echo "-> Creating test files"
touch "foo_file"
touch "test_file"

# Test file search
echo "-> Testing file search"
RESULT=$($(cbp prefix bin)/fd test | tr -d '\r')
EXPECTED="test_file"

assert_eq "${RESULT}" "${EXPECTED}" "Expected matching file name"
