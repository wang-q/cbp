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
elif [ "$OS_TYPE" == "macos" ]; then
    TARGET_ARCH="aarch64-macos-none"
elif [ "$OS_TYPE" == "windows" ]; then
    TARGET_ARCH="x86_64-windows-gnu"
elif [ "$OS_TYPE" == "font" ]; then
    TARGET_ARCH=""
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

export -f extract_source build_tar
