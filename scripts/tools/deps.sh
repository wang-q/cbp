#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

check_dependencies() {
    local pkg="$1"
    local record_file="${CBP_RECORDS}/${pkg}.files"

    if [ ! -f "${record_file}" ]; then
        echo "Warning: Package ${pkg} is not installed"
        return 1
    fi

    echo "==> Dependencies for package ${pkg}:"
    while read -r file; do
        local full_path="${CBP_HOME}/${file}"
        # Skip symlinks
        if [ -L "${full_path}" ]; then
            continue
        fi

        if [ -f "${full_path}" ] && [ -x "${full_path}" ]; then
            # Skip text files
            if file "${full_path}" | grep -q "text"; then
                continue
            fi

            echo "  File: ${file}"
            if [[ "$(uname)" == "Darwin" ]]; then
                # macOS: use otool
                local deps=$(
                    otool -L "${full_path}" |
                        grep -v "${full_path}:" |
                        grep -v "libSystem\.B\.dylib" |
                        grep -v "libc++\.1\.dylib" |
                        sed 's/^[[:space:]]*//'
                )
                if [ -n "${deps}" ]; then
                    echo "${deps}" | sed 's/^/    /'
                else
                    echo "    No additional dependencies"
                fi
            else
                # Linux: use ldd
                if command -v ldd >/dev/null 2>&1; then
                    local ldd_out=$(ldd "${full_path}" 2>&1)
                    if [[ ${ldd_out} == *"not a dynamic executable"* ]]; then
                        echo "    Static executable"
                    else
                        local deps=$(
                            echo "${ldd_out}" |
                                grep -v -E 'linux-vdso|ld-linux' |
                                grep -v -E 'libc.so|libpthread|libdl.so' |
                                grep -v -E 'libm.so|libgcc_s.so|libstdc\+\+'
                        )
                        if [ -n "${deps}" ]; then
                            echo "${deps}" | sed 's/^/    /'
                        else
                            echo "    No additional dependencies"
                        fi
                    fi
                else
                    echo "    Warning: ldd not found"
                fi
            fi
            echo
        fi
    done < "${record_file}"
}

# Process packages
for pkg in "$@"; do
    check_dependencies "${pkg}"
done
