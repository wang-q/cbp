#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

echo "==> Testing ${PROJ} installation"

cbp install dazzdb

# Create test FASTA file
echo "-> Creating test FASTA file"
cat > test.fasta << 'EOF'
>read/1/0_72
ACGTACGTACGTACGTACGTACGTACGTACGTACGTACGTACGTACGTACGTACGTACGTACGTACGTACGT
>read/2/0_72
ACGTACGTACGTACGTACGTACGTACGTACGTACGTACGTACGTACGTACGTACGTACGTACGTACGTACGT
>read/3/0_72
TGCATGCATGCATGCATGCATGCATGCATGCATGCATGCATGCATGCATGCATGCATGCATGCATGCATGCA
EOF

# Create database
echo "-> Creating DAZZ_DB database"
$(cbp prefix bin)/fasta2DB test test.fasta

# Run daligner
echo "-> Testing daligner"
OUTPUT=$($(cbp prefix bin)/daligner -v test test 2>&1)

assert 'echo "${OUTPUT}" | grep -q "Comparing"' "DALIGNER failed to start comparison"
