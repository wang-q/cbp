#!/bin/bash

create_and_build_test() {
    # Create test program
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

    # Compile test program
    zig cc -target ${TARGET_ARCH} \
        ${TEMP_DIR}/test.c \
        -I${TEMP_DIR}/collect/include \
        ${TEMP_DIR}/collect/lib/libgmp.a \
        -o ${TEMP_DIR}/test
}
