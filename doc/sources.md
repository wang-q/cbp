# Source Management

## Basic libraries

```bash

bash scripts/download-source.sh gmp

bash scripts/download-source.sh pkgconf

bash scripts/download-source.sh gdbm

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

bash scripts/download-source.sh bwa

# just .tar file
curl -L http://stat.sys.i.kyoto-u.ac.jp/prog/consel/pub/cnsls020.tgz |
    tar xvf - &&
    tar -czf sources/consel.tar.gz consel/ &&
    rm -fr consel

# use specific commit to ensure reproducibility
curl -L https://github.com/thegenemyers/DAZZ_DB/archive/be65e5991ec0aa4ebbfa926ea00e3680de7b5760.tar.gz |
    tar xvfz - &&
    mv DAZZ_DB-* dazzdb &&
    tar -czf sources/dazzdb.tar.gz dazzdb/ &&
    rm -rf dazzdb

curl -L https://github.com/thegenemyers/DALIGNER/archive/a8e2f42f752f21d21c92fbc39c75b16b52c6cabe.tar.gz |
    tar xvfz - &&
    mv DALIGNER-* daligner &&
    tar -czf sources/daligner.tar.gz daligner/ &&
    rm -rf daligner

curl -L https://github.com/thegenemyers/MERQURY.FK/archive/a1005336b0eae8a1dd478017e3dbbae5366ccda5.tar.gz |
    tar xvfz - &&
    mv MERQURY.FK-* merquryfk &&
    tar -czf sources/merquryfk.tar.gz merquryfk/ &&
    rm -rf merquryfk

curl -L https://github.com/thegenemyers/FASTGA/archive/e97c33ef4daeafdfbb7b5dda56d31eaac9a5e214.tar.gz |
    tar xvfz - &&
    mv FASTGA-* fastga &&
    rm -fr fastga/EXAMPLE &&
    tar -czf sources/fastga.tar.gz fastga/ &&
    rm -rf fastga

bash scripts/download-source.sh multiz

# ./configure
bash scripts/download-source.sh datamash

bash scripts/download-source.sh cabextract

curl -L https://github.com/Benson-Genomics-Lab/TRF/archive/refs/tags/v4.09.1.tar.gz |
    tar xvfz - &&
    mv TRF-* trf &&
    tar -czf sources/trf.tar.gz trf/ &&
    rm -rf trf

curl -L https://github.com/aria2/aria2/releases/download/release-1.37.0/aria2-1.37.0.tar.xz |
    tar xvfJ - &&
    mv aria2-* aria2 &&
    tar -czf sources/aria2.tar.gz aria2/ &&
    rm -rf aria2

bash scripts/download-source.sh gnuplot

# cmake
bash scripts/download-source.sh diamond

```

## `Makefile`

```bash
bash scripts/download-source.sh lastz

bash scripts/download-source.sh mafft

bash scripts/download-source.sh minimap2

bash scripts/download-source.sh miniprot

curl -o sources/phylip.tar.gz -L https://phylipweb.github.io/phylip/download/phylip-3.697.tar.gz

curl -o sources/phast.tar.gz -L https://github.com/CshlSiepelLab/phast/archive/refs/tags/v1.7.tar.gz

# remove unnecessary files to reduce source size
curl -L https://github.com/inab/trimal/archive/refs/tags/v1.5.0.tar.gz |
    tar xvfz - &&
    rm -fr trimal-1.5.0/dataset/ &&
    rm -fr trimal-1.5.0/docs/ &&
    tar -czf sources/trimal.tar.gz trimal-1.5.0/ &&
    rm -rf trimal-1.5.0

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

# use specific commit to ensure reproducibility
curl -L https://github.com/thegenemyers/FASTK/archive/ddea6cf254f378db51d22c6eb21af775fa9e1f77.tar.gz |
    tar xvfz - &&
    mv FASTK-* fastk &&
    tar -czf sources/fastk.tar.gz fastk/ &&
    rm -rf fastk

curl -o sources/paml.tar.gz -L https://github.com/abacus-gene/paml/archive/01508dd10b6e7c746a0768ee3cddadb5c28d5ae0.tar.gz

curl -L https://github.com/chaoszhang/ASTER/archive/e8da7edf8adf4205cf5551630dc77bb81497092b.tar.gz |
    tar xvfz - &&
    mv ASTER-* aster &&
    rm -fr aster/example &&
    rm aster/exe/* &&
    tar -czf sources/aster.tar.gz aster/ &&
    rm -rf aster

curl -L https://github.com/hyattpd/Prodigal/archive/c1e2d361479cc1b18175ea79ebd8ff10411c46cb.tar.gz |
    tar xvfz - &&
    mv Prodigal-* prodigal &&
    tar -czf sources/prodigal.tar.gz prodigal/ &&
    rm -rf prodigal

```

## `./configure`

