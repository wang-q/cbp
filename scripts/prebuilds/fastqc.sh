#!/bin/bash

# Source common build environment
source "$(dirname "${BASH_SOURCE[0]}")/../common.sh"

# Set download URL
DL_URL="https://www.bioinformatics.babraham.ac.uk/projects/fastqc/fastqc_v0.12.1.zip"

# Download and extract
curl -L ${DL_URL} -o ${PROJ}.zip ||
    { echo "Error: Failed to download ${PROJ}"; exit 1; }
unzip ${PROJ}.zip

# Collect files
mkdir -p collect/libexec/fastqc/
cp -r FastQC/* collect/libexec/fastqc/

# Create wrapper script
mkdir -p collect/bin/
cat > collect/bin/fastqc << 'EOF'
#!/bin/bash

# Find the real path of the script
if [ -L "$0" ]; then
    SCRIPT_PATH=$(readlink "$0")
else
    SCRIPT_PATH="$0"
fi
SCRIPT_DIR=$(cd "$(dirname "$SCRIPT_PATH")" && pwd)
BASE_DIR=$(dirname "$SCRIPT_DIR")

# Set memory options if needed
JAVA_OPTS=${JAVA_OPTS:-"-Xmx2g"}

exec "${BASE_DIR}/libexec/fastqc/fastqc" "$@"
EOF

chmod +x collect/libexec/fastqc/fastqc
chmod +x collect/bin/fastqc

# Use build_tar function from common.sh
build_tar
