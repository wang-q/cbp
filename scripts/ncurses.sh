#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

# Set compiler for non-macOS systems
if [ "$OS_TYPE" != "macos" ]; then
    # ../ncurses/./tinfo/lib_baudrate.c:82:10: fatal error: 'sys/ttydev.h' file not found
    export CC="zig cc -target ${TARGET_ARCH}"
    export CXX="zig c++ -target ${TARGET_ARCH}"
fi

# Common configure options
./configure \
    --prefix="${TEMP_DIR}/collect" \
    --disable-dependency-tracking \
    --disable-silent-rules \
    --disable-shared \
    --enable-static \
    --enable-sigwinch \
    --enable-widec \
    --with-shared=no \
    --with-cxx-shared=no \
    --with-gpm=no \
    --without-ada \
    --disable-termcap \
    --disable-db-install \
    --without-manpages \
    --without-tests \
    --without-progs \
    || exit 1

make -j 8 || exit 1
make install || exit 1

# eza -T "${TEMP_DIR}/collect"
# ldd "${TEMP_DIR}/collect/bin/clear"

# Use build_tar function from common.sh
build_tar
