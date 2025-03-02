# Build Process

This file contains build instructions for each component. Note that:

1. All builds use Zig as the cross-compiler targeting glibc 2.17 for Linux and aarch64 for macOS
2. Build artifacts are packaged into .tar.gz files and stored in the `binaries/` directory
3. Each build is performed in a temporary directory to avoid polluting the project's directories

## Core Libraries

These libraries are fundamental dependencies and will be extensively used by other components. Each library comes with:
1. A build script in `scripts/`
2. Additional tests in `scripts/tests/`
3. Tests should be run on native platform only, as cross-compiled binaries cannot be executed on the build machine

* linux

```bash
bash scripts/zlib.sh -t
bash scripts/bzip2.sh -t
bash scripts/libdeflate.sh -t
bash scripts/xz.sh -t

cbp local zlib bzip2 libdeflate xz

```

* macos

```bash
bash scripts/zlib.sh macos -t
bash scripts/bzip2.sh macos -t
bash scripts/libdeflate.sh -t
bash scripts/xz.sh macos -t

cbp local zlib bzip2 libdeflate xz

```

## Other Libraries

```bash
bash scripts/ncurses.sh
cbp local ncurses
bash scripts/readline.sh
cbp local readline
bash scripts/sqlite.sh

bash scripts/gdbm.sh
bash scripts/expat.sh

bash scripts/pixman.sh
bash scripts/libpng.sh

bash scripts/argtable.sh
bash scripts/libxcrypt.sh

bash scripts/gsl.sh

```

## Common tools

```bash
# Download and package binary for current system
bash scripts/jq.sh

# Download, package and test binary
bash scripts/jq.sh -t

# Download and package for specific OS
bash scripts/jq.sh linux     # Download Linux x86_64 binary
bash scripts/jq.sh macos     # Download macOS ARM64 binary
bash scripts/jq.sh windows   # Download Windows x86_64 binary

# Note: Tests will fail when downloading for different OS
# as binaries cannot be executed on incompatible architectures
bash scripts/jq.sh -t macos  # This will fail on Linux

```

```bash
bash scripts/ninja.sh linux  
bash scripts/ninja.sh macos  
bash scripts/ninja.sh windows  

bash scripts/tsv-utils.sh

```

## `Makefile`

```bash
bash scripts/pigz.sh

bash scripts/bwa.sh
bash scripts/minimap2.sh
bash scripts/miniprot.sh

bash scripts/lastz.sh
bash scripts/sickle.sh
bash scripts/faops.sh
bash scripts/phylip.sh

# bash scripts/mafft.sh # mafft has hard-coded paths

bash scripts/phast.sh # build without CLAPACK

bash scripts/consel.sh
bash scripts/trimal.sh

# use specific commit to ensure reproducibility
bash scripts/DAZZ_DB.sh
bash scripts/DALIGNER.sh
bash scripts/MERQURY.FK.sh
bash scripts/FASTGA.sh

cbp local zlib libdeflate htslib
bash scripts/FASTK.sh

bash scripts/multiz.sh
bash scripts/paml.sh
bash scripts/ASTER.sh

```

## `./configure`

```bash
bash scripts/datamash.sh

bash scripts/TRF.sh
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

bash scripts/eigen.sh

```

## Source codes from Git Repositories

```bash
bash scripts/bcalm.sh

```

## Projects requiring specific build environments

* Built on a CentOS 7 VM with gcc 4.8

```bash
bash scripts/boost.sh

```

* Built on a CentOS 7 VM using system libgomp

```bash
bash scripts/FastTree.sh

```

## Rust projects

### CLI utilities

* linux

```bash
bash scripts/rust.sh eza
bash scripts/rust.sh fd
bash scripts/rust.sh ripgrep
bash scripts/rust.sh hyperfine
bash scripts/rust.sh tealdeer
bash scripts/rust.sh tokei

bash scripts/rust.sh fd windows
bash scripts/rust.sh ripgrep windows
bash scripts/rust.sh hyperfine windows
bash scripts/rust.sh tealdeer windows
bash scripts/rust.sh tokei windows

```

* macos

```bash
bash scripts/rust.sh eza macos
bash scripts/rust.sh fd macos
bash scripts/rust.sh ripgrep macos
bash scripts/rust.sh hyperfine macos
bash scripts/rust.sh tealdeer macos
bash scripts/rust.sh tokei macos

```

### My bioinformatics utilities

```bash
bash scripts/rust.sh intspan
bash scripts/rust.sh nwr
bash scripts/rust.sh hnsm
bash scripts/rust.sh pgr
bash scripts/rust.sh anchr

```

## Binary tarballs

```bash
bash scripts/muscle.sh -t linux
bash scripts/reseek.sh -t linux
bash scripts/usearch.sh -t linux

bash scripts/mosdepth.sh -t linux

#bash scripts/pup.sh

bash scripts/iqtree2.sh -t linux
bash scripts/iqtree2.sh macos
bash scripts/iqtree2.sh windows

bash scripts/mash.sh -t linux
bash scripts/mash.sh macos

bash scripts/megahit.sh -t linux

bash scripts/mmseqs.sh -t linux
bash scripts/mmseqs.sh macos

bash scripts/raxml-ng.sh -t linux
bash scripts/raxml-ng.sh macos

bash scripts/freebayes.sh -t linux

# java
bash scripts/fastqc.sh linux
bash scripts/fastqc.sh macos

bash scripts/picard.sh linux
bash scripts/picard.sh macos

```
