# Prebuilds from the official repositories

## Common tools

```bash
# Download and package binary for current system
bash scripts/prebuilds/jq.sh

# Download, package and test binary
bash scripts/prebuilds/jq.sh -t

# Download and package for specific OS
bash scripts/prebuilds/jq.sh linux     # Download Linux x86_64 binary
bash scripts/prebuilds/jq.sh macos     # Download macOS ARM64 binary
bash scripts/prebuilds/jq.sh windows   # Download Windows x86_64 binary

# Note: Tests will fail when downloading for different OS
# as binaries cannot be executed on incompatible architectures
bash scripts/prebuilds/jq.sh -t macos  # This will fail on Linux

bash scripts/prebuilds/ninja.sh -t linux
bash scripts/prebuilds/ninja.sh macos
bash scripts/prebuilds/ninja.sh windows

bash scripts/prebuilds/cmake.sh -t linux
bash scripts/prebuilds/cmake.sh macos
bash scripts/prebuilds/cmake.sh windows

bash scripts/prebuilds/tsv-utils.sh linux
bash scripts/prebuilds/tsv-utils.sh macos

bash scripts/prebuilds/pandoc.sh -t linux
bash scripts/prebuilds/pandoc.sh macos
bash scripts/prebuilds/pandoc.sh windows

bash scripts/prebuilds/tectonic.sh -t linux
bash scripts/prebuilds/tectonic.sh macos
bash scripts/prebuilds/tectonic.sh windows

#bash scripts/prebuilds/pup.sh

```

## Bioinformatics tools

```bash
bash scripts/prebuilds/blast.sh -t linux
bash scripts/prebuilds/blast.sh macos
bash scripts/prebuilds/blast.sh windows

bash scripts/prebuilds/sratoolkit.sh -t linux
bash scripts/prebuilds/sratoolkit.sh macos
bash scripts/prebuilds/sratoolkit.sh windows

bash scripts/prebuilds/muscle.sh -t linux
bash scripts/prebuilds/reseek.sh -t linux
bash scripts/prebuilds/usearch.sh -t linux

bash scripts/prebuilds/mosdepth.sh -t linux

bash scripts/prebuilds/iqtree2.sh -t linux
bash scripts/prebuilds/iqtree2.sh macos
bash scripts/prebuilds/iqtree2.sh windows

bash scripts/prebuilds/mash.sh -t linux
bash scripts/prebuilds/mash.sh macos

bash scripts/prebuilds/megahit.sh -t linux

bash scripts/prebuilds/mmseqs.sh -t linux
bash scripts/prebuilds/mmseqs.sh macos

bash scripts/prebuilds/bowtie2.sh -t linux
bash scripts/prebuilds/bowtie2.sh macos

bash scripts/prebuilds/raxml-ng.sh -t linux
bash scripts/prebuilds/raxml-ng.sh macos

bash scripts/prebuilds/freebayes.sh -t linux

# java
bash scripts/prebuilds/fastqc.sh linux
bash scripts/prebuilds/fastqc.sh macos

bash scripts/prebuilds/picard.sh linux
bash scripts/prebuilds/picard.sh macos

```
