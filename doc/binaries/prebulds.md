# Prebuilds from the official repositories

## Common tools

```bash
bash scripts/prebuild.sh jq linux
bash scripts/prebuild.sh jq macos
bash scripts/prebuild.sh jq windows

bash scripts/prebuild.sh yq linux
bash scripts/prebuild.sh yq macos
bash scripts/prebuild.sh yq windows

bash scripts/prebuild.sh ninja linux
bash scripts/prebuild.sh ninja macos
bash scripts/prebuild.sh ninja windows

bash scripts/prebuilds/cmake.sh linux
bash scripts/prebuilds/cmake.sh macos
bash scripts/prebuilds/cmake.sh windows

bash scripts/prebuilds/tsv-utils.sh linux
bash scripts/prebuilds/tsv-utils.sh macos

bash scripts/prebuild.sh pandoc linux
bash scripts/prebuild.sh pandoc macos
bash scripts/prebuild.sh pandoc windows

bash scripts/prebuild.sh tectonic linux
bash scripts/prebuild.sh tectonic macos
bash scripts/prebuild.sh tectonic windows

bash scripts/prebuilds/pup.sh linux
bash scripts/prebuilds/pup.sh macos

bash scripts/prebuild.sh uv linux
bash scripts/prebuild.sh uv macos
bash scripts/prebuild.sh uv windows

bash scripts/prebuild.sh bat linux
bash scripts/prebuild.sh bat macos
bash scripts/prebuild.sh bat windows

```

## Bioinformatics tools

```bash
bash scripts/prebuilds/blast.sh linux
bash scripts/prebuilds/blast.sh macos
bash scripts/prebuilds/blast.sh windows

bash scripts/prebuilds/sratoolkit.sh linux
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
