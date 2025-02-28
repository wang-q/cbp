# Build Process

This file contains build instructions for each component. Note that:

1. All builds use Zig as the cross-compiler targeting glibc 2.17 for Linux and aarch64 for macOS
2. Build artifacts are packaged into .tar.gz files and stored in the `binaries/` directory
3. Each build is performed in a temporary directory to avoid polluting the project's directories

## Basic Libraries

```bash
bash script/zlib.sh
bash script/bzip2.sh
bash script/xz.sh
bash script/libdeflate.sh

bash install.sh zlib libdeflate bzip2 xz

bash script/ncurses.sh
bash script/readline.sh

bash install.sh ncurses readline

bash script/sqlite.sh

bash script/gdbm.sh
bash script/expat.sh

bash script/libpng.sh
bash script/pixman.sh

bash script/argtable.sh
bash script/libxcrypt.sh

bash script/gsl.sh

# --with-libdeflate
bash script/htslib.sh

```

## `Makefile`

```bash
bash script/pigz.sh

bash script/bwa.sh
bash script/minimap2.sh
bash script/miniprot.sh

bash script/lastz.sh
bash script/sickle.sh
bash script/faops.sh
bash script/phylip.sh

# bash script/mafft.sh # mafft has hard-coded paths

bash script/phast.sh # build without CLAPACK

bash script/consel.sh
bash script/trimal.sh

# use specific commit to ensure reproducibility
bash script/DAZZ_DB.sh
bash script/DALIGNER.sh
bash script/MERQURY.FK.sh
bash script/FASTGA.sh
bash script/FASTK.sh # Depend on zlib, libdeflate and libhts
bash script/multiz.sh
bash script/paml.sh
bash script/ASTER.sh

```

## `./configure`

```bash
bash script/datamash.sh

bash script/TRF.sh
bash script/hmmer.sh
bash script/hmmer2.sh
bash script/mummer.sh

bash script/clustalo.sh # depends on argtable

bash script/htslib.sh # depends on libdeflate, --with-libdeflate

# bundled htslib
bash script/samtools.sh
bash script/bcftools.sh

```

## `cmake`

```bash
bash script/bifrost.sh
bash script/spoa.sh
bash script/diamond.sh

bash script/newick-utils.sh

bash scripts/eigen.sh

```

## Source codes from Git Repositories

```bash
bash script/bcalm.sh

```

## Projects requiring specific build environments

* Built on a CentOS 7 VM with gcc 4.8

```bash
bash script/boost.sh

```

* Built on a CentOS 7 VM using system libgomp

```bash
bash script/FastTree.sh

```

## Rust projects

### CLI utilities

```bash
bash script/rust.sh fd
bash script/rust.sh ripgrep
# bash script/rust.sh bat
bash script/rust.sh hyperfine
bash script/rust.sh tealdeer
bash script/rust.sh tokei

```

### My bioinformatics utilities

```bash
bash script/rust.sh intspan
bash script/rust.sh nwr
bash script/rust.sh hnsm
bash script/rust.sh pgr
bash script/rust.sh anchr

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

bash script/tsv-utils.sh
bash script/pup.sh

bash script/raxml-ng.sh
bash script/mash.sh
bash script/megahit.sh
bash script/mmseqs.sh
bash script/freebayes.sh
bash script/iqtree2.sh

# java
bash script/fastqc.sh
bash script/picard.sh

```
