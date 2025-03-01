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

list_installed() {
    if [ $# -eq 0 ]; then
        echo "==> Installed packages:"
        if [ -d "$HOME/.cbp/binaries" ]; then
            find "$HOME/.cbp/binaries" -name "*.files" -exec basename {} \; |
                sed 's/\.files$//' |
                sort |
                perl -n -e "${PERL_FMT}"
        fi
        echo
    else
        for pkg in "$@"; do
            if [ -f "$HOME/.cbp/binaries/${pkg}.files" ]; then
                echo "==> Files in package ${pkg}:"
                cat "$HOME/.cbp/binaries/${pkg}.files"
                echo
            else
                echo "Warning: Package ${pkg} is not installed"
            fi
        done
    fi
}

# Process arguments
list_installed "$@"
