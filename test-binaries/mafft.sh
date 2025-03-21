#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

echo "==> Testing ${PROJ} installation"

# Test version output
test_version "mafft" "v[0-9]+\." "--version"

# Create test sequences
echo "-> Creating test sequences"
cat > "test.fa" << 'EOF'
>1
A
>2
A
EOF

# Test alignment
echo "-> Testing basic alignment"
MAFFT_OUTPUT=$($(cbp prefix bin)/mafft --auto test.fa)
echo "${MAFFT_OUTPUT}"
assert 'echo "${MAFFT_OUTPUT}" | grep -q ">1"' "Expected sequence header >1 in output"
assert 'echo "${MAFFT_OUTPUT}" | grep -q ">2"' "Expected sequence header >2 in output"
assert 'echo "${MAFFT_OUTPUT}" | grep -q "^[aA]$"' "Expected aligned sequence in output"
