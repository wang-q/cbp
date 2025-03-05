#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

# Set make options based on OS type
if [ "$OS_TYPE" == "windows" ]; then
    # Force output files to use .o extension instead of Windows default .obj
    # This is necessary because the Makefile expects .o files
    CC_EXTRA="-o \$*.o"
fi

# Build with the specified target architecture
make \
    CC="zig cc -target ${TARGET_ARCH} ${CC_EXTRA}" \
    AR="zig ar" \
    RANLIB="zig ranlib" \
    || exit 1

# # Install to collect directory
# make install PREFIX="${TEMP_DIR}/collect"

# Custom install
mkdir -p "${TEMP_DIR}/collect/"{bin,lib,include}
cp bzip2 "${TEMP_DIR}/collect/bin/bzip2${BIN_SUFFIX}"
cp bzip2recover "${TEMP_DIR}/collect/bin/bzip2recover${BIN_SUFFIX}"
cp bzlib.h "${TEMP_DIR}/collect/include/"
cp libbz2.a "${TEMP_DIR}/collect/lib/"

# eza -T ${TEMP_DIR}/collect/

# Useless
# # Create symlinks
# cd "${TEMP_DIR}/collect/bin"
# ln -sf bzdiff bzcmp
# ln -sf bzgrep bzegrep
# ln -sf bzgrep bzfgrep
# ln -sf bzmore bzless

# Run test if requested
if [ "${RUN_TEST}" = "test" ]; then
    source "${BASH_DIR}/tests/bzip2.sh"
    create_and_build_test
    run_test "${TEMP_DIR}/test" "bzip2"
fi

# Create package
build_tar
