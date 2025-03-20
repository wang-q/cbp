#!/bin/bash

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Create temp directory and ensure cleanup
TEMP_DIR=$(mktemp -d)
trap 'rm -rf "$TEMP_DIR"' EXIT

echo "==> Testing BLAST installation"

# Create test FASTA file
echo "-> Creating test FASTA file"
cat > "${TEMP_DIR}/test.fasta" << 'EOF'
>U00096.2:1-70
AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC
EOF

# Test BLASTN
echo "-> Testing blastn"
cd "${TEMP_DIR}"
BLASTN_OUTPUT=$($(cbp prefix bin)/blastn -query test.fasta -subject test.fasta)
if ! echo "$BLASTN_OUTPUT" | grep -q "Identities = 70/70"; then
    echo "BLASTN test FAILED"
    exit 1
fi

# Test makeblastdb
echo "-> Testing makeblastdb"
MAKEDB_OUTPUT=$($(cbp prefix bin)/makeblastdb -in test.fasta -out testdb -dbtype nucl)
if ! echo "$MAKEDB_OUTPUT" | grep -q "Adding sequences from FASTA"; then
    echo "makeblastdb test FAILED"
    exit 1
fi

# Test blastdbcmd
echo "-> Testing blastdbcmd"
DBCMD_OUTPUT=$($(cbp prefix bin)/blastdbcmd -info -db testdb)
if ! echo "$DBCMD_OUTPUT" | grep -q "Database: test"; then
    echo "blastdbcmd test FAILED"
    exit 1
fi

echo "Test PASSED"
exit 0
