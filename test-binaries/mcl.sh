#!/bin/bash

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Create temp directory and ensure cleanup
TEMP_DIR=$(mktemp -d)
trap 'rm -rf "$TEMP_DIR"' EXIT

echo "==> Testing mcl installation"

# Test help output
echo "-> Testing help output"
HELP_OUTPUT=$(mcl -h 2>&1)

if echo "$HELP_OUTPUT" | grep -q "iterands"; then
    echo "Test PASSED"
    exit 0
else
    echo "Test FAILED"
    echo "Expected 'iterands' in help output"
    echo "Got: $HELP_OUTPUT"
    exit 1
fi
