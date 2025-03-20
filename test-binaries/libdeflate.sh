#!/bin/bash

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Create temp directory and ensure cleanup
TEMP_DIR=$(mktemp -d)
trap 'rm -rf "$TEMP_DIR"' EXIT

echo "==> Testing libdeflate installation"

# Basic compression test
echo "-> Running compression test"
echo "test" > "${TEMP_DIR}/foo"

libdeflate-gzip "${TEMP_DIR}/foo" || {
    echo "Compression failed"
    exit 1
}

RESULT=$(libdeflate-gunzip -dc "${TEMP_DIR}/foo.gz") || {
    echo "Decompression failed"
    exit 1
}

if [ "$RESULT" = "test" ]; then
    echo "Basic compression test: PASSED"
    exit 0
else
    echo "Basic compression test: FAILED"
    exit 1
fi
