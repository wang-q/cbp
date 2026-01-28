#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

# Build the project with the specified target architecture and flags

cd rush
make \
    -j 8 \
    CC="zig cc -target ${TARGET_ARCH}" \
    CFLAGS="-I${CBP_INCLUDE} -L${CBP_LIB} -pedantic -Wall -O3 -DSUPPORT_GZIP_COMPRESSED -DVERSION=\\\"v1.5\\\" -DDATE=\\\"31_Oct_2025\\\"" \
    || exit 1

mkdir -p "${TEMP_DIR}/collect/bin"
if [[ -f rush.exe ]]; then
    cp rush.exe "${TEMP_DIR}/collect/bin/"
elif [[ -f rush ]]; then
    cp rush "${TEMP_DIR}/collect/bin/"
else
    echo "==> Error: built executable not found"
    exit 1
fi

eza -T ${TEMP_DIR}/collect
ldd ${TEMP_DIR}/collect/bin/rush

# Collect binaries and create tarball
FN_TAR="${PROJ}.${OS_TYPE}.tar.gz"
cbp tar ${TEMP_DIR}/collect -o "${BASH_DIR}/../binaries/${FN_TAR}" ||
    { echo "==> Error: Failed to create archive"; exit 1; }
