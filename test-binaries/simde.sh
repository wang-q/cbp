#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

echo "==> Testing ${PROJ} installation"

# Create test program
echo "-> Creating test program"
cat > test.c << 'EOF'
#include <assert.h>
#include <simde/arm/neon.h>
#include <simde/x86/sse2.h>

int main() {
    int64_t a = 1, b = 2;
    assert(simde_vaddd_s64(a, b) == 3);
    simde__m128i z = simde_mm_setzero_si128();
    simde__m128i v = simde_mm_undefined_si128();
    v = simde_mm_xor_si128(v, v);
    assert(simde_mm_movemask_epi8(simde_mm_cmpeq_epi8(v, z)) == 0xFFFF);
    return 0;
}
EOF

# Compile and run test
echo "-> Compiling test program"
if [[ "${OSTYPE:-}" == "msys" || "${OSTYPE:-}" == "win32" || "${OS:-}" == "Windows_NT" ]]; then
    gcc \
        -I"$(cbp prefix include)" \
        test.c \
        -o test.exe
    OUTPUT=$(./test.exe)
else
    gcc \
        -I"$(cbp prefix include)" \
        test.c \
        -o test
    OUTPUT=$(./test)
fi

# Check if program ran successfully (assert will cause program to abort if failed)
assert '[ $? -eq 0 ]' "Expected SIMD emulation tests to pass"
