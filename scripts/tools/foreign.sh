#!/bin/bash

list_foreign() {
    echo "==> Foreign files in $HOME/.cbp/:"
    # Create temp file to store known files
    local temp_known=$(mktemp)
    trap 'rm -f ${temp_known}' EXIT

    # Collect files from installed packages
    if [ -d "$HOME/.cbp/binaries" ]; then
        cat "$HOME/.cbp/binaries"/*.files > "${temp_known}" 2>/dev/null
    fi

    # Find and display files not in known list
    if [[ "$(uname)" == "Darwin" ]]; then
        cd "$HOME/.cbp" && find . -type f \
            -not -path "./bin/cbp" \
            -not -path "./binaries/*" \
            -not -path "./cache/*" \
            -print | sed 's|^./||' | sort
    else
        find "$HOME/.cbp/" -type f \
            -not -path "$HOME/.cbp/bin/cbp" \
            -not -path "$HOME/.cbp/binaries/*" \
            -not -path "$HOME/.cbp/cache/*" \
            -printf "%P\n" | sort
    fi | 
    while read -r file; do
        if ! grep -Fxq "$file" "${temp_known}"; then
            echo "  $file"
        fi
    done

    echo
}

# Run the function
list_foreign
