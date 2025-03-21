#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

echo "==> Testing ${PROJ} installation"

# Create test program
echo "-> Creating test program"
cat > test.c << 'EOF'
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <bzlib.h>

int main() {
    const char *test_string = "Hello, bzip2!";
    unsigned int len = strlen(test_string) + 1;
    unsigned int compressed_size = len + 100;  // Add some buffer
    unsigned int decompressed_size = len;

    char *compressed = (char*)malloc(compressed_size);
    char *decompressed = (char*)malloc(decompressed_size);

    // Compress
    int status = BZ2_bzBuffToBuffCompress(
        compressed, &compressed_size,
        (char*)test_string, len,
        9, 0, 0);

    if (status != BZ_OK) {
        printf("Compression failed with error code %d\n", status);
        return 1;
    }

    // Decompress
    status = BZ2_bzBuffToBuffDecompress(
        decompressed, &decompressed_size,
        compressed, compressed_size,
        0, 0);

    if (status != BZ_OK) {
        printf("Decompression failed with error code %d\n", status);
        return 1;
    }

    printf("Original: %s\n", test_string);
    printf("Decompressed: %s\n", decompressed);
    printf("Test %s\n", strcmp(test_string, decompressed) == 0 ? "PASSED" : "FAILED");

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
        "$(cbp prefix lib)/libbz2.a" \
        -o test.exe
    OUTPUT=$(./test.exe)
else
    gcc \
        -I"$(cbp prefix include)" \
        test.c \
        "$(cbp prefix lib)/libbz2.a" \
        -o test
    OUTPUT=$(./test)
fi

assert 'echo "${OUTPUT}" | grep -q "Test PASSED"' "Expected successful compression/decompression test"
