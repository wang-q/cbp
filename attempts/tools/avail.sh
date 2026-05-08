#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/header.sh"

list_packages() {
    local pattern="$1"
    local message="$2"
    echo "==> ${message}"
    gh release view Binaries --json assets -q '.assets[].name' |
        grep -E "${pattern}" |
        sed -E "s/${pattern}//" |
        sort |
        uniq |
        perl -n -e "${PERL_FMT}"
    echo
}

if [ -z "$1" ]; then
    list_packages "\.(linux|macos)\.tar\.gz$" "Available packages"
else
    list_packages "\.$1\.tar\.gz$" "Available packages for $1"
fi