```bash
curl -o sources/hmmer.tar.gz -L http://eddylab.org/software/hmmer/hmmer-3.4.tar.gz

curl -o sources/easel.tar.gz -L https://github.com/EddyRivasLab/easel/archive/refs/tags/easel-0.49.tar.gz

# hmmer2: rename package to avoid conflict with hmmer3
curl -L http://eddylab.org/software/hmmer/2.4i/hmmer-2.4i.tar.gz |
    tar xvfz - &&
    mv hmmer-2.4i hmmer2 &&
    tar -czf sources/hmmer2.tar.gz hmmer2/ &&
    rm -rf hmmer2

curl -o sources/pv.tar.gz -L https://www.ivarch.com/programs/sources/pv-1.9.31.tar.gz

# curl -o sources/MaSuRCA.tar.gz -L https://github.com/alekseyzimin/masurca/releases/download/v4.1.2/MaSuRCA-4.1.2.tar.gz

curl -o sources/mummer.tar.gz -L https://github.com/mummer4/mummer/releases/download/v4.0.1/mummer-4.0.1.tar.gz

curl -L http://www.clustal.org/omega/clustal-omega-1.2.4.tar.gz |
    tar xvfz - &&
    mv clustal-omega-1.2.4 clustalo &&
    tar -czf sources/clustalo.tar.gz clustalo/ &&
    rm -rf clustalo

# The .tar.gz source code from GitHub requires autoconf/automake to generate ./configure
curl -L https://github.com/samtools/htslib/releases/download/1.21/htslib-1.21.tar.bz2 |
    tar xvfj - &&
    tar -czf sources/htslib.tar.gz htslib-1.21/ &&
    rm -rf htslib-1.21

curl -L https://github.com/samtools/samtools/releases/download/1.21/samtools-1.21.tar.bz2 |
    tar xvfj - &&
    tar -czf sources/samtools.tar.gz samtools-1.21/ &&
    rm -rf samtools-1.21

curl -L https://github.com/samtools/bcftools/releases/download/1.21/bcftools-1.21.tar.bz2 |
    tar xvfj - &&
    tar -czf sources/bcftools.tar.gz bcftools-1.21/ &&
    rm -rf bcftools-1.21

curl -L https://github.com/sanger-pathogens/snp-sites/archive/refs/tags/v2.5.1.tar.gz |
    tar xvfz - &&
    mv snp-sites-* snp-sites &&
    rm -rf snp-sites/example_data/*.gz &&
    tar -czf sources/snp-sites.tar.gz snp-sites/ &&
    rm -rf snp-sites

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

```

## `cmake`

```bash
curl -o sources/bifrost.tar.gz -L https://github.com/pmelsted/bifrost/archive/refs/tags/v1.3.5.tar.gz

curl -o sources/spoa.tar.gz -L https://github.com/rvaser/spoa/archive/refs/tags/4.1.4.tar.gz

curl -o sources/chainnet.tar.gz -L https://github.com/wang-q/chainnet/archive/161cb82417f74ed3caa78932a06baeb2e10094e8.tar.gz

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
curl -o sources/eza.tar.gz -L https://github.com/eza-community/eza/archive/refs/tags/v0.20.23.tar.gz

curl -o sources/fd.tar.gz -L https://github.com/sharkdp/fd/archive/refs/tags/v10.2.0.tar.gz

curl -o sources/dust.tar.gz -L https://github.com/bootandy/dust/archive/refs/tags/v1.1.2.tar.gz

curl -o sources/ripgrep.tar.gz -L https://github.com/BurntSushi/ripgrep/archive/refs/tags/14.1.1.tar.gz

curl -o sources/skim.tar.gz -L https://github.com/skim-rs/skim/archive/refs/tags/v0.16.1.tar.gz

curl -o sources/hyperfine.tar.gz -L https://github.com/sharkdp/hyperfine/archive/refs/tags/v1.19.0.tar.gz

curl -o sources/tealdeer.tar.gz -L https://github.com/tealdeer-rs/tealdeer/archive/refs/tags/v1.7.1.tar.gz

curl -o sources/tokei.tar.gz -L https://github.com/XAMPPRocky/tokei/archive/refs/tags/v12.1.2.tar.gz

curl -o sources/jnv.tar.gz -L https://github.com/ynqa/jnv/archive/refs/tags/v0.5.0.tar.gz

curl -o sources/resvg.tar.gz -L https://github.com/linebender/resvg/archive/refs/tags/0.45.0.tar.gz

```

### Bioinformatics utilities

```bash
curl -o sources/nwr.tar.gz -L https://github.com/wang-q/nwr/archive/refs/tags/v0.7.7.tar.gz

curl -o sources/intspan.tar.gz -L https://github.com/wang-q/intspan/archive/refs/tags/v0.8.4.tar.gz

curl -o sources/hnsm.tar.gz -L https://github.com/wang-q/hnsm/archive/refs/tags/v0.3.1.tar.gz

curl -o sources/pgr.tar.gz -L https://github.com/wang-q/pgr/archive/refs/tags/v0.1.0.tar.gz

curl -o sources/anchr.tar.gz -L https://github.com/wang-q/anchr/archive/fadc09fe502e7b31cf6bbd9fa29b7188bf42ae3a.tar.gz

curl -o sources/wgatools.tar.gz -L https://github.com/wjwei-handsome/wgatools/archive/refs/tags/v1.0.0.tar.gz

```
