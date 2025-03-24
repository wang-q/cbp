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

bash scripts/prebuild.sh pup linux
bash scripts/prebuild.sh pup macos

bash scripts/prebuild.sh uv linux
bash scripts/prebuild.sh uv macos
bash scripts/prebuild.sh uv windows

bash scripts/prebuild.sh bat linux
bash scripts/prebuild.sh bat macos
bash scripts/prebuild.sh bat windows

```

## Bioinformatics tools

```bash
bash scripts/prebuild.sh blast linux
bash scripts/prebuild.sh blast macos
bash scripts/prebuild.sh blast windows

bash scripts/prebuilds/sratoolkit.sh linux
bash scripts/prebuilds/sratoolkit.sh macos
bash scripts/prebuilds/sratoolkit.sh windows

bash scripts/prebuild.sh muscle linux
bash scripts/prebuild.sh reseek linux
bash scripts/prebuild.sh usearch linux

bash scripts/prebuild.sh mosdepth linux

bash scripts/prebuilds/iqtree2.sh linux
bash scripts/prebuilds/iqtree2.sh macos
bash scripts/prebuilds/iqtree2.sh windows

bash scripts/prebuild.sh mash linux
bash scripts/prebuild.sh mash macos

bash scripts/prebuilds/megahit.sh -t linux

bash scripts/prebuild.sh mmseqs linux
bash scripts/prebuild.sh mmseqs macos

bash scripts/prebuild.sh bowtie2 linux
bash scripts/prebuild.sh bowtie2 macos

bash scripts/prebuild.sh stringtie linux
bash scripts/prebuild.sh stringtie macos

bash scripts/prebuilds/raxml-ng.sh linux
bash scripts/prebuilds/raxml-ng.sh macos

bash scripts/prebuild.sh freebayes linux

```

## java

```bash
bash scripts/prebuilds/fastqc.sh linux
bash scripts/prebuilds/fastqc.sh macos

bash scripts/prebuilds/picard.sh linux
bash scripts/prebuilds/picard.sh macos

```

## Fonts

```bash
bash scripts/prebuild.sh arial font
bash scripts/prebuild.sh charter font
bash scripts/prebuild.sh helvetica font

# Open Source Fonts
bash scripts/prebuild.sh fira font
bash scripts/prebuild.sh jetbrains-mono font
bash scripts/prebuild.sh firacode-nf font

bash scripts/prebuild.sh lxgw-wenkai font
bash scripts/prebuild.sh source-han-sans font
bash scripts/prebuild.sh source-han-serif font

```
