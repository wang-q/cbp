#!/bin/bash

# Source common build environment
BASH_DIR=$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )

# Usage: prebuild.sh <package_name> [OS]
if [ $# -lt 1 ]; then
    echo "Usage: $0 <package_name> [OS]"
    exit 1
fi

PACKAGE=$1

# package
JSON_FILE="${BASH_DIR}/../packages/${PACKAGE}.json"
if [ ! -f "${JSON_FILE}" ]; then
    echo "Error: Package file not found: ${JSON_FILE}"
    exit 1
fi

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

# Set binary suffix for Windows
if [ "$OS_TYPE" == "windows" ]; then
    BIN_SUFFIX=".exe"
else
    BIN_SUFFIX=""
fi

# Validate the OS type
if [[ "$OS_TYPE" != "linux" ]] &&
   [[ "$OS_TYPE" != "macos" ]] &&
   [[ "$OS_TYPE" != "windows" ]] &&
   [[ "$OS_TYPE" != "font" ]]; then
    echo "Unsupported os_type: $OS_TYPE"
    echo "Supported os_type: linux, macos, windows, font"
    exit 1
fi

# Create temp directory
TEMP_DIR=$(mktemp -d)
trap 'rm -rf ${TEMP_DIR}' EXIT
cd ${TEMP_DIR}  || { echo "Error: Failed to enter temp directory"; exit 1; }

# Read package info
URL=$(jq -r ".downloads.${OS_TYPE}.url" "$JSON_FILE")
if [ -z "${URL}" ]; then
    echo "Error: URL not found for ${OS_TYPE} in ${JSON_FILE}"
    exit 1
fi

# Read all package options
EXTRACT=$(jq -r ".downloads.${OS_TYPE}.extract // empty" "$JSON_FILE")
CLEAN=$(jq -r ".downloads.${OS_TYPE}.clean // empty" "$JSON_FILE")
SHEBANG=$(jq -r ".downloads.${OS_TYPE}.shebang // empty" "$JSON_FILE")

# Handle binary paths as array or single string
BINARY_PATHS=()
while IFS= read -r binary; do
    if [ -n "$binary" ]; then
        BINARY_PATHS+=("$binary")
    fi
done < <(jq -r ".downloads.${OS_TYPE}.binary | if type == \"array\" then .[] else . end" "$JSON_FILE")

if [ ${#BINARY_PATHS[@]} -eq 0 ]; then
    echo "Error: Binary path not found for ${OS_TYPE} in ${JSON_FILE}"
    exit 1
fi

# Download and extract
download_url() {
    local url="$1"
    local output="$2"
    echo "==> Downloading ${url}"
    curl -L "${url}" -o "${output}" ||
        { echo "Error: Failed to download ${url}"; exit 1; }
}

download_package() {
    echo "==> Downloading ${PACKAGE}..."
    if [[ "${URL}" == *.zip ]] || [[ "${URL}" == *.tar.gz ]] || [ -n "${EXTRACT}" ]; then
        # Download file
        if [[ "${URL}" == *.zip ]]; then
            download_url "${URL}" "${PACKAGE}.zip"
            if [ -n "${EXTRACT}" ]; then
                ${EXTRACT} "${PACKAGE}.zip"
            else
                unzip "${PACKAGE}.zip"
            fi
        elif [[ "${URL}" == *.tar.gz ]]; then
            download_url "${URL}" "${PACKAGE}.tar.gz"
            if [ -n "${EXTRACT}" ]; then
                ${EXTRACT} "${PACKAGE}.tar.gz"
            else
                tar xvfz "${PACKAGE}.tar.gz"
            fi
        else
            # Other files that need extraction
            ext="${URL##*.}"
            download_url "${URL}" "${PACKAGE}.${ext}"
            if [ -n "${EXTRACT}" ]; then
                ${EXTRACT} "${PACKAGE}.${ext}"
            fi
        fi

    else
        download_url "${URL}" "${BINARY_PATHS[0]}"
    fi
}

process_binaries() {
    local mode="bin"
    local fn_tar="$1"

    if [ -z "$fn_tar" ]; then
        echo "Error: Missing output filename"
        exit 1
    fi

    if [ "$OS_TYPE" == "font" ]; then
        mode="font"
    fi

    # Handle exclude pattern right after extraction
    if [ -n "${CLEAN}" ]; then
        rm -f ${CLEAN}
    fi
    
    # Add shebang option if enabled
    local shebang_opt=""
    if [ "${SHEBANG}" == "true" ]; then
        shebang_opt="--shebang"
    fi
    
    if [[ "${URL}" == *.zip ]] || [[ "${URL}" == *.tar.gz ]] || [ -n "${EXTRACT}" ]; then
        # Process each binary path
        local all_files=()
        for binary in "${BINARY_PATHS[@]}"; do
            if [[ "${binary}" == *"*"* ]]; then
                shopt -s nullglob
                binary_files=(${binary})
                shopt -u nullglob
                
                if [ ${#binary_files[@]} -gt 0 ]; then
                    all_files+=("${binary_files[@]}")
                fi
            else
                all_files+=("${binary}")
            fi
        done

        if [ ${#all_files[@]} -eq 0 ]; then
            echo "Error: No files found matching patterns"
            exit 1
        fi
        
        cbp collect --mode "${mode}" ${shebang_opt} -o "${fn_tar}" "${all_files[@]}"
    else
        cbp collect --mode "${mode}" ${shebang_opt} -o "${fn_tar}" "${BINARY_PATHS[0]}"
    fi
}

# Main process
download_package
FN_TAR="${PACKAGE}.${OS_TYPE}.tar.gz"
process_binaries "${FN_TAR}"

# Move archive to the central tar directory
mv "${FN_TAR}" "${BASH_DIR}/../binaries/" ||
    { echo "==> Error: Failed to move archive"; exit 1; }
