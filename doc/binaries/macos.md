# Build Process

This document follows the same structure as `doc/binaries.md` for consistency and easier reference.

## `vcpkg` libraries

```bash
bash scripts/vcpkg.sh zlib
bash scripts/vcpkg.sh "bzip2[tool]"
bash scripts/vcpkg.sh libdeflate
bash scripts/vcpkg.sh "liblzma[tools]"

cbp local zlib bzip2 libdeflate liblzma

bash scripts/vcpkg.sh ncurses arm64-osx-release
bash scripts/vcpkg.sh readline-unix arm64-osx-release
mv binaries/readline-unix.macos.tar.gz binaries/readline.macos.tar.gz

bash scripts/vcpkg.sh argtable2
bash scripts/vcpkg.sh expat
bash scripts/vcpkg.sh "libxml2[core,iconv,lzma,zlib]"

# macOS has libxcrypt

bash scripts/vcpkg.sh gsl
# bash scripts/vcpkg.sh gmp
bash scripts/vcpkg.sh simde

```

## `vcpkg` utilities

```bash
# avoid icu from sqlite3[*]
bash scripts/vcpkg.sh "sqlite3[core,tool,dbstat,fts3,fts4,fts5,json1,math,rtree,soundex,zlib]"

bash scripts/vcpkg.sh "openssl[core,tools]" arm64-osx-release

bash scripts/vcpkg.sh "curl[core,tool,ssl,http2,websockets]" arm64-osx-release

bash scripts/vcpkg.sh pkgconf arm64-macos-zig pkgconf=pkg-config

bash scripts/vcpkg.sh python3 arm64-osx-release
mv binaries/python3.macos.tar.gz binaries/python3.11.macos.tar.gz

# bash scripts/vcpkg.sh gnuplot

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
# bash scripts/vcpkg.sh pv arm64-osx-release
# bash scripts/vcpkg.sh aria2 arm64-osx-release

bash scripts/vcpkg.sh trf

# cmake
# bash scripts/vcpkg.sh diamond # need sse3

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

# bash scripts/mafft.sh # mafft has hard-coded paths

# bash scripts/phast.sh # build without CLAPACK

bash scripts/trimal.sh

# use specific commit to ensure reproducibility
cbp local zlib libdeflate htslib
bash scripts/fastk.sh

bash scripts/paml.sh
bash scripts/aster.sh

```

## `./configure`

```bash
bash scripts/hmmer.sh
# bash scripts/hmmer2.sh
bash scripts/mummer.sh

zvm use 0.13.0
cbp local argtable2
bash scripts/clustalo.sh

# bundled htslib
zvm use 0.13.0
bash scripts/samtools.sh
bash scripts/bcftools.sh

bash scripts/mcl.sh

```

## `cmake`

```bash
zvm use 0.13.0
# bash scripts/bifrost.sh
bash scripts/spoa.sh

# bash scripts/newick-utils.sh # bison, flex

```

## Source codes from Git Repositories

```bash
# bash scripts/bcalm.sh

```

## Projects requiring specific build environments

```bash
# bash scripts/FastTree.sh

# fatal error: 'iostream' file not found
vcpkg install --debug --recurse \
    --clean-buildtrees-after-build \
    --x-buildtrees-root=vcpkg/buildtrees \
    --downloads-root=vcpkg/downloads \
    --x-install-root=vcpkg/installed \
    --x-packages-root=vcpkg/packages \
    libgd

```
