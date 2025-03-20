#!/bin/bash

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Create temp directory and ensure cleanup
TEMP_DIR=$(mktemp -d)
trap 'rm -rf "$TEMP_DIR"' EXIT

echo "==> Testing libdeflate installation"

# Create test program
echo "-> Creating test program"
cat > ${TEMP_DIR}/test.c << 'EOF'
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <libdeflate.h>

int main() {
    const char *test_string = "Hello, libdeflate!";
    size_t in_size = strlen(test_string) + 1;
    size_t compressed_size;
    size_t decompressed_size;

    struct libdeflate_compressor *compressor = libdeflate_alloc_compressor(6);
    struct libdeflate_decompressor *decompressor = libdeflate_alloc_decompressor();

    char *compressed = malloc(in_size * 2);  // Ensure enough space
    char *decompressed = malloc(in_size);

    // Compress
    compressed_size = libdeflate_deflate_compress(
        compressor, test_string, in_size,
        compressed, in_size * 2);

    if (compressed_size == 0) {
        printf("Compression failed\n");
        return 1;
    }

    // Decompress
    enum libdeflate_result result = libdeflate_deflate_decompress(
        decompressor, compressed, compressed_size,
        decompressed, in_size, &decompressed_size);

    if (result != LIBDEFLATE_SUCCESS) {
        printf("Decompression failed\n");
        return 1;
    }

    printf("Original: %s\n", test_string);
    printf("Decompressed: %s\n", decompressed);
    printf("Test %s\n", strcmp(test_string, decompressed) == 0 ? "PASSED" : "FAILED");

    libdeflate_free_compressor(compressor);
    libdeflate_free_decompressor(decompressor);
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
        ${TEMP_DIR}/test.c \
        "$(cbp prefix lib)/libdeflate.a" \
        -o ${TEMP_DIR}/test.exe
    ${TEMP_DIR}/test.exe
else
    gcc \
        -I"$(cbp prefix include)" \
        ${TEMP_DIR}/test.c \
        "$(cbp prefix lib)/libdeflate.a" \
        -o ${TEMP_DIR}/test
    ${TEMP_DIR}/test
fi
