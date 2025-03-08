# Build Process

This file contains build instructions for each component. Note that:

1. All builds use Zig as the cross-compiler targeting glibc 2.17 for Linux and aarch64 for macOS
2. Build artifacts are packaged into .tar.gz files and stored in the `binaries/` directory
3. Each build is performed in a temporary directory to avoid polluting the project's directories

## `vcpkg` libraries

```bash
# List all available features for a package
vcpkg search bzip2
# To remove a vcpkg package
vcpkg install --debug --recurse \
    --clean-buildtrees-after-build --clean-packages-after-build \
    --overlay-ports=ports \
    --overlay-triplets="$(cbp prefix triplets)" \
    --x-install-root="$(cbp prefix cache)" \
    zlib:x64-linux-zig
vcpkg remove --debug --recurse \
    --overlay-ports=ports \
    --overlay-triplets="$(cbp prefix triplets)" \
    --x-install-root="$(cbp prefix cache)" \
    zlib:x64-linux-zig
# Install zlib with custom target
# vcpkg install zlib:x64-linux-zig \
#     --cmake-args="-DCMAKE_C_COMPILER_TARGET=aarch64-macos-none" \
#     --cmake-args="-DCMAKE_CXX_COMPILER_TARGET=aarch64-macos-none"

bash scripts/vcpkg.sh zlib linux
bash scripts/vcpkg.sh bzip2[tool] linux
bash scripts/vcpkg.sh libdeflate linux
bash scripts/vcpkg.sh liblzma[tools] linux

cbp local zlib bzip2 libdeflate xz

bash scripts/vcpkg.sh ncurses linux
bash scripts/vcpkg.sh readline linux

bash scripts/vcpkg.sh argtable2 linux
bash scripts/vcpkg.sh expat linux

bash scripts/vcpkg.sh gsl linux
# bash scripts/vcpkg.sh gmp linux

```

## `vcpkg` utilities

```bash
# avoid icu from sqlite3[*]
bash scripts/vcpkg.sh "sqlite3[core,tool,dbstat,fts3,fts4,fts5,json1,math,rtree,soundex,zlib]" linux

bash scripts/vcpkg.sh "openssl[core,tools]" linux

bash scripts/vcpkg.sh "curl[core,tool,ssl,http2,websockets]" linux

bash scripts/vcpkg.sh pkgconf linux pkgconf=pkg-config

# bash scripts/vcpkg.sh graphviz linux
# gdal

```

## My ports

```bash
# Transform Makefile to CMakeLists.txt
bash scripts/vcpkg.sh pigz linux
bash scripts/vcpkg.sh sickle linux
bash scripts/vcpkg.sh faops linux
bash scripts/vcpkg.sh multiz linux

bash scripts/vcpkg.sh bwa linux

bash scripts/vcpkg.sh consel linux

# ./configure
bash scripts/vcpkg.sh trf linux
bash scripts/vcpkg.sh datamash linux

```

## `Makefile`

```bash
bash scripts/minimap2.sh
bash scripts/miniprot.sh

bash scripts/lastz.sh
bash scripts/phylip.sh

# bash scripts/mafft.sh # mafft has hard-coded paths

bash scripts/phast.sh # build without CLAPACK

bash scripts/trimal.sh

# use specific commit to ensure reproducibility
bash scripts/DAZZ_DB.sh
bash scripts/DALIGNER.sh
bash scripts/MERQURY.FK.sh
bash scripts/FASTGA.sh

cbp local zlib libdeflate htslib
bash scripts/FASTK.sh

bash scripts/paml.sh
bash scripts/ASTER.sh

```

## `./configure`

```bash
bash scripts/hmmer.sh
bash scripts/hmmer2.sh
bash scripts/mummer.sh

cbp local argtable
bash scripts/clustalo.sh

cbp local libdeflate
bash scripts/htslib.sh # --with-libdeflate

# bundled htslib
bash scripts/samtools.sh
bash scripts/bcftools.sh

```

## `cmake`

```bash
bash scripts/bifrost.sh
bash scripts/spoa.sh
bash scripts/diamond.sh

bash scripts/newick-utils.sh # bison, flex

```

## Source codes from Git Repositories

```bash
bash scripts/bcalm.sh

```

## Projects requiring specific build environments

* Built on a CentOS 7 VM using system libgomp

```bash
bash scripts/FastTree.sh

```
