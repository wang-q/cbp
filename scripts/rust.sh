#!/bin/bash

BASH_DIR=$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )

cd "${BASH_DIR}"/..

# Check if the OS type is provided as an argument
if [ -z "$1" ]; then
    echo "Usage: $0 <PROJECT_NAME> [os_type]"
    echo "Supported os_type: linux, macos"
    echo "Example: $0 intspan linux"
    exit 1
fi
PROJECT_NAME=$1

# Set the default OS type to 'linux'
OS_TYPE=${2:-linux}

# Validate the OS type
if [[ "$OS_TYPE" != "linux" && "$OS_TYPE" != "macos" ]]; then
    echo "Unsupported os_type: $OS_TYPE"
    echo "Supported os_type: linux, macos"
    exit 1
fi

# Set the target architecture based on the OS type
if [ "$OS_TYPE" == "linux" ]; then
    TARGET_ARCH="x86_64-unknown-linux-gnu.2.17"
elif [ "$OS_TYPE" == "macos" ]; then
    TARGET_ARCH="aarch64-apple-darwin"
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
    cd ${PROJECT_NAME} || cd ${PROJECT_NAME}-* || exit 1
else
    # Enter the PROJECT_NAME project directory for local build
    cd ${PROJECT_NAME} || exit 1
fi

# Build the project with the specified target architecture
cargo zigbuild --target ${TARGET_ARCH} --release || exit 1

# Strip .2.17 from TARGET_ARCH if present
TARGET_ARCH="${TARGET_ARCH%.2.17}"

# List the contents of the release directory
ls $CARGO_TARGET_DIR/${TARGET_ARCH}/release/

# Extract the names of binary targets from Cargo.toml
BINS=$(
    cargo read-manifest |
        jq --raw-output '.targets[] | select( .kind[0] == "bin" ) | .name '
)

# Copy the built binaries to the current directory
mkdir collect
for BIN in $BINS; do
    cp $CARGO_TARGET_DIR/${TARGET_ARCH}/release/$BIN collect/
done

# Define archive name based on OS type
FN_TAR="${PROJECT_NAME}.${OS_TYPE}.tar.gz"

# Create compressed archive with maximum compression
cbp tar collect -o "${FN_TAR}"

# Move archive to the central tar directory
mv ${FN_TAR} ${BASH_DIR}/../binaries/

# Clean up the copied binaries
rm -fr collect
