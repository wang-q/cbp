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
bash scripts/zlib.sh linux test
bash scripts/bzip2.sh linux test
bash scripts/libdeflate.sh linux test
bash scripts/xz.sh linux test

```

* macos

```bash
bash scripts/zlib.sh macos test
bash scripts/bzip2.sh macos test
# bash scripts/libdeflate.sh macos test
bash scripts/xz.sh macos test

```

## Other Libraries

```bash

bash scripts/tools/local.sh zlib bzip2 libdeflate xz

bash scripts/ncurses.sh
bash scripts/readline.sh

bash install.sh ncurses readline

bash scripts/sqlite.sh

bash scripts/gdbm.sh
bash scripts/expat.sh

bash scripts/libpng.sh
bash scripts/pixman.sh

bash scripts/argtable.sh
bash scripts/libxcrypt.sh

bash scripts/gsl.sh

# --with-libdeflate
bash scripts/htslib.sh

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
bash scripts/FASTK.sh # Depend on zlib, libdeflate and libhts
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

bash scripts/clustalo.sh # depends on argtable

bash scripts/htslib.sh # depends on libdeflate, --with-libdeflate

# bundled htslib
bash scripts/samtools.sh
bash scripts/bcftools.sh

```

## `cmake`

```bash
bash scripts/bifrost.sh
bash scripts/spoa.sh
bash scripts/diamond.sh

bash scripts/newick-utils.sh

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

```bash
bash scripts/rust.sh fd
bash scripts/rust.sh ripgrep
# bash scripts/rust.sh bat
bash scripts/rust.sh hyperfine
bash scripts/rust.sh tealdeer
bash scripts/rust.sh tokei

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
BIN=usearch
curl -o ${BIN} -L https://github.com/rcedgar/usearch12/releases/download/v12.0-beta1/usearch_linux_x86_12.0-beta
chmod +x ${BIN}
tar -cf - ${BIN} | gzip -9 > tar/${BIN}.linux.tar.gz
rm ${BIN}

BIN=reseek
curl -o ${BIN} -L https://github.com/rcedgar/reseek/releases/download/v2.3/reseek-v2.3-linux-x86
chmod +x ${BIN}
tar -cf - ${BIN} | gzip -9 > tar/${BIN}.linux.tar.gz
rm ${BIN}

BIN=muscle
curl -o ${BIN} -L https://github.com/rcedgar/muscle/releases/download/v5.3/muscle-linux-x86.v5.3
chmod +x ${BIN}
tar -cf - ${BIN} | gzip -9 > tar/${BIN}.linux.tar.gz
rm ${BIN}

BIN=mosdepth
curl -o ${BIN} -L https://github.com/brentp/mosdepth/releases/download/v0.3.11/mosdepth
chmod +x ${BIN}
tar -cf - ${BIN} | gzip -9 > tar/${BIN}.linux.tar.gz
rm ${BIN}

bash scripts/tsv-utils.sh
bash scripts/pup.sh

bash scripts/raxml-ng.sh
bash scripts/mash.sh
bash scripts/megahit.sh
bash scripts/mmseqs.sh
bash scripts/freebayes.sh
bash scripts/iqtree2.sh

# java
bash scripts/fastqc.sh
bash scripts/picard.sh

```
