#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

cd src

# Build the project with the specified target architecture and flags
make \
    -j 8 \
    -f Makefile.unx \
    all \
    CC="zig cc -target ${TARGET_ARCH}" \
    CFLAGS="-g -Wno-implicit-function-declaration -fcommon" \
    || exit 1

make \
    -j 8 \
    -f Makefile.unx \
    put \
    EXEDIR="${TEMP_DIR}/collect/bin"

cd "${TEMP_DIR}/collect/bin"

rm font*
if [[ "${OS_TYPE}" == "macos" ]]; then
    for f in *.so; do
        mv -v "$f" "${f%.so}.dylib"
    done
fi

cd ${TEMP_DIR}

# Collect binaries and create tarball
FN_TAR="${PROJ}.${OS_TYPE}.tar.gz"
cbp collect -o "${FN_TAR}" collect ||
    { echo "==> Error: Failed to create archive"; exit 1; }
mv "${FN_TAR}" ${BASH_DIR}/../binaries/ ||
    { echo "==> Error: Failed to move archive"; exit 1; }
