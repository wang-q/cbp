#!/bin/bash

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Create temp directory and ensure cleanup
TEMP_DIR=$(mktemp -d)
trap 'rm -rf "$TEMP_DIR"' EXIT

echo "==> Testing bat installation"

# Create test file
echo "-> Creating test file"
cat > "${TEMP_DIR}/test.txt" << 'EOF'
Hello World
This is a test file
EOF

# Test syntax highlighting
echo "-> Testing syntax highlighting"
RESULT=$($(cbp prefix bin)/bat --color=never --style=plain "${TEMP_DIR}/test.txt" | tr -d '\r')
EXPECTED=$'Hello World\nThis is a test file'
EXPECTED=$(echo "$EXPECTED" | tr -d '\r')

if [ "$RESULT" = "$EXPECTED" ]; then
    echo "Test PASSED"
    exit 0
else
    echo "Test FAILED"
    echo "Expected:"
    echo "$EXPECTED"
    echo "Got:"
    echo "$RESULT"
    exit 1
fi
