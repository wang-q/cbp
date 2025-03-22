#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

echo "==> Testing ${PROJ} installation"

# Create test HTML file
echo "-> Creating test HTML file"
cat > test.html << 'EOF'
<body><p>Hello</p></body>
EOF

# Test HTML parsing
echo "-> Testing HTML parsing"
OUTPUT=$(cat test.html | $(cbp prefix bin)/pup 'p text{}')

assert_eq "${OUTPUT}" "Hello" "HTML parsing failed"
