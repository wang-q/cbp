# Prebuilds from the official repositories

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

bash scripts/ninja.sh -t linux
bash scripts/ninja.sh macos
bash scripts/ninja.sh windows

bash scripts/tsv-utils.sh linux
bash scripts/tsv-utils.sh macos

bash scripts/pandoc.sh -t linux
bash scripts/pandoc.sh macos
bash scripts/pandoc.sh windows

#bash scripts/pup.sh

```

## Bioinformatics tools

```bash
bash scripts/blast.sh -t linux
bash scripts/blast.sh macos
bash scripts/blast.sh windows

bash scripts/sratoolkit.sh -t linux
bash scripts/sratoolkit.sh macos
bash scripts/sratoolkit.sh windows

bash scripts/muscle.sh -t linux
bash scripts/reseek.sh -t linux
bash scripts/usearch.sh -t linux

bash scripts/mosdepth.sh -t linux

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
