#!/usr/bin/env bash

BASH_DIR=$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )

# Create temp directory
TEMP_DIR=$(mktemp -d)
trap 'rm -rf ${TEMP_DIR}' EXIT
cd ${TEMP_DIR}  || { echo "Error: Failed to enter temp directory"; exit 1; }

if [[ "$OSTYPE" == "darwin"* ]]; then
    TAR_CMD="gtar"
else
    TAR_CMD="tar"
fi

# Ensure a package name is provided
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <package>"
    exit 1
fi

# Read JSON file
package="$1"
json_file="${BASH_DIR}/../packages/${package}.json"

if [ ! -f "$json_file" ]; then
    echo "Error: Package file ${json_file} not found"
    exit 1
fi

# Extract fields using jq
name=$(jq -r '.name // empty' "$json_file")
if [ -z "$name" ]; then
    echo "Error: Field 'name' not found in ${json_file}"
    exit 1
fi

# Check if package name matches
if [ "$name" != "$package" ]; then
    echo "Error: Package name in JSON ($name) does not match requested package ($package)"
    exit 1
fi

# Get source URL, handling both string and object formats
source_type=$(jq -r 'if type == "object" then "object" else "string" end' <<< "$(jq '.source' "$json_file")")
if [ "$source_type" = "object" ]; then
    source_url=$(jq -r '.source.url // empty' "$json_file")
else
    source_url=$(jq -r '.source // empty' "$json_file")
fi

if [ -z "$source_url" ]; then
    echo "Error: Source URL not found in ${json_file}"
    exit 1
fi

# Download source code to temp directory with temp name
tempfile="${TEMP_DIR}/download.$$"
curl -o "${tempfile}" -L "$source_url"

# Handle extraction and repackaging if specified
if [ "$source_type" = "object" ]; then
    # Get extract command if specified
    extract_cmd=$(jq -r '.source.extract // empty' "$json_file")
    
    # Extract archive
    if [ ! -z "$extract_cmd" ]; then
        $extract_cmd "${tempfile}"
    else
        $TAR_CMD xf "${tempfile}"
    fi

    # eza -l

    # Handle rename if specified
    rename_pattern=$(jq -r '.source.rename | to_entries[] | .key' "$json_file")
    rename_target=$(jq -r '.source.rename | to_entries[] | .value' "$json_file")
    
    if [ ! -z "$rename_pattern" ] && [ ! -z "$rename_target" ]; then
        # Only rename if source and target are different
        # If they are the same, it's just a marker for the target directory name
        if [ "$rename_pattern" != "$rename_target" ]; then
            mv $rename_pattern $rename_target
        fi
    else
        # If rename not specified, use the first directory
        rename_target=$(find . -maxdepth 1 -type d ! -name "." -printf "%f\n" | head -1)
    fi

    # Clean specified files and directories
    clean_paths=$(jq -r '.source.clean[]? // empty' "$json_file")
    if [ ! -z "$clean_paths" ]; then
        echo "$clean_paths" | while read -r path; do
            rm -rf "$path"
        done
    fi

    # Set permissions and create reproducible archive
    find $rename_target -type d -exec chmod 755 {} \;
    find $rename_target -type f -exec chmod 644 {} \;
    GZIP=-n $TAR_CMD --format=gnu \
        --sort=name \
        --owner=0 --group=0 --numeric-owner \
        --mode=a+rX,u+w,go-w \
        --mtime='2024-01-01 00:00Z' \
        -czf "${tempfile}" $rename_target/
fi

# Move final archive to sources directory with proper name
mv "${tempfile}" "${BASH_DIR}/../sources/${name}.tar.gz"
