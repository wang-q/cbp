# Build Process

This document describes the build process for all packages in the cbp (Cross-platform Binary Packages) project.

## Overview

Key points about the build process:

1. Package definitions are stored in the `packages/` directory as JSON files
2. Source codes are downloaded (and optionally repackaged) to the `sources/` directory
3. Most builds use Zig as the cross-compiler targeting glibc 2.17 for Linux
4. Build artifacts are packaged into .tar.gz files and stored in the `binaries/` directory
5. Each build is performed in a temporary directory to avoid polluting the project's directories

## Package Analysis

Commands for analyzing package configurations:

```bash
# Find packages with any rename field at any level in the JSON structure
fd -e json . packages -x sh -c 'jq -e ".. | objects | select(has(\"rename\"))" {} > /dev/null 2>&1 && echo {}'

# Find packages without tests field
fd -e json . packages -x jq -r 'select(.tests == null) | .name'

fd -e json . packages -x jq -r 'select(.tests == null) | .name' |
    grep -v -Fw -f <(fd -e sh . test-binaries -x basename {} .sh) |
    sort

# Find packages that are of type "vcpkg" but don't have a "source" field
# These packages typically use official vcpkg ports and don't need custom source downloads
# Used for identifying packages that rely on vcpkg's standard source acquisition
fd -e json . packages -x jq -r 'select(.type == "vcpkg" and ([.. | objects | has("source")] | any | not)) | .name'

fd -e json . packages -x jq -r 'select(.type == "prebuild" and ([.. | objects | has("binary")] | any | not)) | .name'

# Count all package types and sort by frequency
fd -e json . packages -x jq -r '.type // "undefined"' | sort | uniq -c | sort -rn
#   32 prebuild
#   26 vcpkg
#   20 make
#   17 rust
#   14 autotools
#    9 font
#    5 cmake
#    2 source

fd -e json . packages -x jq -r 'select(.type == "prebuild") | .name'

```

## Package Status

```bash
bash scripts/tools/status.sh |
    rgr md stdin -c 3-5

```

## `vcpkg` libraries

```bash
bash scripts/vcpkg.sh zlib
bash scripts/vcpkg.sh "bzip2[tool]"
bash scripts/vcpkg.sh libdeflate
bash scripts/vcpkg.sh "liblzma[tools]"

cbp local zlib bzip2 libdeflate liblzma

bash scripts/vcpkg.sh ncurses
bash scripts/vcpkg.sh readline

bash scripts/vcpkg.sh argtable2
bash scripts/vcpkg.sh expat
bash scripts/vcpkg.sh "libxml2[core,zlib]"

CFLAGS="-Wno-language-extension-token" bash scripts/vcpkg.sh libxcrypt

bash scripts/vcpkg.sh gsl
bash scripts/vcpkg.sh simde

```

## `vcpkg` utilities

```bash
# avoid icu from sqlite3[*]
bash scripts/vcpkg.sh "sqlite3[core,tool,dbstat,fts3,fts4,fts5,json1,math,rtree,soundex,zlib]"

bash scripts/vcpkg.sh "openssl[core,tools]"

bash scripts/vcpkg.sh "curl[core,tool,ssl,http2,websockets]"

bash scripts/vcpkg.sh pkgconf x64-linux-zig pkgconf=pkg-config

```

## My ports

```bash
# Transform Makefile to CMakeLists.txt
cbp build source bwa
bash scripts/vcpkg.sh bwa

cbp build source consel
bash scripts/vcpkg.sh consel

cbp build source faops
bash scripts/vcpkg.sh faops

cbp build source multiz
bash scripts/vcpkg.sh multiz

cbp build source pigz
bash scripts/vcpkg.sh pigz

cbp build source sickle
bash scripts/vcpkg.sh sickle

cbp build source daligner
bash scripts/vcpkg.sh daligner

cbp build source dazzdb
bash scripts/vcpkg.sh dazzdb

cbp build source fastga
bash scripts/vcpkg.sh fastga

cbp build source merquryfk
bash scripts/vcpkg.sh merquryfk

cbp build source seqtk
bash scripts/vcpkg.sh seqtk

# ./configure
cbp build source cabextract
bash scripts/vcpkg.sh cabextract

cbp build source datamash
bash scripts/vcpkg.sh datamash

cbp build source trf
bash scripts/vcpkg.sh trf

# cmake
cbp build source chainnet
bash scripts/vcpkg.sh chainnet

cbp build source diamond
bash scripts/vcpkg.sh diamond

```

## `make`

