#!/bin/bash

# Format packages in columns
PERL_FMT='
    BEGIN{
        $p="";
        $count=0;
        $width=80;
    }
    chomp;
    $c = substr($_, 0, 1);
    if ($p ne "" and $c ne $p) {
        print "\n";
        $count = 0;
    }
    if ($count > 0 and $count * 16 + 16 > $width) {
        print "\n";
        $count = 0;
    }
    $p = $c;
    printf "  %-14s", $_;
    $count++;
'

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

list_installed() {
    if [ $# -eq 0 ]; then
        echo "==> Installed packages:"
        if [ -d "${CBP_BINARIES}" ]; then
            find_files "${CBP_BINARIES}" "*.files" |
                sed 's/\.files$//' |
                sort |
                perl -n -e "${PERL_FMT}"
        fi
        echo
    else
        for pkg in "$@"; do
            if [ -f "${CBP_BINARIES}/${pkg}.files" ]; then
                echo "==> Files in package ${pkg}:"
                cat "${CBP_BINARIES}/${pkg}.files"
                echo
            else
                echo "Warning: Package ${pkg} is not installed"
            fi
        done
    fi
}

# Process arguments
list_installed "$@"
