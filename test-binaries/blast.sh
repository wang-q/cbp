#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

echo "==> Testing ${PROJ} installation"

# Create test FASTA file
echo "-> Creating test FASTA file"
cat > "test.fasta" << 'EOF'
>U00096.2:1-70
AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC
EOF

# Test BLASTN
echo "-> Testing blastn"
BLASTN_OUTPUT=$($(cbp prefix bin)/blastn -query test.fasta -subject test.fasta)
assert 'echo "${BLASTN_OUTPUT}" | grep -q "Identities = 70/70"' "Expected perfect match in BLASTN output"

# Test makeblastdb
echo "-> Testing makeblastdb"
MAKEDB_OUTPUT=$($(cbp prefix bin)/makeblastdb -in test.fasta -out testdb -dbtype nucl)
assert 'echo "${MAKEDB_OUTPUT}" | grep -q "Adding sequences from FASTA"' "Expected database creation message"

# Test blastdbcmd
echo "-> Testing blastdbcmd"
DBCMD_OUTPUT=$($(cbp prefix bin)/blastdbcmd -info -db testdb)
assert 'echo "${DBCMD_OUTPUT}" | grep -q "Database: test"' "Expected database info output"
