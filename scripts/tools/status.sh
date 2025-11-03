#!/bin/bash

# Pre-cache package lists for all platforms
LINUX_PKGS=$(cbp avail linux)
MACOS_PKGS=$(cbp avail macos)
WINDOWS_PKGS=$(cbp avail windows)
FONT_PKGS=$(cbp avail font)

# Print table header
echo -e "Type\tPackage\tLinux\tmacOS\tWindows"

# Get all types and package names
fd -e json . packages -x jq -r '.type // "undefined"' | sort -u |
while read -r type; do
    [ -z "$type" ] && continue

    first=true
    fd -e json . packages -x jq -r "select(.type == \"$type\") | .name" | sort |
    while read -r pkg; do
        [ -z "$pkg" ] && continue

        if [ "$type" = "font" ]; then
            mark=$(echo "$FONT_PKGS" | grep -w "$pkg" > /dev/null && echo "✅" || echo "")
            linux=$mark
            macos=$mark
            windows=$mark
        else
            linux=$(
                echo "$LINUX_PKGS" | grep -w "$pkg" > /dev/null &&
                    echo "✅" || echo ""
            )
            macos=$(
                echo "$MACOS_PKGS" | grep -w "$pkg" > /dev/null &&
                    echo "✅" || echo ""
            )
            windows=$(
                echo "$WINDOWS_PKGS" | grep -w "$pkg" > /dev/null &&
                echo "✅" || echo ""
            )
        fi

        if [ "$first" = true ]; then
            echo -e "${type}\t${pkg}\t${linux}\t${macos}\t${windows}"
            first=false
        else
            echo -e "\t${pkg}\t${linux}\t${macos}\t${windows}"
        fi
    done
done
