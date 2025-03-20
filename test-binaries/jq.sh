#!/bin/bash

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Create temp directory and ensure cleanup
TEMP_DIR=$(mktemp -d)
trap 'rm -rf "$TEMP_DIR"' EXIT

echo "==> Testing jq installation"

# Create test JSON file
echo "-> Creating test JSON"
echo '{"foo":1, "bar":2}' > "${TEMP_DIR}/test.json"

# Test JSON query
echo "-> Testing JSON query"
RESULT=$(jq .bar "${TEMP_DIR}/test.json")
EXPECTED="2"

if [ "$RESULT" = "$EXPECTED" ]; then
    echo "Test PASSED"
    exit 0
else
    echo "Test FAILED"
    echo "Expected: $EXPECTED"
    echo "Got: $RESULT"
    exit 1
fi
