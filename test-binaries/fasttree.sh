#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

echo "==> Testing ${PROJ} installation"

# Create test FASTA file
echo "-> Creating test FASTA file"
cat > test.fa << 'EOF'
>1
LCLYTHIGRNIYYGSYLYSETWNTTTMLLLITMATAFMGYVLPWGQMSFWGATVITNLFSAIPYIGTNLV
>2
LCLYTHIGRNIYYGSYLYSETWNTGIMLLLITMATAFMGYVLPWGQMSFWGATVITNLFSAIPYIGTNLV
>3
LCLYTHIGRNIYYGSYLYSETWNTGIMLLLITMATAFMGTTLPWGQMSFWGATVITNLFSAIPYIGTNLV
EOF

# Test tree construction
echo "-> Testing tree construction"
OUTPUT=$($(cbp prefix bin)/FastTree test.fa 2> /dev/null)

assert 'echo "${OUTPUT}" | grep -E "1:0\.[0-9]+,2:0\.[0-9]+,3:0\.[0-9]+"' \
    "FastTree failed to generate expected tree format"
