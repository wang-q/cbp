#!/bin/bash

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Create temp directory and ensure cleanup
TEMP_DIR=$(mktemp -d)
trap 'rm -rf "$TEMP_DIR"' EXIT

echo "==> Testing fd installation"

cd "${TEMP_DIR}"

# Create test files
echo "-> Creating test files"
touch "foo_file"
touch "test_file"

# Test file search
echo "-> Testing file search"
RESULT=$($(cbp prefix bin)/fd test | tr -d '\r')
EXPECTED="test_file"

if [ "$RESULT" = "$EXPECTED" ]; then
    echo "Test PASSED"
    exit 0
else
    echo "Test FAILED"
    echo "Expected: $EXPECTED"
    echo "Got: $RESULT"
    exit 1
fi
