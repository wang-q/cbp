#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

echo "==> Testing ${PROJ} installation"

# Create test markdown file
echo "-> Creating test markdown"
cat > "test.md" << 'EOF'
# Homebrew

A package manager for humans. Cats should take a look at Tigerbrew.
EOF

# Expected HTML output
EXPECTED='<h1 id="homebrew">Homebrew</h1>
<p>A package manager for humans. Cats should take a look at
Tigerbrew.</p>'

# Convert markdown to HTML
echo "-> Testing markdown to HTML conversion"
RESULT=$($(cbp prefix bin)/pandoc -f markdown -t html5 "test.md" | tr -d '\r')
EXPECTED=$(echo "${EXPECTED}" | tr -d '\r')

assert_eq "${RESULT}" "${EXPECTED}" "Expected correct HTML output"
