#!/bin/bash

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Create temp directory and ensure cleanup
TEMP_DIR=$(mktemp -d)
trap 'rm -rf "$TEMP_DIR"' EXIT

echo "==> Testing gmp installation"

# Create test program
echo "-> Creating test program"
cat > ${TEMP_DIR}/test.c << 'EOF'
#include <gmp.h>
#include <stdlib.h>
#include <stdio.h>

int main() {
    mpz_t i, j, k;
    mpz_init_set_str (i, "1a", 16);
    mpz_init (j);
    mpz_init (k);
    mpz_sqrtrem (j, k, i);

    if (mpz_get_si (j) != 5 || mpz_get_si (k) != 1) abort();

    printf("Test %s\n", "PASSED");

    return 0;
}
EOF

# Compile and run test
echo "-> Compiling test program"
if [[ "${OSTYPE:-}" == "msys" || "${OSTYPE:-}" == "win32" || "${OS:-}" == "Windows_NT" ]]; then
    gcc \
        -I"$(cbp prefix include)" \
        ${TEMP_DIR}/test.c \
        "$(cbp prefix lib)/libgmp.a" \
        -o ${TEMP_DIR}/test.exe
    ${TEMP_DIR}/test.exe
else
    gcc \
        -I"$(cbp prefix include)" \
        ${TEMP_DIR}/test.c \
        "$(cbp prefix lib)/libgmp.a" \
        -o ${TEMP_DIR}/test
    ${TEMP_DIR}/test
fi
