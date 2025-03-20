#!/bin/bash

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Create temp directory and ensure cleanup
TEMP_DIR=$(mktemp -d)
trap 'rm -rf "$TEMP_DIR"' EXIT

echo "==> Testing openssl installation"

# Create test file and calculate its SHA256
printf "This is a test file" > "${TEMP_DIR}/testfile.txt"
EXPECTED="e2d0fe1585a63ec6009c8016ff8dda8b17719a637405a4e23c0ff81339148249"

# Calculate checksum using openssl
$(cbp prefix bin)/openssl dgst -sha256 -out "${TEMP_DIR}/checksum.txt" "${TEMP_DIR}/testfile.txt"

# Extract and compare checksum
CHECKSUM=$(cat "${TEMP_DIR}/checksum.txt" | cut -d= -f2 | tr -d ' ')

if [ "$CHECKSUM" = "$EXPECTED" ]; then
    echo "Test PASSED"
    exit 0
else
    echo "Test FAILED"
    echo "Expected: $EXPECTED"
    echo "Got: $CHECKSUM"
    exit 1
fi
