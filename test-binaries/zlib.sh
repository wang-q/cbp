#!/bin/bash

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Create temp directory and ensure cleanup
TEMP_DIR=$(mktemp -d)
trap 'rm -rf "$TEMP_DIR"' EXIT

echo "==> Testing zlib installation"

# Create test program
echo "-> Creating test program"
cat > ${TEMP_DIR}/test.c << 'EOF'
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
        ${TEMP_DIR}/test.c \
        "$(cbp prefix lib)/libz.a" \
        -o ${TEMP_DIR}/test.exe
    ${TEMP_DIR}/test.exe
else
    gcc \
        -I"$(cbp prefix include)" \
        ${TEMP_DIR}/test.c \
        "$(cbp prefix lib)/libz.a" \
        -o ${TEMP_DIR}/test
    ${TEMP_DIR}/test
fi
