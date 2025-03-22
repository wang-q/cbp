#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

echo "==> Testing ${PROJ} installation"

# Create test FASTA file
echo "-> Creating test FASTA file"
cat > test.fasta << 'EOF'
>read1
ACGTACGTACGTACGTACGTACGTACGTACGTACGTACGT
>read2
ACGTACGTACGTACGTACGTACGTACGTACGTACGTACGT
>read3
TGCATGCATGCATGCATGCATGCATGCATGCATGCATGCA
>read4
TGCATGCATGCATGCATGCATGCATGCATGCATGCATGCA
EOF

# Create database
echo "-> Creating GDB database"
OUTPUT=$($(cbp prefix bin)/FAtoGDB -v test.fasta test 2>&1)
assert 'echo "${OUTPUT}" | grep -q "Creating genome data base"' "FAtoGDB failed to start"

OUTPUT=$($(cbp prefix bin)/GDBstat test 2>&1)
assert 'echo "${OUTPUT}" | grep -q "Statistics for assembly test"' "GDBstat failed to start"

OUTPUT=$($(cbp prefix bin)/FastGA -v -1:self test.fasta 2>&1)
assert 'echo "${OUTPUT}" | grep -q "Starting adaptive seed merge"' "FastGA failed to start"
assert '[ -f "self.1aln" ]' "Failed to create 1aln file"
