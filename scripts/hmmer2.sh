#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

# Build with the specified target architecture
CC="zig cc -target ${TARGET_ARCH}" \
CFLAGS="-Wno-implicit-function-declaration" \
    ./configure \
    --enable-threads \
    --enable-lfs \
    --disable-altivec \
    || exit 1
make -j 8 || exit 1

# Get binary names from Makefile
BINS=$(make -p | grep "^all: " | sed 's/^all: //')

BINS=$(
    echo $BINS |
    tr " " "\n" |
    grep "^hmm" |
    sort | uniq
)

# Copy the built binaries to the current directory and update BINS
RENAMED_BINS=""
for BIN in $BINS; do
    NEW_NAME="${BIN}2"
    cp src/$BIN "./${NEW_NAME}"
    RENAMED_BINS="${RENAMED_BINS} ${NEW_NAME}"
done
BINS=$RENAMED_BINS

# Collect binaries and create tarball
FN_TAR="${PROJ}.${OS_TYPE}.tar.gz"
cbp collect --mode bin -o "${FN_TAR}" ${BINS} ||
    { echo "==> Error: Failed to create archive"; exit 1; }
mv "${FN_TAR}" ${BASH_DIR}/../binaries/ ||
    { echo "==> Error: Failed to move archive"; exit 1; }
