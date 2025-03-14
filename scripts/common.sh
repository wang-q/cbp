#!/bin/bash

# Prevent direct execution of this script
# ${BASH_SOURCE[0]} is the path of the currently executing script (common.sh)
# ${0} is the path used to call the script
# If they are equal, the script is being executed directly instead of being sourced
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    echo "Error: This script should be sourced, not executed directly"
    echo "Usage: source ${BASH_SOURCE[0]} [-t] [os_type]"
    echo
    echo "Options:"
    echo "  -t          Run tests after build"
    echo
    echo "Arguments:"
    echo "  os_type     Target OS (linux, macos, or windows, default: current OS)"
    echo
    echo "Environment:"
    echo "  BASH_DIR    Directory of the calling script"
    echo "  PROJ        Name of the calling script (without .sh)"
    echo "  OS_TYPE     Operating system type"
    echo "  TARGET_ARCH Target architecture for compilation"
    echo "  BIN_SUFFIX  Binary suffix (.exe for Windows)"
    echo "  TEMP_DIR    Temporary working directory"
    exit 1
fi

# Get the directory of the script and project name
# ${BASH_SOURCE[1]} is the path of the script that sourced this file
# When common.sh is sourced by another script (e.g., zlib.sh),
# ${BASH_SOURCE[1]} refers to zlib.sh's path
BASH_DIR=$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )
PROJ=$(basename "${BASH_SOURCE[1]}" .sh)

# Process command line options
while getopts "t" opt; do
    case ${opt} in
        t)
            RUN_TEST="test"
            ;;
        *)
            echo "Invalid option: -${OPTARG}"
            exit 1
            ;;
    esac
done
shift $((OPTIND-1))

# Set default OS type based on current system
case "$OSTYPE" in
    darwin*)
        DEFAULT_OS="macos"
        ;;
    linux*)
        DEFAULT_OS="linux"
        ;;
    msys*|cygwin*|mingw*)
        DEFAULT_OS="windows"
        ;;
    *)
        echo "Error: Unsupported operating system: $OSTYPE"
        exit 1
        ;;
esac

# Use provided OS type or default
OS_TYPE=${1:-$DEFAULT_OS}

# Validate the OS type
if [[ "$OS_TYPE" != "linux" ]] &&
   [[ "$OS_TYPE" != "macos" ]] &&
   [[ "$OS_TYPE" != "windows" ]] &&
   [[ "$OS_TYPE" != "font" ]]; then
    echo "Unsupported os_type: $OS_TYPE"
    echo "Supported os_type: linux, macos, windows, font"
    exit 1
fi

# Set the target architecture and binary suffix based on OS type
if [ "$OS_TYPE" == "linux" ]; then
    TARGET_ARCH="x86_64-linux-gnu.2.17"
    BIN_SUFFIX=""
elif [ "$OS_TYPE" == "macos" ]; then
    TARGET_ARCH="aarch64-macos-none"
    BIN_SUFFIX=""
elif [ "$OS_TYPE" == "windows" ]; then
    TARGET_ARCH="x86_64-windows-gnu"
    BIN_SUFFIX=".exe"
elif [ "$OS_TYPE" == "font" ]; then
    TARGET_ARCH=""
    BIN_SUFFIX=""
fi

# Create temp directory
TEMP_DIR=$(mktemp -d)
trap 'rm -rf ${TEMP_DIR}' EXIT
cd ${TEMP_DIR}  || { echo "Error: Failed to enter temp directory"; exit 1; }

# Compiling flags
CBP_HOME="$(cbp prefix)"
CBP_INCLUDE="$(cbp prefix include)"
CBP_LIB="$(cbp prefix lib)"

# ====================
# Utility functions
# ====================

# Extract source code function
extract_source() {
    echo "Extracting ${PROJ}.tar.gz..."

    if [ "$OS_TYPE" == "windows" ]; then
        # Windows: use 7z for better symlink handling
        7z x "${BASH_DIR}"/../sources/${PROJ}.tar.gz -so |
            7z x -aoa -si -ttar ||
            { echo "Error: Failed to extract source"; exit 1; }
    else
        # Linux/macOS: normal extraction
        tar xvfz "${BASH_DIR}"/../sources/${PROJ}.tar.gz ||
            { echo "Error: Failed to extract source"; exit 1; }
    fi

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

# Collect specified binaries
#
# This function handles various binary file scenarios:
# 1. Non-Windows:
#    - Input: program        -> Output: program
#    - Input: libcrypto.a    -> Output: libcrypto.a
#
# 2. Windows:
#    - Input: program        -> Output: program.exe
#    - Input: program.exe    -> Output: program.exe
#    - Input: program.dll    -> Output: program.dll
#    - Input: libcrypto.lib  -> Output: libcrypto.lib
#    - Input: libcrypto.a    -> Output: libcrypto.a
#
# Note: For Windows, if 'program' is provided but 'program.exe' exists,
#       the function will use 'program.exe' as the source file.
collect_bins() {
    local bins=("$@")

    # Check if any binaries were specified
    if [ ${#bins[@]} -eq 0 ]; then
        echo "Error: No binaries specified"
        exit 1
    fi

    # Create collect directory
    mkdir -p ${TEMP_DIR}/collect/bin

    # Process each binary file
    for bin in "${bins[@]}"; do
        # Handle binary name with suffix
        local source_bin="${bin}"
        local base_name=$(basename "${bin}")
        local target_bin="${base_name}"

        # Only add suffix for executables on Windows, and only if they don't already have an extension
        if [ -n "${BIN_SUFFIX}" ] && [[ ! "${base_name}" =~ \.(exe|dll|lib|a)$ ]]; then
            target_bin="${base_name}${BIN_SUFFIX}"
        fi

        # Check if source binary has suffix
        if [ -n "${BIN_SUFFIX}" ] && [ -f "${bin}${BIN_SUFFIX}" ]; then
            source_bin="${bin}${BIN_SUFFIX}"
        fi

        chmod +x "${source_bin}" ||
            { echo "Error: Failed to make binary ${source_bin} executable"; exit 1; }
        cp "${source_bin}" "${TEMP_DIR}/collect/bin/${target_bin}" ||
            { echo "Error: Failed to copy binary ${source_bin}"; exit 1; }
    done
}

# Collect binaries from Makefile's all target
collect_make_bins() {
    # Get binary names from Makefile
    BINS=$(make -p | grep "^all: " | sed 's/^all: //')

    # Use collect_bins to process binaries
    collect_bins ${BINS}
}

# Fix shebang lines in scripts
fix_shebang() {
    local file="$1"

    # Check if file exists and is a regular file
    [ -f "$file" ] || return 0

    # Check if file is a text file
    if ! file -b "${file}" | grep -q "text"; then
        return 0
    fi

    # Check if file is a script (has a shebang line)
    if head -n1 "$file" | grep -q '^#!'; then
        perl -pi -e '
            if ($. == 1) {
                s{^#!.*perl}{#!/usr/bin/env perl} and print STDERR "==> Fixed perl shebang in '$file'\n";
                s{^#!.*python}{#!/usr/bin/env python3} and print STDERR "==> Fixed python shebang in '$file'\n";
            }
        ' "$file"
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
export -f collect_bins collect_make_bins
export -f fix_shebang
export -f run_test
