# Source Management

## Basic libraries

```bash
bash scripts/download-source.sh gdbm
bash scripts/download-source.sh gmp
bash scripts/download-source.sh pkgconf

# curl -L https://github.com/llvm/llvm-project/releases/download/llvmorg-19.1.7/openmp-19.1.7.src.tar.xz |
#     tar xvfJ - &&
#     mv openmp-19.1.7.src libomp &&
#     tar -czf sources/libomp.tar.gz libomp/ &&
#     rm -rf libomp

# curl -o sources/clapack.tar.gz -L https://www.netlib.org/clapack/clapack-3.2.1-CMAKE.tgz

```

## My ports

```bash
# Transform Makefile to CMakeLists.txt
bash scripts/download-source.sh pigz

bash scripts/download-source.sh sickle

bash scripts/download-source.sh faops
bash scripts/download-source.sh multiz

bash scripts/download-source.sh bwa

bash scripts/download-source.sh consel

bash scripts/download-source.sh dazzdb
bash scripts/download-source.sh daligner
bash scripts/download-source.sh merquryfk
bash scripts/download-source.sh fastga

# ./configure
bash scripts/download-source.sh datamash

bash scripts/download-source.sh cabextract

bash scripts/download-source.sh trf

bash scripts/download-source.sh aria2

bash scripts/download-source.sh gnuplot

# cmake
bash scripts/download-source.sh diamond

```

## `make`

```bash
bash scripts/download-source.sh fastk
bash scripts/download-source.sh lastz

bash scripts/download-source.sh mafft
bash scripts/download-source.sh trimal

bash scripts/download-source.sh minimap2
bash scripts/download-source.sh miniprot

bash scripts/download-source.sh aster
bash scripts/download-source.sh paml
bash scripts/download-source.sh phast
bash scripts/download-source.sh phylip

bash scripts/download-source.sh prodigal

# curl -L https://github.com/arq5x/bedtools2/archive/refs/tags/v2.31.1.tar.gz |
#     tar xvfz - \
#         --exclude='*/docs*' \
#         --exclude='*/data*' \
#         --exclude='*/genomes*' \
#         --exclude='*/tes*t' \
#         --exclude='*/tutorial*' &&
#     mv bedtools2-2.31.1 bedtools &&
#     tar -czf sources/bedtools.tar.gz bedtools/ &&
#     rm -rf bedtools

```

## `autotools`

```bash
bash scripts/download-source.sh clustalo
bash scripts/download-source.sh mummer
bash scripts/download-source.sh pv
bash scripts/download-source.sh snp-sites

bash scripts/download-source.sh easel
bash scripts/download-source.sh hmmer
bash scripts/download-source.sh hmmer2

# The .tar.gz source code from GitHub requires autoconf/automake to generate ./configure
bash scripts/download-source.sh htslib
bash scripts/download-source.sh samtools
bash scripts/download-source.sh bcftools

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

# curl -o sources/MaSuRCA.tar.gz -L https://github.com/alekseyzimin/masurca/releases/download/v4.1.2/MaSuRCA-4.1.2.tar.gz

```

## `cmake`

```bash
bash scripts/download-source.sh bifrost
bash scripts/download-source.sh spoa

bash scripts/download-source.sh chainnet

# Remove large files
curl -L https://github.com/tjunier/newick_utils/archive/da121155a977197cab9fbb15953ca1b40b11eb87.tar.gz |
    tar xvfz - &&
    mv newick_utils-da121155a977197cab9fbb15953ca1b40b11eb87 newick-utils &&
    fd -t f -S +500k . newick-utils -X rm &&
    tar -czf sources/newick-utils.tar.gz newick-utils/ &&
    rm -rf newick-utils

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

```

## Projects requiring specific build environments

* Built on a CentOS 7 VM using system libgomp

```bash
mkdir -p fasttree &&
    curl -o fasttree/FastTree.c -L https://raw.githubusercontent.com/morgannprice/fasttree/refs/heads/main/old/FastTree-2.1.11.c &&
    tar -czf sources/fasttree.tar.gz fasttree/ &&
    rm -fr fasttree

```

## Rust projects

### CLI utilities

```bash
bash scripts/download-source.sh dust
bash scripts/download-source.sh eza
bash scripts/download-source.sh fd
bash scripts/download-source.sh hyperfine
bash scripts/download-source.sh jnv
bash scripts/download-source.sh resvg
bash scripts/download-source.sh ripgrep
bash scripts/download-source.sh skim
bash scripts/download-source.sh tealdeer
bash scripts/download-source.sh tokei

```

### Bioinformatics utilities

```bash
bash scripts/download-source.sh hnsm
bash scripts/download-source.sh intspan
bash scripts/download-source.sh nwr

bash scripts/download-source.sh pgr
bash scripts/download-source.sh anchr

bash scripts/download-source.sh wgatools

```
