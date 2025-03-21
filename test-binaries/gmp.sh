#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

echo "==> Testing ${PROJ} installation"

# Create test program
echo "-> Creating test program"
cat > test.c << 'EOF'
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
        test.c \
        "$(cbp prefix lib)/libgmp.a" \
        -o test.exe
    OUTPUT=$(./test.exe)
else
    gcc \
        -I"$(cbp prefix include)" \
        test.c \
        "$(cbp prefix lib)/libgmp.a" \
        -o test
    OUTPUT=$(./test)
fi

assert_eq "${OUTPUT}" "Test PASSED" "Expected successful GMP test"
