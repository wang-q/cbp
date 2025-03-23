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
YAML_FILE="${BASH_DIR}/../packages/${PACKAGE}.yaml"
if [ ! -f "${YAML_FILE}" ]; then
    echo "Error: Package file not found: ${YAML_FILE}"
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

# Read YAML file
URL=$(yq ".downloads.${OS_TYPE}.url" "$YAML_FILE")
if [ -z "${URL}" ]; then
    echo "Error: URL not found for ${OS_TYPE} in ${YAML_FILE}"
    exit 1
fi

BINARY=$(yq ".downloads.${OS_TYPE}.binary" "$YAML_FILE")
if [ -z "${BINARY}" ]; then
    echo "Error: Binary path not found for ${OS_TYPE} in ${YAML_FILE}"
    exit 1
fi

EXTRACT=$(yq ".downloads.${OS_TYPE}.extract" "$YAML_FILE")
EXCLUDE=$(yq ".downloads.${OS_TYPE}.exclude" "$YAML_FILE")

collect_bins() {
    local bins=("$@")
    local target_dir="bin"

    if [ "$OS_TYPE" == "font" ]; then
        target_dir="share/fonts"
    fi

    # Check if any files were specified
    if [ ${#bins[@]} -eq 0 ]; then
        echo "Error: No files specified"
        exit 1
    fi

    # Create collect directory
    mkdir -p "${TEMP_DIR}/collect/${target_dir}"

    # Process each file
    for bin in "${bins[@]}"; do
        local source_bin="${bin}"
        local base_name=$(basename "${bin}")
        local target_bin="${base_name}"

        if [ "$OS_TYPE" != "font" ]; then
            # Only add suffix for executables on Windows
            if [ -n "${BIN_SUFFIX}" ] && [[ ! "${base_name}" =~ \.(exe|dll|lib|a)$ ]]; then
                target_bin="${base_name}${BIN_SUFFIX}"
            fi

            # Check if source binary has suffix
            if [ -n "${BIN_SUFFIX}" ] && [ -f "${bin}${BIN_SUFFIX}" ]; then
                source_bin="${bin}${BIN_SUFFIX}"
            fi

            chmod +x "${source_bin}" ||
                { echo "Error: Failed to make binary ${source_bin} executable"; exit 1; }
        fi

        cp "${source_bin}" "${TEMP_DIR}/collect/${target_dir}/${target_bin}" ||
            { echo "Error: Failed to copy file ${source_bin}"; exit 1; }
    done
}

# Download and extract
echo "==> Downloading ${PACKAGE}..."
if [ -n "${EXTRACT}" ] || [[ "${URL}" == *.zip ]] || [[ "${URL}" == *.tar.gz ]] || [[ "${URL}" == *.exe ]]; then
    # Download file
    if [[ "${URL}" == *.zip ]]; then
        curl -L "${URL}" -o "${PACKAGE}.zip"
        if [ -n "${EXTRACT}" ]; then
            ${EXTRACT} "${PACKAGE}.zip"
        else
            unzip "${PACKAGE}.zip"
        fi
    elif [[ "${URL}" == *.tar.gz ]]; then
        curl -L "${URL}" -o "${PACKAGE}.tar.gz"
        if [ -n "${EXTRACT}" ]; then
            ${EXTRACT} "${PACKAGE}.tar.gz"
        else
            tar xvfz "${PACKAGE}.tar.gz"
        fi
    else
        # Other files that need extraction
        ext="${URL##*.}"
        curl -L "${URL}" -o "${PACKAGE}.${ext}"
        ${EXTRACT} "${PACKAGE}.${ext}"
    fi

    # Handle exclude pattern right after extraction
    if [ -n "${EXCLUDE}" ]; then
        rm -f ${EXCLUDE}
    fi

    # Handle glob pattern
    if [[ "${BINARY}" == *"*"* ]]; then
        shopt -s nullglob
        BINARY_FILES=(${BINARY})
        shopt -u nullglob
        
        if [ ${#BINARY_FILES[@]} -eq 0 ]; then
            echo "Error: No files found matching pattern: ${BINARY}"
            exit 1
        fi
        collect_bins "${BINARY_FILES[@]}"
    else
        collect_bins "${BINARY}"
    fi
else
    # Single file
    curl -L "${URL}" -o "${BINARY}" ||
        { echo "Error: Failed to download ${BINARY}"; exit 1; }
    collect_bins "${BINARY}"
fi

# Create package
FN_TAR="${PACKAGE}.${OS_TYPE}.tar.gz"

cbp tar collect -o "${FN_TAR}" --cleanup ||
    { echo "==> Error: Failed to create archive"; exit 1; }

# Move archive to the central tar directory
mv "${FN_TAR}" ${BASH_DIR}/../binaries/ ||
    { echo "==> Error: Failed to move archive"; exit 1; }
