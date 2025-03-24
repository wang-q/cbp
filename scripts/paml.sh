#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

cd src

# Build the project with the specified target architecture and flags
make \
    -j 8 \
    CC="zig cc -target ${TARGET_ARCH}" \
    AR="zig ar" \
    || exit 1

# Get binary names from Makefile
BINS=$(make -p | grep "^all: " | sed 's/^all: //')

# Create collect directory and copy binaries
mkdir -p ${TEMP_DIR}/collect/paml
mkdir -p ${TEMP_DIR}/collect/bin
cp ${BINS} ${TEMP_DIR}/collect/bin/

cd ..

cp -R dat/ ${TEMP_DIR}/collect/paml/
cp -R examples/ ${TEMP_DIR}/collect/paml/

cd ${TEMP_DIR}

# Collect binaries and create tarball
FN_TAR="${PROJ}.${OS_TYPE}.tar.gz"
cbp collect -o "${FN_TAR}" collect ||
    { echo "==> Error: Failed to create archive"; exit 1; }
mv "${FN_TAR}" ${BASH_DIR}/../binaries/ ||
    { echo "==> Error: Failed to move archive"; exit 1; }
