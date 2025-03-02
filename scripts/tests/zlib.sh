#!/bin/bash

create_and_build_test() {
    # Create test program
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

    # Compile test program
    zig cc -target ${TARGET_ARCH} \
        -I${TEMP_DIR}/collect/include \
        ${TEMP_DIR}/test.c \
        ${TEMP_DIR}/collect/lib/libz.a \
        -o ${TEMP_DIR}/test
}
