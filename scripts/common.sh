#!/bin/bash

# Prevent direct execution of this script
# ${BASH_SOURCE[0]} is the path of the currently executing script (common.sh)
# ${0} is the path used to call the script
# If they are equal, the script is being executed directly instead of being sourced
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    echo "Error: This script should be sourced, not executed directly"
    echo "Usage: source ${BASH_SOURCE[0]}"
    echo
    echo "This script defines the following variables:"
    echo "  BASH_DIR    - Directory of the calling script"
    echo "  PROJ        - Name of the calling script (without .sh)"
    echo "  OS_TYPE     - Operating system type (linux or macos)"
    echo "  TARGET_ARCH - Target architecture for compilation"
    echo "  TEMP_DIR    - Temporary working directory"
    exit 1
fi

# Get the directory of the script and project name
# ${BASH_SOURCE[1]} is the path of the script that sourced this file
# When common.sh is sourced by another script (e.g., zlib.sh),
# ${BASH_SOURCE[1]} refers to zlib.sh's path
BASH_DIR=$( cd "$( dirname "${BASH_SOURCE[1]}" )" && pwd )
PROJ=$(basename "${BASH_SOURCE[1]}" .sh)

# Set the OS type based on the system or command line argument
if [ "$(uname -s)" = "Darwin" ]; then
    OS_TYPE=${1:-macos}
else
    OS_TYPE=${1:-linux}
fi
# Set test mode based on command line argument
RUN_TEST=${2:-""}

# Validate the OS type
if [[ "$OS_TYPE" != "linux" && "$OS_TYPE" != "macos" ]]; then
    echo "Unsupported os_type: $OS_TYPE"
    echo "Supported os_type: linux, macos"
    exit 1
fi

# Set the target architecture based on the OS type
if [ "$OS_TYPE" == "linux" ]; then
    TARGET_ARCH="x86_64-linux-gnu.2.17"
elif [ "$OS_TYPE" == "macos" ]; then
    TARGET_ARCH="aarch64-macos-none"
fi

# Create temp directory
TEMP_DIR=$(mktemp -d)
trap 'rm -rf ${TEMP_DIR}' EXIT
cd ${TEMP_DIR}  || { echo "Error: Failed to enter temp directory"; exit 1; }

# ====================
# Utility functions
# ====================

# Extract source code function
extract_source() {
    echo "Extracting ${PROJ}.tar.gz..."
    tar xvfz "${BASH_DIR}"/../sources/${PROJ}.tar.gz ||
        { echo "Error: Failed to extract source"; exit 1; }

    cd ${PROJ} 2>/dev/null ||
        cd ${PROJ}-* 2>/dev/null ||
        { echo "Error: Cannot find source directory"; exit 1; }
}

# Build tar archive
build_tar() {
    local name=${1:-${PROJ}}
    local os_type=${2:-${OS_TYPE}}

    # Define the name of the compressed file
    FN_TAR="${name}.${os_type}.tar.gz"

    # Create compressed archive using cbp tar
    cd ${TEMP_DIR} ||
        { echo "==> Error: temp directory not found"; exit 1; }

    cbp tar collect -o "${FN_TAR}" --cleanup ||
        { echo "==> Error: Failed to create archive"; exit 1; }

    # Move archive to the central tar directory
    mv "${FN_TAR}" ${BASH_DIR}/../binaries/ ||
        { echo "==> Error: Failed to move archive"; exit 1; }
}

# Collect binaries from Makefile's all target
collect_make_bins() {
    # Get binary names from Makefile
    BINS=$(make -p | grep "^all: " | sed 's/^all: //')

    # Create collect directory and copy binaries
    mkdir -p ${TEMP_DIR}/collect/bin
    cp ${BINS} ${TEMP_DIR}/collect/bin/
}

# Collect specified binaries
collect_bins() {
    local bins=("$@")

    # Check if any binaries were specified
    if [ ${#bins[@]} -eq 0 ]; then
        echo "Error: No binaries specified"
        exit 1
    fi

    # Create collect directory and copy binaries
    mkdir -p ${TEMP_DIR}/collect/bin
    cp "${bins[@]}" ${TEMP_DIR}/collect/bin/ ||
        { echo "Error: Failed to copy binaries"; exit 1; }
}

# Fix shebang lines in scripts
fix_shebang() {
    local file="$1"

    # Check if file exists and is a regular file
    [ -f "$file" ] || return 1

    # Check if file is a script (has a shebang line)
    if head -n1 "$file" | grep -q '^#!'; then
        # Replace perl path
        if sed -i '1s|^#!.*/perl.*$|#!/usr/bin/env perl|' "$file"; then
            echo "==> Fixed perl shebang in ${file}"
        fi
        # Replace python path
        if sed -i '1s|^#!.*/python.*$|#!/usr/bin/env python3|' "$file"; then
            echo "==> Fixed python shebang in ${file}"
        fi
    fi
}

# Run test program and verify results
run_test() {
    local test_prog="$1"
    local name="${2:-${PROJ}}"

    echo "==> Running ${name} tests..."

    test_output=$(${test_prog})
    test_status=$?

    echo "${test_output}"

    if [ $test_status -ne 0 ]; then
        echo "==> Error: ${name} test failed with status ${test_status}"
        exit 1
    fi

    if ! echo "${test_output}" | grep -q "PASSED"; then
        echo "==> Error: ${name} test did not pass"
        exit 1
    fi

    echo "==> All ${name} tests passed"
}

export -f extract_source build_tar
export -f collect_make_bins collect_bins 
export -f fix_shebang
export -f run_test
