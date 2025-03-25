#!/usr/bin/env bash

BASH_DIR=$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )

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

source_url=$(jq -r '.source // empty' "$json_file")
if [ -z "$source_url" ]; then
    echo "Error: Field 'source' not found in ${json_file}"
    exit 1
fi

# Check if package name matches
if [ "$name" != "$package" ]; then
    echo "Error: Package name in JSON ($name) does not match requested package ($package)"
    exit 1
fi

# Download source code
curl -o "${BASH_DIR}/../sources/${name}.tar.gz" -L "$source_url"
