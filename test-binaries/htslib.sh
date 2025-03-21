#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

echo "==> Testing ${PROJ} installation"

# Create test SAM file
echo "-> Creating test SAM file"
cat > "test.sam" << 'EOF'
@SQ	SN:chr1	LN:500
r1	0	chr1	100	0	4M	*	0	0	ATGC	ABCD
r2	0	chr1	200	0	4M	*	0	0	AATT	EFGH
EOF

# Test htsfile
echo "-> Testing htsfile"
HTSFILE_OUTPUT=$($(cbp prefix bin)/htsfile "test.sam")
assert 'echo "${HTSFILE_OUTPUT}" | grep -q "SAM"' "Expected SAM format recognition"

# Test bgzip
echo "-> Testing bgzip"
assert '$(cbp prefix bin)/bgzip -c test.sam > sam.gz' "Expected successful compression"
assert '[ -f "sam.gz" ]' "Expected compressed file to exist"

# Test tabix
echo "-> Testing tabix"
assert '$(cbp prefix bin)/tabix -p sam sam.gz' "Expected successful indexing"
assert '[ -f "sam.gz.tbi" ]' "Expected index file to exist"
