#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

echo "==> Testing ${PROJ} installation"

# Create test FASTA file
echo "-> Creating test FASTA file"
cat > "test.fasta" << 'EOF'
>U00096.2:1-70
AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC
EOF

# Test faidx
echo "-> Testing faidx indexing"
$(cbp prefix bin)/samtools faidx test.fasta

# Check index content
EXPECTED="U00096.2:1-70	70	15	70	71"
RESULT=$(cat test.fasta.fai)

assert_eq "${RESULT}" "${EXPECTED}" "Index content mismatch"
