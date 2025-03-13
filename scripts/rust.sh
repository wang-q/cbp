#!/bin/bash

BASH_DIR=$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )

cd "${BASH_DIR}"/..

# Check if the OS type is provided as an argument
if [ -z "$1" ]; then
    echo "Usage: $0 <PROJECT_NAME> [os_type]"
    echo "Supported os_type: linux, macos, windows"
    echo "Example: $0 intspan linux"
    exit 1
fi
PROJECT_NAME=$1

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
OS_TYPE=${2:-$DEFAULT_OS}
USE_NATIVE=0
if [ "$OS_TYPE" == "native" ]; then
    OS_TYPE=$DEFAULT_OS
    USE_NATIVE=1
fi

# Validate the OS type
if [[ "$OS_TYPE" != "linux" && "$OS_TYPE" != "macos" && "$OS_TYPE" != "windows" ]]; then
    echo "Unsupported os_type: $OS_TYPE"
    echo "Supported os_type: linux, macos, windows"
    echo "Add 'native' to use native build"
    exit 1
fi

# Set the target architecture based on the OS type
if [ "$OS_TYPE" == "linux" ]; then
    TARGET_ARCH="x86_64-unknown-linux-gnu.2.17"
elif [ "$OS_TYPE" == "macos" ]; then
    TARGET_ARCH="aarch64-apple-darwin"
elif [ "$OS_TYPE" == "windows" ]; then
    TARGET_ARCH="x86_64-pc-windows-gnu"
fi

# Create a directory for cargo build artifacts
mkdir -p /tmp/cargo
export CARGO_TARGET_DIR=/tmp/cargo

# Check if we should use source tarball or local build
if [ -f "sources/${PROJECT_NAME}.tar.gz" ]; then
    # Create temp directory
    TEMP_DIR=$(mktemp -d)
    trap 'rm -rf ${TEMP_DIR}' EXIT

    # Copy source to temp directory
    cp sources/${PROJECT_NAME}.tar.gz ${TEMP_DIR}/

    # Extract the source code
    cd ${TEMP_DIR}
    tar xvfz ${PROJECT_NAME}.tar.gz || exit 1
    cd ${PROJECT_NAME} 2>/dev/null ||
        cd ${PROJECT_NAME}-* 2>/dev/null ||
        { echo "Error: Cannot find source directory ${PROJECT_NAME}"; exit 1; }
else
    echo "Error: Source file sources/${PROJECT_NAME}.tar.gz not found"
    exit 1
fi

# Build the project with the specified target architecture
if [ "$USE_NATIVE" == "1" ]; then
    cargo build --release || exit 1
else
    cargo zigbuild --target ${TARGET_ARCH} --release || exit 1
fi

# Strip .2.17 from TARGET_ARCH if present
TARGET_ARCH="${TARGET_ARCH%.2.17}"

# List the contents of the release directory
if [ "$USE_NATIVE" == "1" ]; then
    ls $CARGO_TARGET_DIR/release/
else
    ls $CARGO_TARGET_DIR/${TARGET_ARCH}/release/
fi

# Extract the names of binary targets from Cargo.toml
BINS=$(
    cargo metadata --no-deps --format-version 1 |
        jq --raw-output '.packages[] | .targets[] | select( .kind[0] == "bin" ) | .name '
)

# Copy the built binaries to the current directory
mkdir -p collect/bin
for BIN in $BINS; do
    # Determine binary directory based on OS and build type
    if [ "$USE_NATIVE" == "1" ]; then
        BIN_DIR="$CARGO_TARGET_DIR/release"
    else
        BIN_DIR="$CARGO_TARGET_DIR/${TARGET_ARCH}/release"
    fi

    # Add .exe extension for Windows
    if [ "$OS_TYPE" == "windows" ]; then
        BIN_NAME="$BIN.exe"
    else
        BIN_NAME="$BIN"
    fi

    # Check and copy file
    if [ -f "$BIN_DIR/$BIN_NAME" ]; then
        cp "$BIN_DIR/$BIN_NAME" collect/bin/
    else
        echo "Warning: Binary $BIN_NAME not found in $BIN_DIR"
    fi
done

# Define archive name based on OS type
FN_TAR="${PROJECT_NAME}.${OS_TYPE}.tar.gz"

# Create compressed archive with maximum compression
cbp tar collect -o "${FN_TAR}"

# Move archive to the central tar directory
mv ${FN_TAR} ${BASH_DIR}/../binaries/

# Clean up the copied binaries
rm -fr collect
