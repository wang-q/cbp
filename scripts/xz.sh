#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

# ./configure --help

# Build with the specified target architecture
if [ "$OS_TYPE" == "windows" ]; then
    ASM="zig cc -target ${TARGET_ARCH}" \
    CC="zig cc -target ${TARGET_ARCH}" \
    CXX="zig c++ -target ${TARGET_ARCH}" \
    LDFLAGS="-static" \
        ./configure \
        --prefix="${TEMP_DIR}/collect" \
        --disable-debug \
        --disable-dependency-tracking \
        --disable-silent-rules \
        --disable-nls \
        --disable-threads \
        --disable-symbol-versions \
        || exit 1
else
    ASM="zig cc -target ${TARGET_ARCH}" \
    CC="zig cc -target ${TARGET_ARCH}" \
    CXX="zig c++ -target ${TARGET_ARCH}" \
    LDFLAGS="-static" \
        ./configure \
        --prefix="${TEMP_DIR}/collect" \
        --disable-debug \
        --disable-dependency-tracking \
        --disable-silent-rules \
        --disable-nls \
        || exit 1
fi
make -j 8 || exit 1
make install || exit 1

# tree "${TEMP_DIR}/collect"
# ldd "${TEMP_DIR}/collect/bin/xz"

# Run test if requested

if [ "${RUN_TEST}" = "test" ]; then
    test_bin() {
        echo "test" > foo
        "${TEMP_DIR}/collect/bin/xz" foo
        RESULT=$("${TEMP_DIR}/collect/bin/xz" -dc foo.xz)
        [ "$RESULT" = "test" ] && echo "PASSED"
    }
    run_test test_bin
fi

# Use build_tar function from common.sh
build_tar
