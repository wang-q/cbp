#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

# ./configure --help

# Set configure options based on OS type
if [ "$OS_TYPE" == "windows" ]; then
    EXTRA_OPT="--disable-symbol-versions"
fi

# Build with the specified target architecture
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
    ${EXTRA_OPT} \
    || exit 1

make -j 8 || exit 1
make install || exit 1

# eza -T "${TEMP_DIR}/collect"
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
