#!/bin/bash

set -euo pipefail
# shellcheck disable=SC2034
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Create temp directory and ensure cleanup
TEMP_DIR=$(mktemp -d)
trap 'rm -rf "$TEMP_DIR"' EXIT

cd "$TEMP_DIR"

echo "==> Testing tectonic installation"

# Test 1: Version check
echo "-> Checking version"
tectonic --version || {
    echo "FAILED: Version check failed"
    exit 1
}

# Test 2: Basic compilation
echo "-> Testing basic compilation"
cat > test.tex << 'EOF'
\documentclass{article}
\begin{document}
Hello, World!
\end{document}
EOF

tectonic test.tex || {
    echo "FAILED: Basic compilation failed"
    exit 1
}

# Test 3: Check output file
if [ -f "test.pdf" ]; then
    echo "PASSED: PDF file generated successfully"
else
    echo "FAILED: PDF file not found"
    exit 1
fi

echo "==> All tests passed"
