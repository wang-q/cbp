#!/bin/bash

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Create temp directory and ensure cleanup
TEMP_DIR=$(mktemp -d)
trap 'rm -rf "$TEMP_DIR"' EXIT

echo "==> Testing liblzma installation"

# Create test data
echo "-> Creating test data"
TEST_FILE="${TEMP_DIR}/data.txt"
printf '%.0s.' {1..1000} > "$TEST_FILE"
ORIGINAL_CONTENT=$(cat "$TEST_FILE")

# Test compression
echo "-> Testing compression"
xz "$TEST_FILE"
if [ -f "$TEST_FILE" ]; then
    echo "Compression failed: original file still exists"
    exit 1
fi

# Test decompression
echo "-> Testing decompression"
xz -d "${TEST_FILE}.xz"
DECOMPRESSED_CONTENT=$(cat "$TEST_FILE")

if [ "$ORIGINAL_CONTENT" = "$DECOMPRESSED_CONTENT" ]; then
    echo "Test PASSED"
    exit 0
else
    echo "Test FAILED: content mismatch"
    exit 1
fi
