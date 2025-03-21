#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

echo "==> Testing ${PROJ} installation"

# Create test program
echo "-> Creating test program"
cat > test.c << 'EOF'
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <zlib.h>

int main() {
    const char *version = zlibVersion();
    printf("zlib version: %s\n", version);

    const char *test_string = "Hello, zlib!";
    uLong len = strlen(test_string) + 1;
    uLong clen = compressBound(len);
    unsigned char *compressed = (unsigned char*)malloc(clen);
    unsigned char *decompressed = (unsigned char*)malloc(len);

    // Compress
    compress(compressed, &clen, (const unsigned char*)test_string, len);

    // Decompress
    uLong dlen = len;
    uncompress(decompressed, &dlen, compressed, clen);

    printf("Original: %s\n", test_string);
    printf("Decompressed: %s\n", (char*)decompressed);
    printf("Test %s\n", strcmp(test_string, (char*)decompressed) == 0 ? "PASSED" : "FAILED");

    free(compressed);
    free(decompressed);
    return 0;
}
EOF

# Compile and run test
echo "-> Compiling test program"
if [[ "${OSTYPE:-}" == "msys" || "${OSTYPE:-}" == "win32" || "${OS:-}" == "Windows_NT" ]]; then
    gcc \
        -I"$(cbp prefix include)" \
        test.c \
        "$(cbp prefix lib)/libz.a" \
        -o test.exe
    OUTPUT=$(./test.exe)
else
    gcc \
        -I"$(cbp prefix include)" \
        test.c \
        "$(cbp prefix lib)/libz.a" \
        -o test
    OUTPUT=$(./test)
fi

assert 'echo "${OUTPUT}" | grep -q "Test PASSED"' "Expected successful compression/decompression test"
