#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

echo "==> Testing ${PROJ} installation"

# Create test sequence file
echo "-> Creating test sequence file"
cat > test.fa << 'EOF'
>seq
aggaaacctgccatggcctcctggtgagctgtcctcatccactgctcgctgcctctccag
atactctgacccatggatcccctgggtgcagccaagccacaatggccatggcgccgctgt
actcccacccgccccaccctcctgatcctgctatggacatggcctttccacatccctgtg
EOF

# Run TRF analysis
echo "-> Running TRF analysis"
$(cbp prefix bin)/trf test.fa 2 7 7 80 10 50 500 2>/dev/null || true

# Check output file
assert '[ -f "test.fa.2.7.7.80.10.50.500.1.txt.html" ]' "TRF failed to generate output file"
assert 'grep -q "Length: 180" test.fa.2.7.7.80.10.50.500.1.txt.html' "TRF output does not contain expected content"
