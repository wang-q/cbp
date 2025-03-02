#!/bin/bash

# Source common build environment
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Set download URL
DL_URL="https://www.bioinformatics.babraham.ac.uk/projects/fastqc/fastqc_v0.12.1.zip"

# Download and extract
curl -L ${DL_URL} -o ${PROJ}.zip ||
    { echo "Error: Failed to download ${PROJ}"; exit 1; }
unzip ${PROJ}.zip

# Collect files
mkdir -p collect/share
cp -r FastQC/* collect/share/

# Create wrapper script
cat > collect/fastqc << 'EOF'
#!/bin/bash
SCRIPT_DIR=$(dirname $(readlink -f "$0"))
exec "${SCRIPT_DIR}/libexec/fastqc/fastqc" "$@"
EOF

chmod +x collect/libexec/fastqc/fastqc
chmod +x collect/fastqc

# Use build_tar function from common.sh
build_tar
