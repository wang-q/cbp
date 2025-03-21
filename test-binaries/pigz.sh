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
$(cbp prefix bin)/pigz -p 2 "${TEST_FILE}"
assert '[ ! -f "${TEST_FILE}" ]' "Compression failed: original file still exists"

# Test decompression
echo "-> Testing decompression"
$(cbp prefix bin)/pigz -p 2 -d "${TEST_FILE}.gz"
DECOMPRESSED_CONTENT=$(cat "${TEST_FILE}")

assert_eq "${ORIGINAL_CONTENT}" "${DECOMPRESSED_CONTENT}" "Content mismatch after compression/decompression"
