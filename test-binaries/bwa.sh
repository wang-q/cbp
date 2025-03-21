#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

echo "==> Testing ${PROJ} installation"

# Create test FASTA file
echo "-> Creating test FASTA file"
cat > "test.fasta" << 'EOF'
>0
AGATGTGCTG
EOF

# Test bwa index
echo "-> Testing bwa index"
$(cbp prefix bin)/bwa index test.fasta
assert '[ -f "test.fasta.bwt" ]' "Expected index file test.fasta.bwt to exist"

# Test bwa mem
echo "-> Testing bwa mem"
BWA_OUTPUT=$($(cbp prefix bin)/bwa mem test.fasta test.fasta)
assert 'echo "${BWA_OUTPUT}" | grep -q "AGATGTGCTG"' "Expected sequence in BWA output"
