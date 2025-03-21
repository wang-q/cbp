#!/bin/bash

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Create temp directory and ensure cleanup
TEMP_DIR=$(mktemp -d)
trap 'rm -rf "$TEMP_DIR"' EXIT

echo "==> Testing tealdeer installation"

# Test tldr command
echo "-> Testing tldr command"
$(cbp prefix bin)/tldr -u
OUTPUT=$($(cbp prefix bin)/tldr cat)

if echo "$OUTPUT" | grep -q "concatenate"; then
    echo "Test PASSED"
    exit 0
else
    echo "Test FAILED"
    echo "Expected output containing 'concatenate'"
    echo "Got: $OUTPUT"
    exit 1
fi
