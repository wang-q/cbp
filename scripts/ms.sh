#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

zig cc -target ${TARGET_ARCH} \
    msGCHOT.c streecGCHOT.c rand1.c \
    -lm \
    -Wno-implicit-int \
    -Wno-return-mismatch \
    -Wno-implicit-function-declaration \
    -Wno-deprecated-non-prototype \
    -Wno-return-type \
    -Wno-empty-body \
    -o ms ||
    exit 1

# Check binary dependencies
ldd ms
# ./ms

# Collect binaries and create tarball
FN_TAR="${PROJ}.${OS_TYPE}.tar.gz"
cbp collect --mode bin -o "${FN_TAR}" ms ||
    { echo "==> Error: Failed to create archive"; exit 1; }
mv "${FN_TAR}" ${BASH_DIR}/../binaries/ ||
    { echo "==> Error: Failed to move archive"; exit 1; }
