#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

# ./configure --help

# Build with the specified target architecture
CC="zig cc -target ${TARGET_ARCH}" \
CXX="zig c++ -target ${TARGET_ARCH}" \
CFLAGS="-I$HOME/bin/include" \
CXXFLAGS="-I$HOME/bin/include" \
CPPFLAGS="-I$HOME/bin/include -DSQLITE_ENABLE_API_ARMOR=1 -DSQLITE_ENABLE_COLUMN_METADATA=1 -DSQLITE_ENABLE_DBSTAT_VTAB=1 -DSQLITE_ENABLE_FTS3=1 -DSQLITE_ENABLE_FTS3_PARENTHESIS=1 -DSQLITE_ENABLE_FTS5=1 -DSQLITE_ENABLE_JSON1=1 -DSQLITE_ENABLE_MEMORY_MANAGEMENT=1 -DSQLITE_ENABLE_RTREE=1 -DSQLITE_ENABLE_STAT4=1 -DSQLITE_ENABLE_UNLOCK_NOTIFY=1 -DSQLITE_MAX_VARIABLE_NUMBER=250000 -DSQLITE_USE_URI=1" \
LDFLAGS="-L$HOME/bin/lib -static" \
    ./configure \
    --prefix="${TEMP_DIR}/collect" \
    --disable-dependency-tracking \
    --disable-silent-rules \
    --disable-shared \
    --enable-static \
    --enable-readline \
    --disable-editline \
    --enable-session \
    --with-readline-cflags="-I$HOME/bin/include" \
    --with-readline-ldflags="-L$HOME/bin/lib -static -lreadline -lncursesw" \
    || exit 1
make -j 8 || exit 1
make install || exit 1

# tree "${TEMP_DIR}/collect"
# ldd "${TEMP_DIR}/collect/sqlite3"

# Use build_tar function from common.sh
build_tar
