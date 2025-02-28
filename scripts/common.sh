#!/bin/bash

# Prevent direct execution of this script
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
BASH_DIR=$( cd "$( dirname "${BASH_SOURCE[1]}" )" && pwd )
PROJ=$(basename "${BASH_SOURCE[1]}" .sh)

# Set the default OS type to 'linux'
OS_TYPE=${1:-linux}

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

    # Create compressed archive
    cd ${TEMP_DIR}/collect ||
        { echo "Error: collect directory not found"; exit 1; }

    # Remove unnecessary documentation directories
    rm -rf share/info/ share/man/ share/doc/ share/locale/

    tar -cf - * | gzip -9 > ${TEMP_DIR}/${FN_TAR} ||
        { echo "Error: Failed to create archive"; exit 1; }

    # Move archive to the central tar directory
    mv ${TEMP_DIR}/${FN_TAR} ${BASH_DIR}/../binaries/ ||
        { echo "Error: Failed to move archive"; exit 1; }
}

# Collect binaries from Makefile's all target
collect_make_bins() {
    # Get binary names from Makefile
    BINS=$(make -p | grep "^all: " | sed 's/^all: //')

    # Create collect directory and copy binaries
    mkdir -p ${TEMP_DIR}/collect
    cp ${BINS} ${TEMP_DIR}/collect/
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
    mkdir -p ${TEMP_DIR}/collect
    cp "${bins[@]}" ${TEMP_DIR}/collect/ ||
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
        sed -i '1s|^#!.*/perl.*$|#!/usr/bin/env perl|' "$file"
        # Replace python path
        sed -i '1s|^#!.*/python.*$|#!/usr/bin/env python3|' "$file"
    fi
}

export -f extract_source build_tar collect_make_bins collect_bins fix_shebang
