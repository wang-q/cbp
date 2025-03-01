#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

# ./configure --help

# Build with the specified target architecture
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
make -j 8 || exit 1
make install || exit 1

# tree "${TEMP_DIR}/collect"
# ldd "${TEMP_DIR}/collect/bin/xz"
# Run test if requested

if [ "${RUN_TEST}" = "test" ]; then
    cd "${TEMP_DIR}"
    echo "test" > foo
    "${TEMP_DIR}/collect/bin/xz" foo
    RESULT=$("${TEMP_DIR}/collect/bin/xz" -dc foo.xz)
    if [ "$RESULT" = "test" ]; then
        echo "==> Test PASSED"
    else
        echo "==> Test FAILED"
        echo "Expected: test"
        echo "Got: $RESULT"
        exit 1
    fi
else
    echo "==> Skipping tests (use 'test' as second argument to enable)"
fi

# Use build_tar function from common.sh
# build_tar
build_tar
