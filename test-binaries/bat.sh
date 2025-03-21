#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

echo "==> Testing ${PROJ} installation"

# Create test file
echo "-> Creating test file"
cat > "test.txt" << 'EOF'
Hello World
This is a test file
EOF

# Test syntax highlighting
echo "-> Testing syntax highlighting"
RESULT=$($(cbp prefix bin)/bat --color=never --style=plain "test.txt" | tr -d '\r')
EXPECTED=$'Hello World\nThis is a test file'
EXPECTED=$(echo "${EXPECTED}" | tr -d '\r')

assert_eq "${RESULT}" "${EXPECTED}" "Expected correct file content"
