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

source "$(dirname "${BASH_SOURCE[0]}")/header.sh"

list_installed() {
    if [ $# -eq 0 ]; then
        echo "==> Installed packages:"
        if [ -d "${CBP_RECORDS}" ]; then
            find_files "${CBP_RECORDS}" "*.files" |
                sed 's/\.files$//' |
                sort |
                perl -n -e "${PERL_FMT}"
        fi
        echo
    else
        for pkg in "$@"; do
            if [ -f "${CBP_RECORDS}/${pkg}.files" ]; then
                echo "==> Files in package ${pkg}:"
                cat "${CBP_RECORDS}/${pkg}.files"
                echo
            else
                echo "==> Package ${pkg} is not installed"
            fi
        done
    fi
}

# Process arguments
list_installed "$@"
