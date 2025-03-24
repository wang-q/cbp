#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

# Build the project with the specified target architecture and flags
sed -i 's/# vecLib on/ifdef NOTSKIPIT\n# vecLib on/g' src/make-include.mk || exit 1
sed -i 's/# bypassed altogether/endif/g' src/make-include.mk || exit 1

cd src
make \
    CC="zig cc -target ${TARGET_ARCH}" \
    || exit 1

# Create compressed archive
mkdir -p ${TEMP_DIR}/collect
mv ../bin ${TEMP_DIR}/collect/

cd ${TEMP_DIR}

# Collect binaries and create tarball
FN_TAR="${PROJ}.${OS_TYPE}.tar.gz"
cbp collect -o "${FN_TAR}" collect ||
    { echo "==> Error: Failed to create archive"; exit 1; }
mv "${FN_TAR}" ${BASH_DIR}/../binaries/ ||
    { echo "==> Error: Failed to move archive"; exit 1; }
