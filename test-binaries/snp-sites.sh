#!/bin/bash

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Create temp directory and ensure cleanup
TEMP_DIR=$(mktemp -d)
trap 'rm -rf "$TEMP_DIR"' EXIT

echo "==> Testing snp-sites installation"

# Test version output
echo "-> Testing version output"
VERSION_OUTPUT=$($(cbp prefix bin)/snp-sites -V 2>&1)

if [ -n "$VERSION_OUTPUT" ] && [[ "$VERSION_OUTPUT" =~ ^snp-sites\ [0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo "Test PASSED"
    exit 0
else
    echo "Test FAILED"
    echo "Expected output in format 'snp-sites X.Y.Z'"
    echo "Got: $VERSION_OUTPUT"
    exit 1
fi
