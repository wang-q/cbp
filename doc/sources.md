# Source Management

## Manage package json

```bash
# Find packages with source.rename field
fd -e json . packages -x sh -c 'jq -e ".source.rename" {} > /dev/null 2>&1 && echo {}'

# Find packages with any rename field at any level in the JSON structure
fd -e json . packages -x sh -c 'jq -e ".. | objects | select(has(\"rename\"))" {} > /dev/null 2>&1 && echo {}'

# Find packages without tests field
fd -e json . packages -x sh -c 'jq -e ".tests" {} > /dev/null 2>&1 || echo {}'

# Find packages that are of type "vcpkg" but don't have a "source" field
# These packages typically use official vcpkg ports and don't need custom source downloads
# Used for identifying packages that rely on vcpkg's standard source acquisition
fd -e json . packages -x sh -c 'jq -e "select(.type == \"vcpkg\" and ([.. | objects | has(\"source\")] | any | not))" {} > /dev/null 2>&1 && echo {}'

fd -e json . packages -x sh -c 'jq -e "select(.type == \"prebuild\" and ([.. | objects | has(\"binary\")] | any | not))" {} > /dev/null 2>&1 && echo {}'

# Count all package types and sort by frequency
fd -e json . packages -x jq -r '.type // "undefined"' | sort | uniq -c | sort -rn
# 29 prebuild
# 26 vcpkg
# 18 make
# 16 rust
# 12 autotools
# 9 font
# 5 cmake
# 2 source

fd -e json . packages -x sh -c 'jq -e "select(.type == \"prebuild\")" {} > /dev/null 2>&1 && echo {}'

fd -e json . packages -x sh -c 'jq -e "select(has(\"type\") | not)" {} > /dev/null 2>&1 && echo {}'

```

## My ports

```bash
# Transform Makefile to CMakeLists.txt
cbp build source bwa
cbp build source consel
cbp build source dazzdb daligner fastga merquryfk
cbp build source faops multiz
cbp build source pigz
cbp build source sickle

# ./configure
cbp build source cabextract
cbp build source datamash
cbp build source trf
cbp build source gnuplot

# cmake
cbp build source diamond

```

## `make`

```bash
cbp build source aster paml phast phylip
cbp build source fastk
cbp build source lastz
cbp build source mafft trimal
cbp build source minimap2 miniprot
cbp build source prodigal

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
cbp build source gdbm pv

cbp build source clustalo mummer snp-sites
cbp build source easel hmmer hmmer2

# The .tar.gz source code from GitHub requires autoconf/automake to generate ./configure
cbp build source htslib bcftools samtools

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
cbp build source bifrost
cbp build source spoa
cbp build source chainnet

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
cbp build source dust eza fd ripgrep skim
cbp build source jnv resvg
cbp build source hyperfine tokei tealdeer

```

### Bioinformatics utilities

```bash
cbp build source hnsm intspan nwr
cbp build source pgr anchr
cbp build source wgatools

```
