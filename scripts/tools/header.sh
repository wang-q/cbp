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
CBP_RECORDS="$CBP_HOME/records"

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
    mkdir -p "$CBP_BIN" "$CBP_CACHE" "$CBP_RECORDS"
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
    local pkg="$1"
    local pkg_file="$2"

    echo "==> Installing ${pkg}"
    ensure_dirs

    # List files in package
    tar tzf "${pkg_file}" > "${CBP_RECORDS}/${pkg}.files" || {
        echo "    Failed to list files in ${pkg}"
        return 1
    }

    # Extract files
    tar xzf "${pkg_file}" --directory="${CBP_HOME}" || {
        echo "    Failed to extract ${pkg}"
        rm -f "${CBP_RECORDS}/${pkg}.files"
        return 1
    }

    echo "    Done"
    return 0
}

# Download package from GitHub release
download_package() {
    local pkg="$1"
    local temp_file="${CBP_CACHE}/${pkg}.${OS_TYPE}.tar.gz.incomplete"
    
    ensure_dirs
    
    if ! gh release download Binaries -p "${pkg}.${OS_TYPE}.tar.gz" -O "${temp_file}"; then
        echo "    Failed to download ${pkg} for ${OS_TYPE}"
        return 1
    fi
    
    mv "${temp_file}" "${CBP_CACHE}/${pkg}.${OS_TYPE}.tar.gz"
    return 0
}