```bash
cbp build source aster
bash scripts/aster.sh

cbp build source bedtools
cbp install zlib bzip2 libdeflate liblzma
cbp remove htslib # confuse bundled htslib
bash scripts/bedtools.sh

cbp build source fastk
cbp local zlib libdeflate htslib
bash scripts/fastk.sh

cbp build source lastz
bash scripts/lastz.sh

cbp build source mafft
bash scripts/mafft.sh

cbp build source minimap2
bash scripts/minimap2.sh

cbp build source miniprot
bash scripts/miniprot.sh

cbp build source paml
bash scripts/paml.sh

cbp build source phast
bash scripts/phast.sh # build without CLAPACK

cbp build source phylip
bash scripts/phylip.sh

cbp build source prodigal
bash scripts/prodigal.sh

cbp build source trimal
bash scripts/trimal.sh

```

## `autotools`

```bash
cbp build source bcftools
bash scripts/bcftools.sh    # bundled htslib

cbp build source clustalo
cbp local argtable2
bash scripts/clustalo.sh

cbp build source easel
bash scripts/easel.sh

cbp build srouce gdbm
bash scripts/gdbm.sh

cbp build source hmmer
bash scripts/hmmer.sh

cbp build source hmmer2
bash scripts/hmmer2.sh

cbp build source htslib
cbp local libdeflate
zvm use 0.13.0
bash scripts/htslib.sh  # --with-libdeflate

cbp build source mummer
bash scripts/mummer.sh

cbp build source parallel
bash scripts/parallel.sh

cbp build source pv
cbp local ncurses
bash scripts/pv.sh

cbp build source samtools
bash scripts/samtools.sh    # bundled htslib

cbp build source snp-sites
bash scripts/snp-sites.sh

cbp build source stow
bash scripts/stow.sh

# mcl
curl -L https://micans.org/mcl/src/cimfomfa-22-273.tar.gz |
    tar xz &&
    mv cimfomfa-* cimfomfa &&
    curl -L https://micans.org/mcl/src/mcl-22-282.tar.gz |
    tar xz &&
    mv mcl-* mcl &&
    mv cimfomfa mcl/ &&
    tar -czf sources/mcl.tar.gz mcl/ &&
    rm -rf mcl
bash scripts/mcl.sh

# curl -o sources/MaSuRCA.tar.gz -L https://github.com/alekseyzimin/masurca/releases/download/v4.1.2/MaSuRCA-4.1.2.tar.gz

```

## `cmake`

```bash
cbp build source bifrost
zvm use 0.13.0
bash scripts/bifrost.sh

cbp build source spoa
bash scripts/spoa.sh

# Remove large files
curl -L https://github.com/tjunier/newick_utils/archive/da121155a977197cab9fbb15953ca1b40b11eb87.tar.gz |
    tar xvfz - &&
    mv newick_utils-da121155a977197cab9fbb15953ca1b40b11eb87 newick-utils &&
    fd -t f -S +500k . newick-utils -X rm &&
    tar -czf sources/newick-utils.tar.gz newick-utils/ &&
    rm -rf newick-utils
bash scripts/newick-utils.sh # bison, flex

```

## Source codes from Git Repositories

This section clones recursively and sets up all required git repo at specific commits.

```bash
# bcalm
REPO=bcalm
git clone --recursive https://github.com/GATB/${REPO}.git
cd ${REPO}
git checkout v2.2.3

rm -rf .git
rm -rf gatb-core/.git
cd ..
tar -cf - ${REPO}/ | gzip -9 > sources/${REPO}.tar.gz
rm -rf ${REPO}

bash scripts/bcalm.sh

```

## Projects requiring specific build environments

* Build on Ubuntu host using CentOS 7 container to utilize system libgomp
* Use singularity container which automatically mounts host's $HOME directory
* Working directory constraints:
    * cbp/ must be a real directory, not a symbolic link
    * Do not place cbp/ under /mnt/c/ in WSL to avoid performance issues

```bash
singularity pull docker://wangq/vcpkg-centos:master

mv vcpkg-centos_master.sif vcpkg/vcpkg-centos.sif

mkdir -p fasttree &&
    curl -o fasttree/FastTree.c -L https://raw.githubusercontent.com/morgannprice/fasttree/refs/heads/main/old/FastTree-2.1.11.c &&
    tar -czf sources/fasttree.tar.gz fasttree/ &&
    rm -fr fasttree
bash scripts/fasttree.sh

```

