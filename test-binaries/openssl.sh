#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

echo "==> Testing ${PROJ} installation"

# Create test file and calculate its SHA256
printf "This is a test file" > "testfile.txt"
EXPECTED="e2d0fe1585a63ec6009c8016ff8dda8b17719a637405a4e23c0ff81339148249"

# Calculate checksum using openssl
$(cbp prefix bin)/openssl dgst -sha256 -out "checksum.txt" "testfile.txt"

# Extract and compare checksum
CHECKSUM=$(cat "checksum.txt" | cut -d= -f2 | tr -d ' ')

assert_eq "${CHECKSUM}" "${EXPECTED}" "SHA256 checksum mismatch"
