# Build Process

This document follows the same structure as `doc/binaries.md` for consistency and easier reference.

## `vcpkg` libraries

```bash
bash scripts/vcpkg.sh zlib macos
bash scripts/vcpkg.sh bzip2[tool] macos
bash scripts/vcpkg.sh libdeflate macos
bash scripts/vcpkg.sh liblzma[tools] macos

cbp local zlib bzip2 libdeflate liblzma

# bash scripts/vcpkg.sh ncurses macos
# bash scripts/vcpkg.sh readline macos

bash scripts/vcpkg.sh argtable2 macos
bash scripts/vcpkg.sh expat macos

bash scripts/vcpkg.sh gsl macos
# bash scripts/vcpkg.sh gmp macos

# bash scripts/vcpkg.sh libpng[core,tools] macos
bash scripts/vcpkg.sh openjpeg macos

# bash scripts/vcpkg.sh fontconfig[core,tools] macos

```

## `vcpkg` utilities

```bash
# avoid icu from sqlite3[*]
bash scripts/vcpkg.sh "sqlite3[core,tool,dbstat,fts3,fts4,fts5,json1,math,rtree,soundex,zlib]" macos

# bash scripts/vcpkg.sh "openssl[core,tools]" macos

# bash scripts/vcpkg.sh "curl[core,tool,ssl,http2,websockets]" macos

bash scripts/vcpkg.sh pkgconf macos pkgconf=pkg-config

# bash scripts/vcpkg.sh graphviz macos
# gdal

```

## My ports

```bash
# Transform Makefile to CMakeLists.txt
bash scripts/vcpkg.sh pigz macos
bash scripts/vcpkg.sh sickle macos
bash scripts/vcpkg.sh faops macos

bash scripts/vcpkg.sh bwa macos

bash scripts/vcpkg.sh consel macos

# use specific commit to ensure reproducibility
bash scripts/vcpkg.sh dazzdb macos
bash scripts/vcpkg.sh daligner macos
bash scripts/vcpkg.sh merquryfk macos
bash scripts/vcpkg.sh fastga macos

bash scripts/vcpkg.sh multiz macos

# ./configure
bash scripts/vcpkg.sh trf macos
bash scripts/vcpkg.sh datamash macos

# cmake
# bash scripts/vcpkg.sh diamond macos

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
