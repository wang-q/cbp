#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

echo "==> Testing ${PROJ} installation"

# Create test input file
echo "-> Creating test input file"
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

assert '[ -f "outtree" ]' "outtree file not found"
RESULT=$(cat outtree)
assert_eq "${RESULT}" "${EXPECTED}" "Tree output mismatch"
