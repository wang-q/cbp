#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

# ./configure --help

# Build with the specified target architecture
./configure \
    --prefix="${TEMP_DIR}/collect" \
    || exit 1

make || exit 1
make install || exit 1

# Rename binary
mv ${TEMP_DIR}/collect/bin/sql ${TEMP_DIR}/collect/bin/parallel-sql

# # ldd ${TEMP_DIR}/collect/bin/pv
# eza -T ${TEMP_DIR}/collect/

# Collect binaries and create tarball
FN_TAR="${PROJ}.${OS_TYPE}.tar.gz"
cd $TEMP_DIR/collect
cbp collect --shebang . -o "${BASH_DIR}/../binaries/${FN_TAR}" ||
    { echo "==> Error: Failed to create archive"; exit 1; }
