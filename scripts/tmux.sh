#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

# ./configure --help

# Build with the specified target architecture

# need getrandom, can't use -target
CC="zig cc" \
AR="zig ar" \
RANLIB="zig ranlib" \
CFLAGS="-I${CBP_INCLUDE} -Wno-macro-redefined -Wno-implicit-function-declaration" \
CPPFLAGS="-I${CBP_INCLUDE} -Wno-macro-redefined -Wno-implicit-function-declaration" \
LDFLAGS="-L${CBP_LIB}" \
PKG_CONFIG_LIBDIR="${CBP_LIB}/pkgconfig" \
    ./configure \
    --prefix="${TEMP_DIR}/collect" \
    --disable-silent-rules \
    --enable-sixel \
    --enable-utf8proc \
    || exit 1

make || exit 1
make install || exit 1

mkdir -p ${TEMP_DIR}/collect/share/tmux
cp example_tmux.conf ${TEMP_DIR}/collect/share/tmux/

# ldd ${TEMP_DIR}/collect/bin/tmux
# otool -L ${TEMP_DIR}/collect/bin/tmux
eza -T ${TEMP_DIR}/collect/

# Collect binaries and create tarball
FN_TAR="${PROJ}.${OS_TYPE}.tar.gz"
cbp tar ${TEMP_DIR}/collect -o "${BASH_DIR}/../binaries/${FN_TAR}" ||
    { echo "==> Error: Failed to create archive"; exit 1; }
