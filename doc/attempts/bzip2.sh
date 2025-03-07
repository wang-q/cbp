#!/bin/bash

# Source common build environment: extract source, setup dirs and functions
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Extract source code
extract_source

# Set make options based on OS type
if [ "$OS_TYPE" == "windows" ]; then
    CC="gcc"
    # Modify Makefile to force .o extension for object files
    # Replace '$(CFLAGS) -c' with '$(CFLAGS) -c -o $*.o'
    perl -pi -e 's{\$\(CFLAGS\) -c}{\$(CFLAGS) -c -o \$*.o}g' Makefile
else
    CC="zig cc -target ${TARGET_ARCH}"
fi

# Build with the specified target architecture
make \
    CC="${CC}" \
    || exit 1

# # Install to collect directory
# make install PREFIX="${TEMP_DIR}/collect"

# Custom install
mkdir -p "${TEMP_DIR}/collect/"{bin,lib,include}
cp bzip2 "${TEMP_DIR}/collect/bin/bzip2${BIN_SUFFIX}"
cp bzip2recover "${TEMP_DIR}/collect/bin/bzip2recover${BIN_SUFFIX}"
cp bzlib.h "${TEMP_DIR}/collect/include/"
cp libbz2.a "${TEMP_DIR}/collect/lib/"

eza -T ${TEMP_DIR}/collect/

# Run test if requested
if [ "${RUN_TEST}" = "test" ]; then
    source "${BASH_DIR}/tests/bzip2.sh"
    create_and_build_test
    run_test "${TEMP_DIR}/test" "bzip2"
fi

# Create package
build_tar
