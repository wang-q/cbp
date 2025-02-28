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
