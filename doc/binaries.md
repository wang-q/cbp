# Build Process

This file contains build instructions for each component. Note that:

1. All builds use Zig as the cross-compiler targeting glibc 2.17 for Linux and aarch64 for macOS
2. Build artifacts are packaged into .tar.gz files and stored in the `binaries/` directory
3. Each build is performed in a temporary directory to avoid polluting the project's directories

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
bash scripts/vcpkg.sh "libxml2[core,iconv,lzma,zlib]"

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

# static python can't load shared libraries
# zvm use 0.14.0
# CFLAGS="-Wno-date-time" \
#     bash scripts/vcpkg.sh python3

```

## My ports

```bash
# Transform Makefile to CMakeLists.txt
bash scripts/vcpkg.sh pigz
bash scripts/vcpkg.sh sickle
bash scripts/vcpkg.sh faops

bash scripts/vcpkg.sh bwa

bash scripts/vcpkg.sh consel

# use specific commit to ensure reproducibility
bash scripts/vcpkg.sh dazzdb
bash scripts/vcpkg.sh daligner
bash scripts/vcpkg.sh merquryfk
bash scripts/vcpkg.sh fastga

bash scripts/vcpkg.sh multiz

# ./configure
bash scripts/vcpkg.sh datamash
bash scripts/vcpkg.sh cabextract

bash scripts/vcpkg.sh trf

# cmake
bash scripts/vcpkg.sh chainnet
bash scripts/vcpkg.sh diamond

```

## Libraries

```bash
# ./configure
bash scripts/gdbm.sh

zvm use 0.13.0
cbp local libdeflate
bash scripts/htslib.sh # --with-libdeflate

```

## `Makefile`

```bash
bash scripts/minimap2.sh
bash scripts/miniprot.sh

bash scripts/lastz.sh
bash scripts/phylip.sh

bash scripts/mafft.sh

bash scripts/phast.sh # build without CLAPACK

bash scripts/trimal.sh

# use specific commit to ensure reproducibility
cbp local zlib libdeflate htslib
bash scripts/fastk.sh

bash scripts/paml.sh
bash scripts/aster.sh

bash scripts/prodigal.sh

```

## `./configure`

```bash
bash scripts/hmmer.sh
bash scripts/easel.sh
bash scripts/hmmer2.sh
bash scripts/mummer.sh

cbp local argtable2
bash scripts/clustalo.sh

cbp local ncurses
bash scripts/pv.sh

# bundled htslib
bash scripts/samtools.sh
bash scripts/bcftools.sh

bash scripts/snp-sites.sh
bash scripts/mcl.sh

```

## `cmake`

```bash
zvm use 0.13.0
bash scripts/bifrost.sh
bash scripts/spoa.sh

bash scripts/newick-utils.sh # bison, flex

```

## Source codes from Git Repositories

```bash
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

bash scripts/fasttree.sh

```

## Prebuilds from the official repositories

### Common tools

```bash
cbp build prebuild jq yq
cbp build prebuild ninja uv

cbp build prebuild pandoc tectonic
cbp build prebuild bat 
cbp build prebuild pup tsv-utils

bash scripts/prebuilds/cmake.sh linux
bash scripts/prebuilds/cmake.sh macos
bash scripts/prebuilds/cmake.sh windows

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

### java

```bash
bash scripts/prebuilds/fastqc.sh linux
bash scripts/prebuilds/fastqc.sh macos

bash scripts/prebuilds/picard.sh linux
bash scripts/prebuilds/picard.sh macos

```
