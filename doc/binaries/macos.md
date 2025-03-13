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

bash scripts/vcpkg.sh gsl
# bash scripts/vcpkg.sh gmp
bash scripts/vcpkg.sh simde

bash scripts/vcpkg.sh "libxml2[core,iconv,lzma,zlib]"

```

## `vcpkg` utilities

```bash
# avoid icu from sqlite3[*]
bash scripts/vcpkg.sh "sqlite3[core,tool,dbstat,fts3,fts4,fts5,json1,math,rtree,soundex,zlib]"

bash scripts/vcpkg.sh "openssl[core,tools]" arm64-osx-release

bash scripts/vcpkg.sh "curl[core,tool,ssl,http2,websockets]" arm64-osx-release

bash scripts/vcpkg.sh pkgconf arm64-macos-zig pkgconf=pkg-config

# bash scripts/vcpkg.sh cpuinfo[core,tools]

# bash scripts/vcpkg.sh graphviz
# gdal

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
bash scripts/vcpkg.sh trf
bash scripts/vcpkg.sh datamash

# cmake
# bash scripts/vcpkg.sh diamond # need sse3

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
# bash scripts/mummer.sh

cbp local argtable2
# bash scripts/clustalo.sh

cbp local libdeflate
bash scripts/htslib.sh # --with-libdeflate

# bundled htslib
# bash scripts/samtools.sh
# bash scripts/bcftools.sh

```

## `cmake`

```bash
# bash scripts/bifrost.sh
# bash scripts/spoa.sh

# bash scripts/newick-utils.sh # bison, flex

```

## Source codes from Git Repositories

```bash
# bash scripts/bcalm.sh

```

## Projects requiring specific build environments

* Built on a CentOS 7 VM using system libgomp

```bash
# bash scripts/FastTree.sh

```
