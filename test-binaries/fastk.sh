#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

echo "==> Testing ${PROJ} installation"

# Create test FASTA file
echo "-> Creating test FASTA file"
cat > test.fasta << 'EOF'
>read1
ACGTACGTACGTACGTACGTACGT
>read2
ACGTACGTACGTACGTACGTACGT
>read3
TGCATGCATGCATGCATGCATGCA
EOF

# Test FastK
echo "-> Testing FastK"
OUTPUT=$($(cbp prefix bin)/FastK -v -k12 -t1 test.fasta 2>&1)
assert 'echo "${OUTPUT}" | grep -q "Phase 1:"' "FastK failed to start analysis"
assert '[ -f "test.ktab" ]' "FastK failed to generate profile"
