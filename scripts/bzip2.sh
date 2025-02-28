#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

# Build with the specified target architecture
make \
    CC="zig cc -target ${TARGET_ARCH}" \
    AR="zig ar" \
    RANLIB="zig ranlib" \
    || exit 1

# Install to collect directory
make install PREFIX="${TEMP_DIR}/collect"

# Create symlinks
cd "${TEMP_DIR}/collect/bin"
ln -sf bzdiff bzcmp
ln -sf bzgrep bzegrep
ln -sf bzgrep bzfgrep
ln -sf bzmore bzless

# Clean up and reorganize
cd "${TEMP_DIR}/collect"
mv "${TEMP_DIR}/collect/bin"/* "${TEMP_DIR}/collect/"
rm -rf "${TEMP_DIR}/collect/bin"
rm -rf "${TEMP_DIR}/collect/man"

# Run test if requested
if [ "${RUN_TEST}" = "test" ]; then
    source "${BASH_DIR}/tests/bzip2.sh"
    create_and_build_test
    run_test "${TEMP_DIR}/test" "bzip2"
else
    echo "==> Skipping tests (use 'test' as second argument to enable)"
fi

# Create package
build_tar
