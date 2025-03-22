#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

echo "==> Testing ${PROJ} installation"

# Create test FASTA file
echo "-> Creating test FASTA file"
cat > test.fasta << 'EOF'
>read/1/0_16
ACGTACGTACGTACGT
>read/2/0_16
TGCATGCATGCATGCA
>read/3/0_16
GCTAGCTAGCTAGCTA
EOF

# Test fasta2DB
echo "-> Testing fasta2DB"
$(cbp prefix bin)/fasta2DB test test.fasta
assert '[ -f "test.db" ]' "Failed to create database file"
assert '[ -f ".test.idx" ]' "Failed to create index file"

# Test DBstats
echo "-> Testing DBstats"
OUTPUT=$($(cbp prefix bin)/DBstats test 2>&1)

assert 'echo "${OUTPUT}" | grep -q "3 reads"' "Incorrect number of reads in database"
