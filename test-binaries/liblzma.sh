#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

echo "==> Testing ${PROJ} installation"

# Create test data
echo "-> Creating test data"
TEST_FILE="data.txt"
printf '%.0s.' {1..1000} > "${TEST_FILE}"
ORIGINAL_CONTENT=$(cat "${TEST_FILE}")

# Test compression
echo "-> Testing compression"
$(cbp prefix bin)/xz "${TEST_FILE}"
assert '[ ! -f "${TEST_FILE}" ]' "Expected original file to be replaced by compressed file"

# Test decompression
echo "-> Testing decompression"
$(cbp prefix bin)/xz -d "${TEST_FILE}.xz"
DECOMPRESSED_CONTENT=$(cat "${TEST_FILE}")

assert_eq "${ORIGINAL_CONTENT}" "${DECOMPRESSED_CONTENT}" "Expected content to match after compression/decompression"