```bash
# lapack-reference
singularity run \
    vcpkg/vcpkg-centos.sif \
/opt/vcpkg/vcpkg install --debug --recurse \
    --clean-buildtrees-after-build \
    --x-buildtrees-root=vcpkg/buildtrees \
    --downloads-root=vcpkg/downloads \
    --x-install-root=vcpkg/installed \
    --x-packages-root=vcpkg/packages \
    lapack-reference[core,cblas]:x64-linux-release

cbp collect vcpkg/installed/vcpkg/info/lapack-reference_*_x64-linux-release.list

# gmp
singularity run \
    vcpkg/vcpkg-centos.sif \
/opt/vcpkg/vcpkg install --debug --recurse \
    --clean-buildtrees-after-build \
    --x-buildtrees-root=vcpkg/buildtrees \
    --downloads-root=vcpkg/downloads \
    --x-install-root=vcpkg/installed \
    --x-packages-root=vcpkg/packages \
    gmp:x64-linux-release

cbp collect vcpkg/installed/vcpkg/info/gmp_*_x64-linux-release.list

# glib -Wno-missing-prototypes -Wno-strict-prototypes
# fontconfig[tools] -std=gnu99
# pkgconf -D_GNU_SOURCE
singularity run \
    --env CFLAGS="-D_GNU_SOURCE -std=gnu99 -Wno-missing-prototypes -Wno-strict-prototypes" \
    vcpkg/vcpkg-centos.sif \
/opt/vcpkg/vcpkg install --debug --recurse \
    --clean-buildtrees-after-build \
    --x-buildtrees-root=vcpkg/buildtrees \
    --downloads-root=vcpkg/downloads \
    --x-install-root=vcpkg/installed \
    --x-packages-root=vcpkg/packages \
    graphviz:x64-linux-release

singularity run vcpkg/vcpkg-centos.sif \
    ldd -v vcpkg/installed/x64-linux-release/tools/graphviz/dot

singularity run vcpkg/vcpkg-centos.sif \
    ldd -v vcpkg/installed/x64-linux-release/tools/glib/gio

cbp collect --ignore tools/graphviz/graphviz/libgvplugin \
    vcpkg/installed/vcpkg/info/graphviz_*_x64-linux-release.list
mv graphviz.linux.tar.gz binaries/

# gnuplot
cbp build source gnuplot

singularity run \
    --env CFLAGS="-D_GNU_SOURCE -std=gnu99 -Wno-missing-prototypes -Wno-strict-prototypes" \
    vcpkg/vcpkg-centos.sif \
/opt/vcpkg/vcpkg install --debug --recurse \
    --clean-buildtrees-after-build \
    --overlay-ports=ports \
    --x-buildtrees-root=vcpkg/buildtrees \
    --downloads-root=vcpkg/downloads \
    --x-install-root=vcpkg/installed \
    --x-packages-root=vcpkg/packages \
    gnuplot:x64-linux-release

cbp collect vcpkg/installed/vcpkg/info/gnuplot_*_x64-linux-release.list
mv gnuplot.linux.tar.gz binaries/

# python3
singularity run \
    vcpkg/vcpkg-centos.sif \
/opt/vcpkg/vcpkg install --debug --recurse \
    --clean-buildtrees-after-build \
    --overlay-ports=ports \
    --x-buildtrees-root=vcpkg/buildtrees \
    --downloads-root=vcpkg/downloads \
    --x-install-root=vcpkg/installed \
    --x-packages-root=vcpkg/packages \
    python3[core,extensions]:x64-linux-release

cbp collect vcpkg/installed/vcpkg/info/python3_*_x64-linux-release.list
mv python3.linux.tar.gz binaries/python3.11.linux.tar.gz

# static python can't load shared libraries
# zvm use 0.14.0
# CFLAGS="-Wno-date-time" \
#     bash scripts/vcpkg.sh python3

```

## Prebuilt packages from the official repositories

### Development Environments

```bash
cbp build prebuild cmake
cbp build prebuild nodejs
cbp build prebuild openjdk # version 17

```

### Standalone Tools

```bash
cbp build prebuild jq yq
cbp build prebuild gh
cbp build prebuild ninja uv

cbp build prebuild aria2
cbp build prebuild bat
cbp build prebuild pandoc tectonic
cbp build prebuild pup tsv-utils

```

### Bioinformatics tools

```bash
cbp build prebuild blast

cbp build prebuild muscle reseek usearch
cbp build prebuild freebayes mosdepth megahit

cbp build prebuild bowtie2 stringtie
cbp build prebuild iqtree2
cbp build prebuild mash mmseqs raxml-ng

bash scripts/prebuilds/sratoolkit.sh linux
bash scripts/prebuilds/sratoolkit.sh macos
bash scripts/prebuilds/sratoolkit.sh windows

```

### Java

These packages require `java` environment. They are installed in `libexec` with symlinks or shims placed in `bin/`.

These packages might not be the latest versions due to the provided OpenJDK 17, but they provide similar functionalities.

```bash
cbp build prebuild aliview
cbp build prebuild fastqc
cbp build prebuild figtree
cbp build prebuild igv
cbp build prebuild picard

```

### Fonts

```bash
cbp install cabextract

cbp build font arial charter helvetica

# Open Source Fonts
cbp build font fira jetbrains-mono firacode-nf

cbp build font lxgw-wenkai
cbp build font source-han-sans
cbp build font source-han-serif

```
