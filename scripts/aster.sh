#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

# Modify the Makefile to use zig cc and specify the target architecture
sed -i.bak 's/g++/$(CXX)/g' makefile || exit 1
sed -i.bak "1i\\
CXX = zig c++ -target ${TARGET_ARCH} -fpermissive -fcommon -Wno-unqualified-std-cast-call
" makefile || exit 1

# Build the project
make astral || exit 1
make astral-pro || exit 1
make wastral || exit 1
make caster-site || exit 1
make caster-pair || exit 1
make waster-site || exit 1

# Collect binaries and create tarball
FN_TAR="${PROJ}.${OS_TYPE}.tar.gz"
cbp collect --mode bin -o "${FN_TAR}" bin/ ||
    { echo "==> Error: Failed to create archive"; exit 1; }
mv "${FN_TAR}" ${BASH_DIR}/../binaries/ ||
    { echo "==> Error: Failed to move archive"; exit 1; }
