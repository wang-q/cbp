#!/bin/bash

# Detect platform
if [[ "$(uname)" == "Darwin" ]]; then
    OS_TYPE="macos"
else
    OS_TYPE="linux"
fi

BASH_DIR=$( cd "$( dirname "${BASH_SOURCE[1]}" )" && pwd )

# Common directories
CBP_HOME="$HOME/.cbp"
CBP_BIN="$CBP_HOME/bin"
CBP_CACHE="$CBP_HOME/cache"
CBP_BINARIES="$CBP_HOME/binaries"

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

# Create necessary directories
ensure_dirs() {
    mkdir -p "$CBP_BIN" "$CBP_CACHE" "$CBP_BINARIES"
}

# Platform specific find command
find_files() {
    local dir="$1"
    local pattern="${2:-*}"
    if [[ "$(uname)" == "Darwin" ]]; then
        cd "$dir" && find . -type f -name "$pattern" -print | sed 's|^./||' | sort
    else
        find "$dir" -type f -name "$pattern" -printf "%P\n" | sort
    fi
}

# Install package from a tar.gz file
install_package() {
    local pkg_name="$1"
    local pkg_file="$2"

    echo "==> Installing ${pkg_name}"
    ensure_dirs

    # List files in package
    tar tzf "${pkg_file}" > "${CBP_BINARIES}/${pkg_name}.files" || {
        echo "    Failed to list files in ${pkg_name}"
        return 1
    }

    # Extract files
    tar xzf "${pkg_file}" --directory="${CBP_HOME}" || {
        echo "    Failed to extract ${pkg_name}"
        rm -f "${CBP_BINARIES}/${pkg_name}.files"
        return 1
    }

    echo "    Done"
    return 0
}
