#!/bin/bash

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Create temp directory and ensure cleanup
TEMP_DIR=$(mktemp -d)
trap 'rm -rf "$TEMP_DIR"' EXIT

echo "==> Testing samtools installation"

# Create test FASTA file
echo "-> Creating test FASTA file"
cat > "${TEMP_DIR}/test.fasta" << 'EOF'
>U00096.2:1-70
AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC
EOF

# Test faidx
echo "-> Testing faidx indexing"
cd "${TEMP_DIR}"
samtools faidx test.fasta

# Check index content
EXPECTED="U00096.2:1-70	70	15	70	71"
RESULT=$(cat test.fasta.fai)

if [ "$RESULT" = "$EXPECTED" ]; then
    echo "Test PASSED"
    exit 0
else
    echo "Test FAILED"
    echo "Expected: $EXPECTED"
    echo "Got: $RESULT"
    exit 1
fi
