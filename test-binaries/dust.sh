#!/bin/bash

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Create temp directory and ensure cleanup
TEMP_DIR=$(mktemp -d)
trap 'rm -rf "$TEMP_DIR"' EXIT

echo "==> Testing dust installation"

cd "${TEMP_DIR}"

# Create test directory structure
echo "-> Creating test directory structure"
mkdir -p dir1/subdir

# Create test files with platform-independent method
if [[ "${OSTYPE:-}" == "msys" || "${OSTYPE:-}" == "win32" || "${OS:-}" == "Windows_NT" ]]; then
    # Windows: use fsutil to create files
    fsutil file createnew dir1/file1 1048576
    fsutil file createnew dir1/subdir/file2 2097152
else
    # Unix-like systems: use dd
    dd if=/dev/zero of=dir1/file1 bs=1M count=1
    dd if=/dev/zero of=dir1/subdir/file2 bs=2M count=1
fi

# Test dust
echo "-> Testing dust"
RESULT=$($(cbp prefix bin)/dust --no-percent-bars dir1 | grep -E "^[0-9.]+(M|K)" | wc -l)

if [ "$RESULT" -ge 2 ]; then
    echo "Test PASSED"
    exit 0
else
    echo "Test FAILED"
    echo "Expected at least 2 size entries"
    echo "Got: $RESULT"
    exit 1
fi
