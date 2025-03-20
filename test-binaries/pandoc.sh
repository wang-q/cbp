#!/bin/bash

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Create temp directory and ensure cleanup
TEMP_DIR=$(mktemp -d)
trap 'rm -rf "$TEMP_DIR"' EXIT

echo "==> Testing pandoc installation"

# Create test markdown file
echo "-> Creating test markdown"
cat > "${TEMP_DIR}/test.md" << 'EOF'
# Homebrew

A package manager for humans. Cats should take a look at Tigerbrew.
EOF

# Expected HTML output
EXPECTED='<h1 id="homebrew">Homebrew</h1>
<p>A package manager for humans. Cats should take a look at
Tigerbrew.</p>'

# Convert markdown to HTML
echo "-> Testing markdown to HTML conversion"
RESULT=$($(cbp prefix bin)/pandoc -f markdown -t html5 "${TEMP_DIR}/test.md" | tr -d '\r')
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
