#!/bin/bash

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Create temp directory and ensure cleanup
TEMP_DIR=$(mktemp -d)
trap 'rm -rf "$TEMP_DIR"' EXIT

echo "==> Testing muscle installation"

# Test version output
echo "-> Testing version output"
VERSION_OUTPUT=$(muscle -version)

if echo "$VERSION_OUTPUT" | grep -q "muscle"; then
    echo "Test PASSED"
    exit 0
else
    echo "Test FAILED"
    echo "Expected 'muscle' in version output"
    echo "Got: $VERSION_OUTPUT"
    exit 1
fi
