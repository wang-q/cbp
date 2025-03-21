# Prebuilds from the official repositories

## Common tools

```bash
# Download and package for specific OS
bash scripts/prebuilds/jq.sh linux     # Download Linux x86_64 binary
bash scripts/prebuilds/jq.sh macos     # Download macOS ARM64 binary
bash scripts/prebuilds/jq.sh windows   # Download Windows x86_64 binary

bash scripts/prebuilds/ninja.sh -t linux
bash scripts/prebuilds/ninja.sh macos
bash scripts/prebuilds/ninja.sh windows

bash scripts/prebuilds/cmake.sh -t linux
bash scripts/prebuilds/cmake.sh macos
bash scripts/prebuilds/cmake.sh windows

bash scripts/prebuilds/tsv-utils.sh linux
bash scripts/prebuilds/tsv-utils.sh macos

bash scripts/prebuilds/pandoc.sh linux
bash scripts/prebuilds/pandoc.sh macos
bash scripts/prebuilds/pandoc.sh windows

bash scripts/prebuilds/tectonic.sh linux
bash scripts/prebuilds/tectonic.sh macos
bash scripts/prebuilds/tectonic.sh windows

#bash scripts/prebuilds/pup.sh

bash scripts/prebuilds/uv.sh linux
bash scripts/prebuilds/uv.sh macos
bash scripts/prebuilds/uv.sh windows

bash scripts/prebuilds/bat.sh linux
bash scripts/prebuilds/bat.sh macos
bash scripts/prebuilds/bat.sh windows

```

## Bioinformatics tools

```bash
bash scripts/prebuilds/blast.sh linux
bash scripts/prebuilds/blast.sh macos
bash scripts/prebuilds/blast.sh windows

bash scripts/prebuilds/sratoolkit.sh -t linux
bash scripts/prebuilds/sratoolkit.sh macos
bash scripts/prebuilds/sratoolkit.sh windows

bash scripts/prebuilds/muscle.sh linux
bash scripts/prebuilds/reseek.sh linux
bash scripts/prebuilds/usearch.sh -t linux

bash scripts/prebuilds/mosdepth.sh linux

bash scripts/prebuilds/iqtree2.sh linux
bash scripts/prebuilds/iqtree2.sh macos
bash scripts/prebuilds/iqtree2.sh windows

bash scripts/prebuilds/mash.sh linux
bash scripts/prebuilds/mash.sh macos

bash scripts/prebuilds/megahit.sh -t linux

bash scripts/prebuilds/mmseqs.sh linux
bash scripts/prebuilds/mmseqs.sh macos

bash scripts/prebuilds/bowtie2.sh linux
bash scripts/prebuilds/bowtie2.sh macos

bash scripts/prebuilds/stringtie.sh linux
bash scripts/prebuilds/stringtie.sh macos

bash scripts/prebuilds/raxml-ng.sh linux
bash scripts/prebuilds/raxml-ng.sh macos

bash scripts/prebuilds/freebayes.sh linux

# java
bash scripts/prebuilds/fastqc.sh linux
bash scripts/prebuilds/fastqc.sh macos

bash scripts/prebuilds/picard.sh linux
bash scripts/prebuilds/picard.sh macos

```
