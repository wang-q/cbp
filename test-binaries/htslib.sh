#!/bin/bash

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Create temp directory and ensure cleanup
TEMP_DIR=$(mktemp -d)
trap 'rm -rf "$TEMP_DIR"' EXIT

echo "==> Testing htslib installation"

# Create test SAM file
echo "-> Creating test SAM file"
cat > "${TEMP_DIR}/test.sam" << 'EOF'
@SQ	SN:chr1	LN:500
r1	0	chr1	100	0	4M	*	0	0	ATGC	ABCD
r2	0	chr1	200	0	4M	*	0	0	AATT	EFGH
EOF

# Test htsfile
echo "-> Testing htsfile"
HTSFILE_OUTPUT=$($(cbp prefix bin)/htsfile "${TEMP_DIR}/test.sam")
if ! echo "$HTSFILE_OUTPUT" | grep -q "SAM"; then
    echo "htsfile test FAILED"
    exit 1
fi

# Test bgzip
echo "-> Testing bgzip"
cd "${TEMP_DIR}"
if ! $(cbp prefix bin)/bgzip -c test.sam > sam.gz; then
    echo "bgzip test FAILED"
    exit 1
fi

if [ ! -f "sam.gz" ]; then
    echo "bgzip output file not found"
    exit 1
fi

# Test tabix
echo "-> Testing tabix"
if ! $(cbp prefix bin)/tabix -p sam sam.gz; then
    echo "tabix test FAILED"
    exit 1
fi

if [ ! -f "sam.gz.tbi" ]; then
    echo "tabix index file not found"
    exit 1
fi

echo "Test PASSED"
exit 0
