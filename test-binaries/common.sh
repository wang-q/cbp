#!/bin/bash

# Prevent direct execution of this script
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    echo "Error: This script should be sourced, not executed directly"
    echo "Usage: source ${BASH_SOURCE[0]}"
    exit 1
fi

# Get the directory of the script and project name
BASH_DIR=$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )
PROJ=$(basename "${BASH_SOURCE[1]}" .sh)

# Set strict mode
set -euo pipefail

# Create temp directory
TEMP_DIR=$(mktemp -d)
trap 'rm -rf ${TEMP_DIR}' EXIT
cd ${TEMP_DIR} || { echo "Error: Failed to enter temp directory"; exit 1; }

# ====================
# Utility functions
# ====================

# Check if command exists in bin directory
check_command() {
    local cmd="$1"
    local cmd_path="$(cbp prefix bin)/${cmd}"

    if [ ! -f "${cmd_path}" ] && [ ! -f "${cmd_path}.exe" ]; then
        echo "Error: Command '${cmd}' not found"
        return 1
    fi
    return 0
}

# Test version output
test_version() {
    local cmd="$1"
    local pattern="$2"
    local version_arg="${3:---version}"

    echo "-> Testing version output"
    check_command "${cmd}" || return 1

    local VERSION_OUTPUT
    VERSION_OUTPUT=$($(cbp prefix bin)/${cmd} ${version_arg} 2>&1)
    echo "${VERSION_OUTPUT}"

    if [ -n "${VERSION_OUTPUT}" ] && [[ "${VERSION_OUTPUT}" =~ ${pattern} ]]; then
        echo "Version test PASSED"
        return 0
    else
        echo "Version test FAILED"
        echo "Expected output matching pattern: ${pattern}"
        echo "Got:"
        echo "${VERSION_OUTPUT}"
        return 1
    fi
}

# Assert condition is true
assert() {
    local condition="$1"
    local message="${2:-Expected condition to be true}"

    # Temporarily disable errexit
    set +e
    eval "${condition}"
    local result=$?
    set -e

    if [ ${result} -eq 0 ]; then
        echo "Test PASSED"
        return 0
    else
        echo "Test FAILED"
        echo "${message}"
        return 1
    fi
}

# Assert values are equal
assert_eq() {
    local result="$1"
    local expected="$2"
    local message="${3:-Expected output}"

    if [ "${result}" = "${expected}" ]; then
        echo "Test PASSED"
        return 0
    else
        echo "Test FAILED"
        echo "${message}"
        echo "Expected:"
        echo "${expected}"
        echo "Got:"
        echo "${result}"
        return 1
    fi
}

export -f test_version assert assert_eq
