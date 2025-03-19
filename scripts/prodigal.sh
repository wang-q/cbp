#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

# Build the project with the specified target architecture and flags
make \
    -j 8 \
    CC="zig cc -target ${TARGET_ARCH}" \
    CFLAGS="-I${CBP_INCLUDE} -pedantic -Wall -O3 -DSUPPORT_GZIP_COMPRESSED" \
    LDFLAGS="-L${CBP_LIB}" \
    || exit 1

make install INSTALLDIR=${TEMP_DIR}/collect/bin

eza -T ${TEMP_DIR}/collect
ldd ${TEMP_DIR}/collect/bin/prodigal

# Collect binaries and create tarball
build_tar
