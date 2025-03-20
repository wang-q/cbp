#!/bin/bash

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Create temp directory and ensure cleanup
TEMP_DIR=$(mktemp -d)
trap 'rm -rf "$TEMP_DIR"' EXIT

echo "==> Testing PHYLIP installation"

# Create test input file
echo "-> Creating test input file"
cd "${TEMP_DIR}"
cat > infile << 'EOF'
7         6
Alpha1    110110
Alpha2    110110
Beta1     110000
Beta2     110000
Gamma1    100110
Delta     001001
Epsilon   001110
EOF

# Test pars
echo "-> Testing pars"
EXPECTED="(((Epsilon:0.00,Delta:3.00):2.00,Gamma1:0.00):1.00,(Beta2:0.00,Beta1:0.00):2.00,Alpha2:0.00,Alpha1:0.00);"
echo "Y" | $(cbp prefix bin)/pars > /dev/null

if [ ! -f "outtree" ]; then
    echo "Test FAILED: outtree file not found"
    exit 1
fi

RESULT=$(cat outtree)
if [ "$RESULT" = "$EXPECTED" ]; then
    echo "Test PASSED"
    exit 0
else
    echo "Test FAILED"
    echo "Expected: $EXPECTED"
    echo "Got: $RESULT"
    exit 1
fi
