#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

# Build with the specified target architecture
make \
    static \
    -j 8 \
    CC="zig cc -target ${TARGET_ARCH}" \
    CXX="zig c++ -target ${TARGET_ARCH}" \
    AR="zig ar" \
    RANLIB="zig ranlib" \
    CFLAGS="-I${CBP_INCLUDE}" \
    CXXFLAGS="-Wno-unused-but-set-variable -Wno-unused-result -Wno-implicit-const-int-float-conversion" \
    BT_LIBS="-lm -lpthread ${CBP_LIB}/libz.a ${CBP_LIB}/libbz2.a ${CBP_LIB}/liblzma.a" \
    || exit 1

mv bin/bedtools.static bin/bedtools

# otool -L bin/bedtools
# ldd $TEMP_DIR/collect/bin/bedtools
eza -T bin/

# Collect binaries and create tarball
FN_TAR="${PROJ}.${OS_TYPE}.tar.gz"
cbp collect --mode bin -o "${BASH_DIR}/../binaries/${FN_TAR}" \
    bin/bedtools ||
    { echo "==> Error: Failed to create archive"; exit 1; }
