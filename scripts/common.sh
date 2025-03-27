#!/bin/bash

# Common Shell Library
#
# This script provides a set of common variables and utility functions for building
# and packaging projects. It serves as a shared library and must be sourced by other
# scripts rather than executed directly.
#
# Usage:
#   source "$(dirname "${BASH_SOURCE[0]}")/common.sh"
#
# Arguments:
#   os_type     Target OS (linux, macos, or windows, default: current OS)
#
# Variables:
#   BASH_DIR    Directory of the calling script
#   PROJ        Name of the calling script (without .sh)
#   OS_TYPE     Operating system type
#   TARGET_ARCH Target architecture for compilation
#                 - linux: x86_64-linux-gnu.2.17
#                 - macos: aarch64-macos-none
#                 - windows: x86_64-windows-gnu
#   TEMP_DIR    Temporary working directory (auto-cleaned on exit)
#   CBP_HOME    CBP installation prefix
#   CBP_INCLUDE Include directory
#   CBP_LIB     Library directory

# Prevent direct execution of this script
# ${BASH_SOURCE[0]} is the path of the currently executing script (common.sh)
# ${0} is the path used to call the script
# If they are equal, the script is being executed directly instead of being sourced
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    echo "Error: This script should be sourced, not executed directly"
    echo "Usage: source ${BASH_SOURCE[0]} [-t] [os_type]"
    exit 1
fi

# Get the directory of the script and project name
# ${BASH_SOURCE[1]} is the path of the script that sourced this file
# When common.sh is sourced by another script (e.g., zlib.sh),
# ${BASH_SOURCE[1]} refers to zlib.sh's path
BASH_DIR=$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )
PROJ=$(basename "${BASH_SOURCE[1]}" .sh)

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
   [[ "$OS_TYPE" != "windows" ]]; then
    echo "Unsupported os_type: $OS_TYPE"
    echo "Supported os_type: linux, macos, windows"
    exit 1
fi

# Set the target architecture and binary suffix based on OS type
if [ "$OS_TYPE" == "linux" ]; then
    TARGET_ARCH="x86_64-linux-gnu.2.17"
elif [ "$OS_TYPE" == "macos" ]; then
    TARGET_ARCH="aarch64-macos-none"
elif [ "$OS_TYPE" == "windows" ]; then
    TARGET_ARCH="x86_64-windows-gnu"
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

export -f extract_source
